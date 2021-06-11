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

    fn use_result<F, V, E>(n: i32, f: F) -> (i32, Result<V, E>)
    where
        F: Fn() -> Result<V, E>,
    {
        let mut rt :i32 = 0;
        for _ in 0..n {
            rt = match f() {
                Ok(_) => 0,
                Err(_) => -1,
            };
        }

        (rt, f())
    }

    #[bench]
    fn bench_nzi32_100(b: &mut Bencher) {
        b.iter(|| use_result(100, return_nzi32));
    }

    #[bench]
    fn bench_nzi32_10000(b: &mut Bencher) {
        b.iter(|| use_result(10000, return_nzi32));
    }

    #[bench]
    fn bench_nzi32_1000000(b: &mut Bencher) {
        b.iter(|| use_result(1000000, return_nzi32));
    }

    #[bench]
    fn bench_nzi16_100(b: &mut Bencher) {
        b.iter(|| use_result(100, return_nzi16));        
    }

    #[bench]
    fn bench_nzi16_10000(b: &mut Bencher) {
        b.iter(|| use_result(10000, return_nzi16));        
    }

    #[bench]
    fn bench_nzi16_1000000(b: &mut Bencher) {
        b.iter(|| use_result(1000000, return_nzi16));        
    }

    #[bench]
    fn bench_i32_100(b: &mut Bencher) {
        b.iter(|| use_result(100, return_i32)); 
    }

    #[bench]
    fn bench_i32_10000(b: &mut Bencher) {
        b.iter(|| use_result(10000, return_i32)); 
    }

    #[bench]
    fn bench_i32_1000000(b: &mut Bencher) {
        b.iter(|| use_result(1000000, return_i32)); 
    }    

    #[bench]
    fn bench_i16_100(b: &mut Bencher) {
        b.iter(|| use_result(100, return_i16)); 
    }

    #[bench]
    fn bench_i16_10000(b: &mut Bencher) {
        b.iter(|| use_result(10000, return_i16)); 
    }

    #[bench]
    fn bench_i16_1000000(b: &mut Bencher) {
        b.iter(|| use_result(1000000, return_i16)); 
    }    
}

fn main() {}
