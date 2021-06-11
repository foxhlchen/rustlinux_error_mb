#![feature(test)]

use core::num::{NonZeroI16, NonZeroI32};
extern crate test;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ErrorNzi32(NonZeroI32);
pub struct ErrorNzi16(NonZeroI16);
pub struct ErrorI16(i16);
pub struct ErrorI32(i32);

#[cfg(test)]
mod bench {
    use super::*;
    use test::Bencher;

    fn return_nzi32() -> Result<(), ErrorNzi32> {
        unsafe {
            static mut I: i32 = 0;
            I += 1;


            if I % 2 == 0 {
                return Ok(());
            }

            Err(ErrorNzi32( NonZeroI32::new_unchecked(I)))
        }
    }

    fn return_nzi16() -> Result<(), ErrorNzi16> {
        unsafe {
            static mut I: i16 = 0;
            I += 1;

            if I % 2 == 0 {
                return Ok(());
            }

            Err(ErrorNzi16(NonZeroI16::new_unchecked(I)))
        }
    }

    fn return_i32() -> Result<(), i32> {
        unsafe {
            static mut I: i32 = 0;
            I += 1;

            if I % 2 == 0 {
                return Ok(());
            }

            Err(I)
        }
    }

    fn return_i16() -> Result<(), i16> {
        unsafe {
            static mut I: i16 = 0;
            I += 1;

            if I % 2 == 0 {
                return Ok(());
            }

            Err(I)
        }
    }

    fn use_result<F, V, E>(f: F) -> Result<V, E>
    where
        F: Fn() -> Result<V, E>,
    {
        for _ in 0..500000000 {
            match f() {
                Ok(_) => 0,
                Err(_) => -1,
            };
        }

        f()
    }

    #[bench]
    fn bench_nzi32(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..50000000 {
                match use_result(return_nzi32) {
                    Ok(_) => 0,
                    Err(_) => -1,
                };
            }
        });
    }

    #[bench]
    fn bench_nzi16(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..50000000 {
                match use_result(return_nzi16) {
                    Ok(_) => 0,
                    Err(_) => -1,
                };
            }
        });        
    }

    #[bench]
    fn bench_i32(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..50000000 {
                match use_result(return_i32) {
                    Ok(_) => 0,
                    Err(_) => -1,
                };
            }
        }); 
    }

    #[bench]
    fn bench_i16(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..50000000 {
                match use_result(return_i16) {
                    Ok(_) => 0,
                    Err(_) => -1,
                };
            }
        }); 
    }
}

fn main() {}
