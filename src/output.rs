use std::marker::PhantomData;
use std::mem::size_of;
use std::num::Wrapping;
use std::ops::{Add, BitAnd, BitXor, Mul, Shl, Shr};
use {PcgOutput, PcgState, PcgResult};

#[derive(Clone, Debug)]
pub struct XshRs<Result, State>(PhantomData<(Result, State)>)
where Result: PcgResult<State>, State: PcgState;

impl<Result, State> Default for XshRs<Result, State>
where
    Result: PcgResult<State>,
    State: PcgState,
{
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
    fn output(internal: State) -> Result {
        let bits = size_of::<State>() * 8;
        let result_bits = size_of::<State>() * 8;
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
    fn output(internal: State) -> Result {
        // constexpr bitcount_t bits        = bitcount_t(sizeof(itype) * 8);
        let bits = size_of::<State>() * 8;
        // constexpr bitcount_t xtypebits   = bitcount_t(sizeof(xtype)*8);
        let result_bits = size_of::<Result>() * 8;
        // constexpr bitcount_t sparebits   = bits - xtypebits;
        let spare_bits = bits - result_bits;
        // constexpr bitcount_t wantedopbits =
        //                       xtypebits >= 128 ? 7
        //                     : xtypebits >=  64 ? 6
        //                     : xtypebits >=  32 ? 5
        //                     : xtypebits >=  16 ? 4
        //                     :                    3;
        let wanted_op_bits = match result_bits {
            0...15 => 3,
            16...31 => 4,
            32...63 => 5,
            64...127 => 6,
            _ => 7
        };
        // constexpr bitcount_t opbits =
        //                       sparebits >= wantedopbits ? wantedopbits
        //                                                 : sparebits;
        let op_bits = if spare_bits >= wanted_op_bits { wanted_op_bits } else { spare_bits };
        // constexpr bitcount_t amplifier = wantedopbits - opbits;
        let amplifier = wanted_op_bits - op_bits;
        // constexpr bitcount_t mask = (1 << opbits) - 1;
        let mask = (1 << op_bits) - 1;
        // constexpr bitcount_t topspare    = opbits;
        let top_spare = op_bits;
        // constexpr bitcount_t bottomspare = sparebits - topspare;
        let bottom_spare = spare_bits - top_spare;
        // constexpr bitcount_t xshift      = (topspare + xtypebits)/2;
        let x_shift = (top_spare + result_bits) / 2;
        // bitcount_t rot = opbits ? bitcount_t(internal >> (bits - opbits)) & mask
        //                         : 0;
        let rot = if op_bits != 0 {
            (internal.clone() >> (bits - op_bits)).into_usize() & mask
        } else {
            0
        };
        // bitcount_t amprot = (rot << amplifier) & mask;
        let amp_rot = (rot << amplifier) & mask;
        // internal ^= internal >> xshift;
        let internal = internal.clone() ^ (internal >> x_shift);
        // xtype result = xtype(internal >> bottomspare);
        let result = Result::from_state(internal >> bottom_spare);
        // result = rotr(result, amprot);
        let result = result.rotate_right(amp_rot as u32);
        // return result;
        result
    }
}
