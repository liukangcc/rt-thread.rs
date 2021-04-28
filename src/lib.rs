
//! # RT-Thread for Rust
//!
//!
//!
//!
//!
//!

// #![no_std]
#![allow(non_camel_case_types)]
mod clock;
mod components;
mod cpu;
mod device;
mod idle;
mod ipc;
mod irq;
mod kservice;
mod mem;
mod memheap;
mod object;
mod schedule;
mod signal;
mod slab;
mod thread;
mod timer;

pub mod include;

#[cfg(test)]
mod tests {

    use crate::include::rtdef::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test2() {
        use include::rtdef;
        let number:rt_base_t = 0;
        println!("number:{}", number);
    }
}
