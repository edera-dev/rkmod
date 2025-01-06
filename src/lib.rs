//! rkmod: an embeddable alternative to libkmod
//!
//! rkmod implements the functionality of libkmod in a reusable library
//! with no dependencies on C bindings. the key goal is to successfully
//! load a kernel module and it's dependencies with pure Rust, handling
//! signed modules properly with validation.
pub mod compression;

/// content: provide raw file content for kmods
///
/// One interesting challenge of interacting with kernel modules is
/// that they can be compressed or just plain ELF files. The APIs
/// in this Rust module provide access to raw file contents using
/// the [bytes] crate, which provides cheaply cloneable access to
/// bytes.
pub mod elf;
pub mod error;
pub mod util;

pub mod cache;
pub mod controller;
pub mod database;
pub mod directory;
#[cfg(feature = "module-manager")]
pub mod manager;
#[cfg(feature = "file-mmap")]
pub mod mmap;
pub mod object;
pub mod symbol;
