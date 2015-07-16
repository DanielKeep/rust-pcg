macro_rules! pcg_define_constants {
    ($name:ident: $type_:ty = $multiplier:expr, $increment:expr) => {
        impl PcgMultiplier<$type_> for $name {
            fn multiplier() -> $type_ { $multiplier }
        }
        impl PcgIncrement<$type_> for $name {
            fn increment() -> $type_ { $increment }
        }
    };
}

macro_rules! pcg_impl_result {
    ($type_:ident: $($from_tys:ty),+) => {
        $(
            impl PcgResult<$from_tys> for $type_ {
                fn from_state(value: $from_tys) -> Self {
                    value as $type_
                }

                fn rotate_right(self, n: u32) -> Self {
                    $type_::rotate_right(self, n)
                }
            }
        )*
    }
}

macro_rules! pcg_impl_state {
    ($type_:ident) => {
        impl PcgState for $type_ {
            type Wrapping = ::std::num::Wrapping<$type_>;

            fn bit_0() -> Self { 1 }
            fn bit_3() -> Self { 4 }
            fn into_usize(self) -> usize { self as usize }
            fn is_odd(&self) -> bool { (self & 1) == 1 }
            fn is_mcg_wrapped(&self) -> bool { (self & 3) != 0 }
            fn negate(self) -> Self { (::std::num::Wrapping(!self) + ::std::num::Wrapping(1)).0 }
            fn wrapped(self) -> Self { self | 3 }
            fn wrapping(self) -> Self::Wrapping { ::std::num::Wrapping(self) }
        }

        impl $crate::stream::UniqueStreamState for $type_ {
            fn from_usize(value: usize) -> Self {
                value as $type_
            }
        }
    };
}

macro_rules! pcg_setseq {
    ($name:ident: $rt:ty, $st:ty, $o:ident) => {
        pub type $name = Engine<$rt, $st, $o<$rt, $st>,
            OutputPreviousIfSmall<$st>, SpecificStream<$st>, PcgDefault>;
    };
}
