use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem::size_of;
use std::num::Wrapping;
use num::{Bounded, One, Zero};
use {PcgGenerator, PcgMultiplier, PcgOutput, PcgPhase, PcgResult, PcgState, PcgStatefulStream, PcgStream};
use bounds::{DistanceToState, NextBoundedResult};

#[derive(Clone, Debug)]
pub struct Engine<Result, State, Output, Phase, Stream, Multiplier>
where
    Result: PcgResult<State>,
    State: PcgState,
    Output: PcgOutput<Result, State>,
    Phase: PcgPhase,
    Stream: PcgStream<State>,
    Multiplier: PcgMultiplier<State>,
{
    state: State,
    stream: Stream,
    _phantom: PhantomData<(Result, Output, Phase, Stream, Multiplier)>,
}

impl<Result, State, Output, Phase, Stream, Multiplier>
Engine<Result, State, Output, Phase, Stream, Multiplier>
where
    Result: PcgResult<State>,
    State: PcgState,
    Output: PcgOutput<Result, State>,
    Phase: PcgPhase,
    Stream: PcgStream<State>,
    Multiplier: PcgMultiplier<State>,
{
    pub fn new() -> Self
    where
        State: From<u64>,
        Stream: Default,
    {
        Self::with_state(0xcafe_f00d_d15e_a5e5.into())
    }

    pub fn with_state(state: State) -> Self
    where
        Stream: Default,
    {
        use bounds::WrappingState;
        let stream = Stream::default();
        Engine {
            state: {
                if Stream::is_mcg() {
                    state.wrapped()
                } else {
                    Self::bump(&stream, (state.wrapping() + stream.increment().wrapping()).into_state())
                }
            },
            stream: stream,
            _phantom: PhantomData,
        }
    }

    pub fn with_stream(state: State, stream_state: Stream::State) -> Self
    where
        Stream: PcgStatefulStream<State>,
    {
        use bounds::WrappingState;
        let stream = Stream::from_stream_state(stream_state);
        Engine {
            state: {
                if Stream::is_mcg() {
                    state.wrapped()
                } else {
                    Self::bump(&stream, (state.wrapping() + stream.increment().wrapping()).into_state())
                }
            },
            stream: stream,
            _phantom: PhantomData,
        }
    }

    // TODO: engine(itype, sm::stream_state)
    // TODO: engine(seedSeq)

    // TODO: seed (probably don't need it; it's just a re-assignment)

    pub fn dump_internals(&self) -> DumpEngineInternals<Result, State, Output, Phase, Stream, Multiplier>
    where State: Debug {
        DumpEngineInternals { ptr: self }
    }

    // pub fn max() -> Result
    // where Result: Bounded {
    //     Result::max_value()
    // }

    // pub fn min() -> Result
    // where Result: Bounded {
    //     Result::min_value()
    // }

    pub fn wrapped(&self) -> bool {
        if Stream::is_mcg() {
            // For MCGs, the low order two bits never change. In this
            // implementation, we keep them fixed at 3 to make this test
            // easier.
            self.state.is_mcg_wrapped()
        } else {
            self.state == State::zero()
        }
    }

    fn advance_impl(state: State, delta: State, cur_mult: State, cur_plus: State) -> State {
        use bounds::WrappingState;

        // The method used here is based on Brown, "Random Number Generation
        // with Arbitrary Stride,", Transactions of the American Nuclear
        // Society (Nov. 1994).  The algorithm is very similar to fast
        // exponentiation.
        //
        // Even though delta is an unsigned integer, we can pass a
        // signed integer to go backwards, it just goes "the long way round".
        let zero = State::zero();
        let one = State::one().wrapping();

        let mut delta = delta;
        let mut cur_mult = cur_mult.wrapping();
        let mut cur_plus = cur_plus.wrapping();
        let mut acc_mult = State::one().wrapping();
        let mut acc_plus = State::zero().wrapping();

        while delta > zero {
            if delta.is_odd() {
                acc_mult = acc_mult * cur_mult.clone();
                acc_plus = acc_plus * cur_mult.clone() + cur_plus.clone();
            }
            cur_plus = (cur_mult.clone() + one.clone()) * cur_plus;
            cur_mult = cur_mult.clone() * cur_mult;
            delta = delta >> 1;
        }
        (acc_mult * state.wrapping() + acc_plus).into_state()
    }

    fn distance(&self, new_state: State, mask: State) -> State {
        Self::distance_impl(self.state.clone(), new_state, Multiplier::multiplier(), self.stream.increment(), mask)
    }

    fn distance_impl(cur_state: State, new_state: State, cur_mult: State, cur_plus: State, mask: State) -> State {
        use bounds::WrappingState;

        let one = State::one().wrapping();
        let mut the_bit = (if Stream::is_mcg() { State::bit_3() } else { State::bit_0() }).wrapping();
        let mut distance = State::zero();

        let new_state = new_state.wrapping();
        let mut cur_state = cur_state.wrapping();
        let mut cur_mult = cur_mult.wrapping();
        let mut cur_plus = cur_plus.wrapping();
        let mask = mask.wrapping();

        while cur_state.clone() & mask.clone() != new_state.clone() & mask.clone() {
            if cur_state.clone() & the_bit.clone() != new_state.clone() & the_bit.clone() {
                cur_state = cur_state * cur_mult.clone() + cur_plus.clone();
                distance = distance | the_bit.clone().into_state();
            }
            debug_assert!(cur_state.clone() & the_bit.clone() == new_state.clone() & the_bit.clone());
            the_bit = the_bit << 1;
            cur_plus = (cur_mult.clone() + one.clone()) * cur_plus;
            cur_mult = cur_mult.clone() * cur_mult;
        }

        if Stream::is_mcg() { distance >> 2 } else { distance }
    }

    fn bump(stream: &Stream, state: State) -> State {
        use bounds::WrappingState;
        (state.wrapping() * Multiplier::multiplier().wrapping() + stream.increment().wrapping()).into_state()
    }

    fn base_generate(&mut self) -> State {
        self.state = Self::bump(&self.stream, self.state.clone());
        self.state.clone()
    }

    fn base_generate0(&mut self) -> State {
        let old_state = self.state.clone();
        self.state = Self::bump(&self.stream, self.state.clone());
        old_state
    }
}

