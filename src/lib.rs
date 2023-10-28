#![no_std]
#![cfg_attr(feature = "_internal_use_allocator_api", feature(allocator_api))]
#![allow(incomplete_features)]
#![cfg_attr(not(feature = "nightly_lazy_type_alias"), allow(type_alias_bounds))]
#![cfg_attr(feature = "nightly_lazy_type_alias", feature(lazy_type_alias))]
#![cfg_attr(feature = "nightly_strict_provenance", feature(strict_provenance))]

extern crate alloc;

use alloc::vec::Vec; //TODO calloc
use alloc::vec::Vec as StdVec;
use calloc::{Allocator, Global};
use core::{mem, ops::Deref};
//use cross;
use store::lifos::FixedDequeLifos;

mod calloc;
mod idx;
mod store;
