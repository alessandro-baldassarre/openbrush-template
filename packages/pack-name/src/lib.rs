#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

mod data;
pub mod error;
pub mod impls;
pub mod traits;

pub use crate::data::Data;
