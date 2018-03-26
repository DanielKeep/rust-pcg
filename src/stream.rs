use std::cmp::min;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::Shl;
use {PcgDefault, PcgIncrement, PcgState, PcgSetStream, PcgStream};

#[derive(Clone, Debug)]
pub struct UniqueStream<State>(PhantomData<State>)
where State: PcgState + UniqueStreamState;

impl<State> UniqueStream<State>
where State: PcgState + UniqueStreamState {
    #[inline]
    pub fn stream(&self) -> State {
        self.increment() >> 1
    }
}

impl<State> Default for UniqueStream<State>
where State: PcgState + UniqueStreamState {
    #[inline]
    fn default() -> Self {
        UniqueStream(PhantomData)
    }
}

pub trait UniqueStreamState {
    #[inline]
    fn from_usize(value: usize) -> Self;
}

impl<State> PcgStream<State> for UniqueStream<State>
where State: PcgState + UniqueStreamState {
    #[inline] fn can_specify_stream() -> bool { false }
    #[inline] fn is_mcg() -> bool { false }
    #[inline] fn streams_pow2() -> usize { min(size_of::<State>(), size_of::<usize>())*8 - 1 }

    #[inline]
    fn increment(&self) -> State {
        State::from_usize((self as *const _ as usize) | 1)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde_derive", derive(Deserialize, Serialize))]
pub struct NoStream<State>(
    #[cfg_attr(feature = "serde_derive", serde(skip))] PhantomData<State>,
)
where State: PcgState;

impl<State> Default for NoStream<State>
where State: PcgState {
    #[inline]
    fn default() -> Self {
        NoStream(PhantomData)
    }
}

impl<State> PcgStream<State> for NoStream<State>
where State: PcgState {
    #[inline] fn can_specify_stream() -> bool { false }
    #[inline] fn is_mcg() -> bool { true }
    #[inline] fn streams_pow2() -> usize { 0 }

    #[inline]
    fn increment(&self) -> State {
        State::zero()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde_derive", derive(Deserialize, Serialize))]
pub struct SingleStream<State>(
    #[cfg_attr(feature = "serde_derive", serde(skip))] PhantomData<State>,
)
where State: PcgState;

impl<State> SingleStream<State>
where
    State: PcgState,
    PcgDefault: PcgIncrement<State>,
{
    #[inline]
    pub fn stream(&self) -> State {
        self.increment() >> 1
    }
}

impl<State> Default for SingleStream<State>
where State: PcgState {
    #[inline]
    fn default() -> Self {
        SingleStream(PhantomData)
    }
}

impl<State> PcgStream<State> for SingleStream<State>
where
    State: PcgState,
    PcgDefault: PcgIncrement<State>,
{
    #[inline] fn can_specify_stream() -> bool { false }
    #[inline] fn is_mcg() -> bool { false }
    #[inline] fn streams_pow2() -> usize { 0 }

    #[inline]
    fn increment(&self) -> State {
        PcgDefault::increment()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde_derive", derive(Deserialize, Serialize))]
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
    #[inline]
    pub fn stream(&self) -> State {
        self.inc.clone() >> 1
    }

    #[inline]
    pub fn set_stream(&mut self, specific_seq: State) {
        *self = SpecificStream::with_stream(specific_seq);
    }
}

impl<State> Default for SpecificStream<State>
where
    State: PcgState + Default + Shl<usize, Output=State>,
    PcgDefault: PcgIncrement<State>,
{
    #[inline]
    fn default() -> Self {
        SpecificStream {
            inc: State::default(),
        }
    }
}

impl<State> PcgSetStream<State> for SpecificStream<State>
where
    State: PcgState + Shl<usize, Output=State>,
    PcgDefault: PcgIncrement<State>,
{
    #[inline]
    fn with_stream(specific_seq: State) -> Self {
        SpecificStream {
            inc: (specific_seq << 1) | State::one(),
        }
    }
}

impl<State> PcgStream<State> for SpecificStream<State>
where
    State: PcgState + Shl<usize>,
    PcgDefault: PcgIncrement<State>,
{
    #[inline] fn can_specify_stream() -> bool { true }
    #[inline] fn is_mcg() -> bool { false }
    #[inline] fn streams_pow2() -> usize { size_of::<State>()*8 - 1 }

    #[inline]
    fn increment(&self) -> State {
        self.inc.clone()
    }
}