impl<Result, State, Output, Phase, Stream, Multiplier>
PcgGenerator
for Engine<Result, State, Output, Phase, Stream, Multiplier>
where
    Result: PcgResult<State>,
    State: PcgState,
    Output: PcgOutput<Result, State>,
    Phase: PcgPhase,
    Stream: PcgStream<State>,
    Multiplier: PcgMultiplier<State>,
{
    type Result = Result;
    type State = State;
    type Output = Output;
    type Phase = Phase;
    type Stream = Stream;
    type Multiplier = Multiplier;

    fn advance(&mut self, delta: State) {
        self.state = Self::advance_impl(
            self.state.clone(),
            delta,
            Multiplier::multiplier(),
            self.stream.increment()
        );
    }

    fn backstep(&mut self, delta: State) {
        self.advance(delta.negate());
    }

    fn discard(&mut self, delta: State) {
        self.advance(delta);
    }

    fn distance_to(&self, other: &Self) -> Option<State>
    where State: DistanceToState {
        if self.stream.increment() != other.stream.increment() {
            return None;
        }

        let mask = !State::zero();
        Some(self.distance(other.state.clone(), mask))
    }

    fn next(&mut self) -> Result {
        let new = if Phase::output_previous() {
            self.base_generate0()
        } else {
            self.base_generate()
        };
        Output::output(new)
    }

    fn next_bounded(&mut self, upper_bound: Result) -> Result
    where Result: NextBoundedResult {
        use bounds::NextBoundedWrappingResult;

        let max = Result::max_value().wrapping();
        let min = Result::min_value().wrapping();
        let one = Result::one().wrapping();
        let ub = upper_bound.wrapping();

        let threshold = (max - min.clone() + one - ub.clone()).into_result() % ub.clone().into_result();

        loop {
            let r = self.next().wrapping() - min.clone();
            if r.clone().into_result() >= threshold {
                return r.into_result() % ub.into_result();
            }
        }
    }

    fn period_pow2() -> usize {
        size_of::<State>()*8 - if Stream::is_mcg() { 2 } else { 0 }
    }

    fn streams_pow2() -> usize {
        Stream::streams_pow2()
    }
}

// TODO: Engine: Eq, Sub, Input, Output

pub struct DumpEngineInternals<'a, Result, State, Output, Phase, Stream, Multiplier>
where
    Result: 'a + PcgResult<State>,
    State: 'a + PcgState + Debug,
    Output: 'a + PcgOutput<Result, State>,
    Phase: 'a + PcgPhase,
    Stream: 'a + PcgStream<State>,
    Multiplier: 'a + PcgMultiplier<State>,
    State::Wrapping: 'a,
{
    ptr: &'a Engine<Result, State, Output, Phase, Stream, Multiplier>,
}

impl<'a, Result, State, Output, Phase, Stream, Multiplier>
Debug
for DumpEngineInternals<'a, Result, State, Output, Phase, Stream, Multiplier>
where
    Result: PcgResult<State>,
    State: PcgState + Debug,
    Output: PcgOutput<Result, State>,
    Phase: PcgPhase,
    Stream: PcgStream<State>,
    Multiplier: PcgMultiplier<State>,
    State::Wrapping: 'a,
{
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        try!(write!(fmt, "{:?} {:?} {:?}",
            Multiplier::multiplier(),
            self.ptr.stream.increment(),
            self.ptr.state));
        Ok(())
    }
}
