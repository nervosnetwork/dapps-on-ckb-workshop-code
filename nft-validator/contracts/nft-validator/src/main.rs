#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    default_alloc, entry,
    error::SysError,
    high_level::{load_cell_lock_hash, load_script, QueryIter},
};

entry!(entry);
default_alloc!();

/// Program entry
fn entry() -> i8 {
    // Call main function and return error code
    match main() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}

/// Error
#[repr(i8)]
enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    // Add customized errors here...
    InvalidArgument,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
        }
    }
}

fn main() -> Result<(), Error> {
    // We will need to extract governance lock from current running script
    let script = load_script()?;
    let args: Bytes = script.args().unpack();
    if args.len() < 32 {
        return Err(Error::InvalidArgument);
    }
    let mut governance_lock_hash = [0u8; 32];
    governance_lock_hash.copy_from_slice(&args[0..32]);

    let mut input_lock_hashes = QueryIter::new(load_cell_lock_hash, Source::Input);
    let _governance_mode = input_lock_hashes.any(|lock_hash| lock_hash == governance_lock_hash);

    Ok(())
}
