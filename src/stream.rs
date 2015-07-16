use std::marker::PhantomData;
use std::cmp::min;
use std::mem::size_of;
use std::ops::Shl;
use num::{One, Zero};
use {PcgDefault, PcgIncrement, PcgState, PcgStatefulStream, PcgStream};

#[derive(Clone, Debug)]
pub struct UniqueStream<State>(PhantomData<State>)
where State: PcgState + UniqueStreamState;

impl<State> UniqueStream<State>
where State: PcgState + UniqueStreamState {
    pub fn stream(&self) -> State {
        self.increment() >> 1
    }
}

pub trait UniqueStreamState {
    fn from_usize(value: usize) -> Self;
}

impl<State> PcgStream<State> for UniqueStream<State>
where State: PcgState + UniqueStreamState {
    fn can_specify_stream() -> bool { false }
    fn is_mcg() -> bool { false }
    fn streams_pow2() -> usize { min(size_of::<State>(), size_of::<usize>())*8 - 1 }

    fn increment(&self) -> State {
        State::from_usize((self as *const _ as usize) | 1)
    }
}

#[derive(Clone, Debug)]
pub struct NoStream<State>(PhantomData<State>)
where State: PcgState + Zero;

impl<State> PcgStream<State> for NoStream<State>
where State: PcgState + Zero {
    fn can_specify_stream() -> bool { false }
    fn is_mcg() -> bool { true }
    fn streams_pow2() -> usize { 0 }

    fn increment(&self) -> State {
        State::zero()
    }
}

#[derive(Clone, Debug)]
pub struct SingleStream<State>(PhantomData<State>)
where State: PcgState;

impl<State> SingleStream<State>
where
    State: PcgState,
    PcgDefault: PcgIncrement<State>,
{
    pub fn stream(&self) -> State {
        self.increment() >> 1
    }
}

impl<State> PcgStream<State> for SingleStream<State>
where
    State: PcgState,
    PcgDefault: PcgIncrement<State>,
{
    fn can_specify_stream() -> bool { false }
    fn is_mcg() -> bool { false }
    fn streams_pow2() -> usize { 0 }

    fn increment(&self) -> State {
        PcgDefault::increment()
    }
}

#[derive(Clone, Debug)]
pub struct SpecificStream<State>
where
    State: PcgState + Shl<usize>,
    PcgDefault: PcgIncrement<State>,
{
    inc: State,
}

impl<State> SpecificStream<State>
where
    State: PcgState + Shl<usize, Output=State>,
    PcgDefault: PcgIncrement<State>,
{
    pub fn stream(&self) -> State {
        self.inc.clone() >> 1
    }

    pub fn set_stream(&mut self, specific_seq: State) {
        *self = SpecificStream::from_stream_state(specific_seq);
    }
}

impl<State> Default for SpecificStream<State>
where
    State: PcgState + Default + Shl<usize, Output=State>,
    PcgDefault: PcgIncrement<State>,
{
    fn default() -> Self {
        SpecificStream {
            inc: State::default(),
        }
    }
}

impl<State> PcgStatefulStream<State> for SpecificStream<State>
where
    State: PcgState + Shl<usize, Output=State>,
    PcgDefault: PcgIncrement<State>,
{
    type State = State;

    fn from_stream_state(state: State) -> Self {
        SpecificStream {
            inc: (state << 1) | State::one(),
        }
    }
}

impl<State> PcgStream<State> for SpecificStream<State>
where
    State: PcgState + Shl<usize>,
    PcgDefault: PcgIncrement<State>,
{
    fn can_specify_stream() -> bool { true }
    fn is_mcg() -> bool { false }
    fn streams_pow2() -> usize { size_of::<State>()*8 - 1 }

    fn increment(&self) -> State {
        self.inc.clone()
    }
}
