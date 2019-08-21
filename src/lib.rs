#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod account;
pub mod error;
pub mod hash;
pub mod process;
pub mod state;
pub mod transaction;
pub mod u264;

use crate::process::process_transactions;
use crate::state::{Backend, InMemoryBackend};
use crate::transaction::{Transaction, Transfer};
use alloc::vec::Vec;
use arrayref::array_ref;
use u264::U264;

#[cfg_attr(not(feature = "std"), no_std)]
static ALLOC: qimalloc::QIMalloc = qimalloc::QIMalloc::INIT;

#[cfg(feature = "scout")]
mod native {
    extern "C" {
        pub fn eth2_loadPreStateRoot(offset: *const u32);
        pub fn eth2_blockDataSize() -> u32;
        pub fn eth2_blockDataCopy(outputOfset: *const u32, offset: u32, length: u32);
        pub fn eth2_savePostStateRoot(offset: *const u32);
    }
}

#[cfg(feature = "scout")]
#[no_mangle]
pub extern "C" fn main() {
    // Get input size
    let input_size = unsafe { native::eth2_blockDataSize() as usize };

    // Copy input into buffer (too lazy to dynmically allocate atm)
    let mut input = [0u8; 42000];
    unsafe {
        native::eth2_blockDataCopy(input.as_mut_ptr() as *const u32, 0, input_size as u32);
    }

    let tx_count = u32::from_le_bytes(*array_ref!(input, 0, 4)) as usize;

    // size_of(transfer) == 176
    let transactions = unsafe {
        let mut ret = Vec::<Transaction>::new();

        for i in (0..4 + (tx_count * 176)).skip(4).step_by(176) {
            let mut buf = [0u8; 176];
            buf.copy_from_slice(&input[i..(i + 176)]);

            let tx = Transaction::Transfer(Transfer {
                to: (*array_ref![buf, 0, 32]).into(),
                from: (*array_ref![buf, 32, 32]).into(),
                nonce: u64::from_le_bytes(*array_ref![buf, 64, 8]),
                amount: u64::from_le_bytes(*array_ref![buf, 72, 8]),
                signature: *array_ref![buf, 80, 96],
            });

            ret.push(tx);
        }

        ret
    };

    let mut mem = InMemoryBackend::new(256);
    assert_eq!(mem.load(&input[4 + (tx_count * 176)..]), Ok(()));
    assert_eq!(process_transactions(&mut mem, &transactions), Ok(()));

    let roots = mem.roots().unwrap();

    // Verify pre-state root == calculated pre-state root
    let mut pre_state_root = [0u8; 32];
    unsafe { native::eth2_loadPreStateRoot(pre_state_root.as_mut_ptr() as *const u32) }
    assert_eq!(pre_state_root, roots.0);

    // Return post state
    unsafe { native::eth2_savePostStateRoot(roots.1.as_ptr() as *const u32) }
}
