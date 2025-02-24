/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! General utility functions
use crate as pg_sys;

/// Converts a `pg_sys::NameData` struct into a `&str`.  
///
/// This is a zero-copy operation and the returned `&str` is tied to the lifetime
/// of the provided `pg_sys::NameData`
#[inline]
pub fn name_data_to_str(name_data: &pg_sys::NameData) -> &str {
    unsafe { core::ffi::CStr::from_ptr(name_data.data.as_ptr()) }.to_str().unwrap()
}
