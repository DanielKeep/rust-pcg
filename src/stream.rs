use std::marker::PhantomData;
use std::cmp::min;
use std::mem::size_of;
use std::ops::Shl;
use num::{One, Zero};
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
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

impl<State> Default for UniqueStream<State>
where State: PcgState + UniqueStreamState {
    fn default() -> Self {
        UniqueStream(PhantomData)
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
where State: PcgState;

impl<State> Default for NoStream<State>
where State: PcgState {
    fn default() -> Self {
        NoStream(PhantomData)
    }
}

impl<State> PcgStream<State> for NoStream<State>
where State: PcgState {
    fn can_specify_stream() -> bool { false }
    fn is_mcg() -> bool { true }
    fn streams_pow2() -> usize { 0 }

    fn increment(&self) -> State {
        State::zero()
    }
}

impl<State> Decodable for NoStream<State>
where State: PcgState {
    fn decode<D>(d: &mut D) -> ::std::result::Result<Self, D::Error>
    where D: Decoder {
        try!(d.read_nil());
        Ok(NoStream::default())
    }
}

impl<State> Encodable for NoStream<State>
where State: PcgState {
    fn encode<E>(&self, e: &mut E) -> ::std::result::Result<(), E::Error>
    where E: Encoder {
        e.emit_nil()
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

impl<State> Default for SingleStream<State>
where State: PcgState {
    fn default() -> Self {
        SingleStream(PhantomData)
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

impl<State> Decodable for SingleStream<State>
where State: PcgState {
    fn decode<D>(d: &mut D) -> ::std::result::Result<Self, D::Error>
    where D: Decoder {
        try!(d.read_nil());
        Ok(SingleStream::default())
    }
}

impl<State> Encodable for SingleStream<State>
where State: PcgState {
    fn encode<E>(&self, e: &mut E) -> ::std::result::Result<(), E::Error>
    where E: Encoder {
        e.emit_nil()
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

impl<State> Decodable for SpecificStream<State>
where
    State: PcgState + Decodable + Shl<usize>,
    PcgDefault: PcgIncrement<State>,
{
    fn decode<D>(d: &mut D) -> ::std::result::Result<Self, D::Error>
    where D: Decoder {
        Ok(SpecificStream {
            inc: try!(State::decode(d)),
        })
    }
}

impl<State> Encodable for SpecificStream<State>
where
    State: PcgState + Encodable + Shl<usize>,
    PcgDefault: PcgIncrement<State>,
{
    fn encode<E>(&self, e: &mut E) -> ::std::result::Result<(), E::Error>
    where E: Encoder {
        self.inc.encode(e)
    }
}
