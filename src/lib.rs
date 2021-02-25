#![no_std]
#![feature(min_const_generics)]
#![feature(assoc_char_funcs)]
#![feature(associated_type_bounds)]
#![feature(type_alias_impl_trait)]

#[macro_use]
extern crate lazy_static;

pub mod drivers;
pub mod osattrs;
pub mod proc;
pub mod runtime;
