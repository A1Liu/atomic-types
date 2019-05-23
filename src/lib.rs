//! Simple crate that adds an `AtomicallySized` trait and an associated trait called
//! `Atomic<T>` that makes generic containers accross atomic types easier to implement.

#![no_std]

mod impls;
mod traits;

pub use impls::*;
pub use traits::*;
