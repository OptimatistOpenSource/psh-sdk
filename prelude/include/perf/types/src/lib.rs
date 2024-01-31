#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use core::ptr;
use rkyv::ser::serializers::AllocSerializer;
use rkyv::{Archive, Deserialize, Serialize};

pub mod config;
pub mod counting;
pub mod event;

/// Serialize bytes
pub fn ser<T>(val: &T) -> Box<[u8]>
where
    T: Serialize<AllocSerializer<256>>,
{
    rkyv::to_bytes::<T, 256>(val).unwrap().into_boxed_slice()
}

/// Deserialize bytes
/// # Safety
/// [`bytes`] must be valid for [`rkyv::archived_root`]
pub unsafe fn de<T>(bytes: &[u8]) -> T
where
    T: Archive,
    T::Archived: Deserialize<T, rkyv::Infallible>,
{
    unsafe { rkyv::archived_root::<T>(bytes) }
        .deserialize(&mut rkyv::Infallible)
        .unwrap()
}

/// Deserialize the raw slice parts
/// # Safety
/// [`sered_ptr`] must be a valid ptr to a slice with [`sered_len`] for [`rkyv::archived_root`]
pub unsafe fn raw_parts_de<T>(sered_ptr: *mut u8, sered_len: usize) -> T
where
    T: Archive,
    T::Archived: Deserialize<T, rkyv::Infallible>,
{
    let ptr = ptr::slice_from_raw_parts_mut::<u8>(sered_ptr, sered_len as _);
    de::<T>(&*ptr)
}

/// Claim the ownership of the raw slice parts and deserialize it
/// # Safety
/// [`sered_ptr`] must be a valid ptr to a slice with [`sered_len`] for [`rkyv::archived_root`]
pub unsafe fn claim_raw_parts_de<T>(sered_ptr: *mut u8, sered_len: usize) -> T
where
    T: Archive,
    T::Archived: Deserialize<T, rkyv::Infallible>,
{
    let ptr = ptr::slice_from_raw_parts_mut::<u8>(sered_ptr, sered_len as _);
    let sered = unsafe { Box::from_raw(ptr) };
    de::<T>(&sered)
}
