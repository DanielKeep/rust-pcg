#![allow(non_camel_case_types)]

use engine::Engine;
use output::{RxsMXs, XshRr, XshRs};
use stream::{NoStream, SingleStream, SpecificStream, UniqueStream};
use {OutputPreviousIfSmall, PcgDefault};

pcg_setseq! { SetSeqXshRr_64_32: u32, u64, XshRr }
pcg_oneseq! { OneSeqXshRr_64_32: u32, u64, XshRr }
pcg_unique! { UniqueXshRr_64_32: u32, u64, XshRr }
pcg_mcg! { McgXshRs_64_32: u32, u64, XshRs }

pcg_setseq! { SetSeqRxsMXs_8_8: u8, u8, RxsMXs }
pcg_setseq! { SetSeqRxsMXs_16_16: u16, u16, RxsMXs }
pcg_setseq! { SetSeqRxsMXs_32_32: u32, u32, RxsMXs }
pcg_setseq! { SetSeqRxsMXs_64_64: u64, u64, RxsMXs }
