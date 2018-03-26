use std::num::Wrapping;
use std::ops::{Add, BitAnd, Mul, Not, Rem, Shl, Shr, Sub};
use num_traits::{Bounded, One};

pub trait DistanceToState: Not<Output=Self> + Sized {}

impl<State> DistanceToState for State
where State: Not<Output=State> {}

pub trait NextBoundedResult: Sized + Clone + Bounded + Ord + One + Rem<Output=Self> {
    type Wrapping: NextBoundedWrappingResult<Result=Self>;
    fn wrapping(self) -> Self::Wrapping;
}

impl<Result> NextBoundedResult for Result
where
    Result: Clone + Bounded + Ord + One + Rem<Output=Result>,
    Wrapping<Result>: NextBoundedWrappingResult<Result=Result>,
{
    type Wrapping = Wrapping<Result>;
    fn wrapping(self) -> Self::Wrapping { Wrapping(self) }
}

pub trait NextBoundedWrappingResult: Sized + Clone + Add<Output=Self> + Sub<Output=Self> {
    type Result;
    fn into_result(self) -> Self::Result;
}

impl<Result> NextBoundedWrappingResult for Wrapping<Result>
where Wrapping<Result>: Clone + Add<Output=Wrapping<Result>> + Sub<Output=Wrapping<Result>> {
    type Result = Result;
    fn into_result(self) -> Self::Result { self.0 }
}

pub trait WrappingState:
    Clone
    + Eq
    + Add<Output=Self>
    + BitAnd<Output=Self>
    + Mul<Output=Self>
    + Shl<usize, Output=Self>
    + Shr<usize, Output=Self>
{
    type State;
    fn into_state(self) -> Self::State;
}

impl<State> WrappingState for Wrapping<State>
where
    Wrapping<State>:
        Clone
        + Eq
        + Add<Output=Wrapping<State>>
        + BitAnd<Output=Wrapping<State>>
        + Mul<Output=Wrapping<State>>
        + Shl<usize, Output=Wrapping<State>>
        + Shr<usize, Output=Wrapping<State>>,
{
    type State = State;
    fn into_state(self) -> Self::State { self.0 }
}
