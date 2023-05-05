use std::hash::BuildHasherDefault;

use dashmap::mapref::one::{RefMut, Ref};
use rustc_hash::FxHasher;

pub trait Handle: Sized {
    type HandleType;

    fn raw(&self) -> u64;

    fn get_mut<'a>(self) -> Result<RefMut<'a, u64, Self::HandleType, BuildHasherDefault<FxHasher>>, openxr_sys::Result>;

    fn get<'a>(self) -> Result<Ref<'a, u64, Self::HandleType, BuildHasherDefault<FxHasher>>, openxr_sys::Result>;

    fn destroy(self) -> Result<(), openxr_sys::Result>;

    fn is_null(self) -> bool;
}
