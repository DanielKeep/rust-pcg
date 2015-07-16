#![allow(non_camel_case_types)]

use engine::Engine;
use output::{XshRr, XshRs};
use stream::{NoStream, SingleStream, SpecificStream, UniqueStream};
use {OutputPreviousIfSmall, PcgDefault};

pcg_setseq! { SetSeqXshRr_64_32: u32, u64, XshRr }
pcg_oneseq! { OneSeqXshRr_64_32: u32, u64, XshRr }
pcg_unique! { UniqueXshRr_64_32: u32, u64, XshRr }
pcg_mcg! { McgXshRs_64_32: u32, u64, XshRs }
