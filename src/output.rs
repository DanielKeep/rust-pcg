use std::marker::PhantomData;
use std::mem::size_of;
use std::num::Wrapping;
use std::ops::{Add, BitAnd, BitXor, Mul, Shl, Shr};
use {PcgOutput, PcgState, PcgResult, McgMultiplier};

#[derive(Clone, Debug)]
pub struct XshRs<Result, State>(PhantomData<(Result, State)>)
where Result: PcgResult<State>, State: PcgState;

impl<Result, State> Default for XshRs<Result, State>
where
    Result: PcgResult<State>,
    State: PcgState,
{
    #[inline]
    fn default() -> Self {
        XshRs(PhantomData)
    }
}

impl<Result, State> PcgOutput<Result, State> for XshRs<Result, State>
where
    Result: PcgResult<State>,
    State: PcgState + BitAnd<Output=State> + BitXor<Output=State> + Shr<usize, Output=State>,
    Wrapping<State>: Add<Output=Wrapping<State>>
        + BitAnd<Output=Wrapping<State>>
        + Mul<Output=Wrapping<State>>
        + Shl<usize, Output=Wrapping<State>>
        + Shr<usize, Output=Wrapping<State>>,
{
    #[inline]
    fn output(internal: State) -> Result {
        let bits = size_of::<State>() * 8;
        let result_bits = size_of::<Result>() * 8;
        let spare_bits = bits - result_bits;
        let op_bits = {
            if spare_bits >= 64 + 5 { 5 }
            else if spare_bits >= 32 + 4 { 4 }
            else if spare_bits >= 16 + 3 { 3 }
            else if spare_bits >= 4 + 2 { 2 }
            else if spare_bits >= 1 + 1 { 1 }
            else { 0 }
        };
        let mask = (1 << op_bits) - 1;
        let max_rand_shift = mask;
        let top_spare = op_bits;
        let bottom_spare = spare_bits - top_spare;
        let x_shift = top_spare + (result_bits + max_rand_shift) / 2;
        let r_shift = if op_bits != 0 {
            (internal.clone() >> (bits - op_bits)).into_usize() & mask
        } else {
            0
        };
        let internal = internal.clone() ^ (internal >> x_shift);
        Result::from_state(internal >> (bottom_spare - max_rand_shift + r_shift))
    }
}

#[derive(Clone, Debug)]
pub struct XshRr<Result, State>(PhantomData<(Result, State)>)
where Result: PcgResult<State>, State: PcgState;

impl<Result, State> Default for XshRr<Result, State>
where
    Result: PcgResult<State>,
    State: PcgState,
{
    #[inline]
    fn default() -> Self {
        XshRr(PhantomData)
    }
}

impl<Result, State> PcgOutput<Result, State> for XshRr<Result, State>
where
    Result: PcgResult<State>,
    State: PcgState + BitAnd<Output=State> + BitXor<Output=State> + Shr<usize, Output=State>,
    Result: ::std::fmt::Debug, State: ::std::fmt::Debug,
    Wrapping<State>: Add<Output=Wrapping<State>>
        + BitAnd<Output=Wrapping<State>>
        + Mul<Output=Wrapping<State>>
        + Shl<usize, Output=Wrapping<State>>
        + Shr<usize, Output=Wrapping<State>>,
{
    #[inline]
    fn output(internal: State) -> Result {
        let bits = size_of::<State>() * 8;
        let result_bits = size_of::<Result>() * 8;
        let spare_bits = bits - result_bits;
        let wanted_op_bits = match result_bits {
            0...15 => 3,
            16...31 => 4,
            32...63 => 5,
            64...127 => 6,
            _ => 7
        };
        let op_bits = if spare_bits >= wanted_op_bits { wanted_op_bits } else { spare_bits };
        let amplifier = wanted_op_bits - op_bits;
        let mask = (1 << op_bits) - 1;
        let top_spare = op_bits;
        let bottom_spare = spare_bits - top_spare;
        let x_shift = (top_spare + result_bits) / 2;
        let rot = if op_bits != 0 {
            (internal.clone() >> (bits - op_bits)).into_usize() & mask
        } else {
            0
        };
        let amp_rot = (rot << amplifier) & mask;
        let internal = internal.clone() ^ (internal >> x_shift);
        let result = Result::from_state(internal >> bottom_spare);
        let result = result.rotate_right(amp_rot as u32);
        result
    }
}

#[derive(Clone, Debug)]
pub struct RxsMXs<Result, State>(PhantomData<(Result, State)>)
where Result: PcgResult<State>, State: PcgState;

impl<Result, State> Default for RxsMXs<Result, State>
where
    Result: PcgResult<State>,
    State: PcgState,
{
    #[inline]
    fn default() -> Self {
        RxsMXs(PhantomData)
    }
}

impl<Result, State> PcgOutput<Result, State> for RxsMXs<Result, State>
where
    Result: PcgResult<State>,
    State: PcgState + McgMultiplier + BitXor<Output=State>,
{
    #[inline]
    fn output(internal: State) -> Result {
        use bounds::WrappingState;

        let result_bits = size_of::<Result>() * 8;
        let bits = size_of::<State>() * 8;
        let op_bits = match result_bits {
            0...15 => 2,
            16...31 => 3,
            32...63 => 4,
            64...127 => 5,
            _ => 6
        };
        let shift = bits - result_bits;
        let mask = (1 << op_bits) - 1;
        let r_shift = if op_bits != 0 {
            (internal.clone() >> (bits - op_bits)).into_usize() & mask
        } else {
            0
        };
        let internal = internal.clone() ^ (internal >> (op_bits + r_shift));
        let internal = (internal.wrapping() * <State as McgMultiplier>::multiplier().wrapping()).into_state();
        let result = internal >> shift;
        let result = result.clone() ^ (result >> ((2 * result_bits + 2) / 3));
        Result::from_state(result)
    }
}
