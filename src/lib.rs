#![cfg_attr(not(feature = "std"), no_std)]

mod client;

pub mod v1 {
    pub use super::*;
}
