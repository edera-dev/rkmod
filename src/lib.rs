//! rkmod: an embeddable alternative to libkmod
//!
//! rkmod implements the functionality of libkmod in a reusable library
//! with no dependencies on C bindings. the key goal is to successfully
//! load a kernel module and it's dependencies with pure Rust, handling
//! signed modules properly with validation.
pub mod compression;

pub mod error;
pub mod util;

pub mod cache;
#[cfg(all(target_os = "linux", feature = "controller"))]
pub mod controller;
#[cfg(feature = "database")]
pub mod database;
#[cfg(all(target_os = "linux", feature = "module-manager"))]
pub mod manager;
#[cfg(feature = "file-mmap")]
pub mod mmap;
pub mod object;
#[cfg(feature = "signature-core")]
pub mod signature;
