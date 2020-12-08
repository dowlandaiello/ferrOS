#![no_std]
#![feature(min_const_generics)]
#![feature(assoc_char_funcs)]
#![feature(associated_type_bounds)]
#![feature(type_alias_impl_trait)]
#![feature(const_raw_ptr_deref)]

#[macro_use]
extern crate lazy_static;

pub mod drivers;
pub mod osattrs;
