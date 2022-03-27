#![allow(dead_code)]

use std::any::TypeId;

pub fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

pub fn get_type_of<T: 'static>(_: &T) -> TypeId {
    TypeId::of::<T>()
}
