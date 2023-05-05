use crate::openxr::prelude::*;

use std::ffi::{c_char, CStr};

pub unsafe fn enumerate<I: Clone>(
    input_count: u32,
    output_count: &mut Option<u32>,
    items_ptr: *mut I,
    items: &[I],
) -> Result<(), XrResult> {
    // if output_count.is_none() {
    // 	return Err(XrResult::ERROR_VALIDATION_FAILURE);
    // }
    *output_count = Some(items.len() as u32);
    if input_count == 0 || items_ptr.is_null() {
        return Ok(());
    }
    if input_count < items.len() as u32 {
        return Err(XrResult::ERROR_SIZE_INSUFFICIENT);
    }
    if items_ptr.is_null() {
        return Ok(());
    }
    std::ptr::copy_nonoverlapping(items.as_ptr(), items_ptr, items.len());

    Ok(())
}

pub fn str_from_const_char<'a>(ptr: *const c_char) -> Result<&'a str, XrResult> {
    if ptr.is_null() {
        return Err(XrResult::ERROR_VALIDATION_FAILURE);
    }

    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map_err(|_| XrResult::ERROR_VALIDATION_FAILURE)
}

pub fn copy_str_to_buffer(string: &str, buf: &mut [c_char]) {
    bytemuck::cast_slice_mut(&mut buf[..string.len()]).copy_from_slice(string.as_bytes());
    buf[string.len()] = 0;
}