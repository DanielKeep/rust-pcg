#![allow(non_camel_case_types)]

use engine::Engine;
use output::XshRr;
use stream::SpecificStream;
use {OutputPreviousIfSmall, PcgDefault};

pcg_setseq! { SetSeqXshRr_64_32: u32, u64, XshRr }
