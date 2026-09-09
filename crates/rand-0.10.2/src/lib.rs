// Copyright 2018 Developers of the Rand project.
// Copyright 2013-2017 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Random-number generators and samplers
#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico"
)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]
#![no_std]
#![cfg_attr(feature = "simd_support", feature(portable_simd))]
#![cfg_attr(
    all(feature = "simd_support", target_feature = "avx512bw"),
    feature(stdarch_x86_avx512)
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::float_cmp,
    clippy::neg_cmp_op_on_partial_ord,
    clippy::nonminimal_bool
)]
#![deny(clippy::undocumented_unsafe_blocks)]

extern crate alloc;
extern crate std;

// Re-export rand_core itself
pub use rand_core;

// Re-exports from rand_core
pub use rand_core::{CryptoRng, Rng, SeedableRng, TryCryptoRng, TryRng};

///
pub mod error
{
    pub use std::error::{ * };
}
///
pub mod fmt
{
    pub use std::fmt::{ * };
}
///
pub mod io
{
    pub use std::io::{ * };
}
///
pub mod mem
{
    pub use std::mem::{ * };
}
///
pub mod num
{
    pub use std::num::{ * };
}
///
pub mod ptr
{
    pub use std::ptr::{ * };
}
///
pub mod slice
{
    pub use std::slice::{ * };

    /// Polyfill for `maybe_uninit_slice` feature's `MaybeUninit::slice_assume_init_mut`.
    #[inline(always)] pub unsafe fn assume_init_mut<T>(slice: &mut [crate::mem::MaybeUninit<T>]) -> &mut [T]
    {
        let ptr = crate::ptr::from_mut(slice) as *mut [T];
        // SAFETY: `MaybeUninit<T>` is guaranteed to be layout-compatible with `T`.
        unsafe { &mut *ptr }
    }
}

pub mod distr;
pub mod prelude;
mod rng;
pub mod rngs;
pub mod seq;

// Public exports
#[cfg(feature = "thread_rng")]
pub use crate::rngs::thread::rng;

pub use rng::{Fill, RngExt};

#[cfg(feature = "thread_rng")]
use crate::distr::{Distribution, StandardUniform};

/// Construct and seed an RNG
///
/// This method yields a seeded RNG, using [`rng`] ([`ThreadRng`]) if enabled or
/// [`SysRng`] otherwise.
///
/// # Examples
///
/// ```
/// let mut rng: rand::rngs::SmallRng = rand::make_rng();
/// # let _ = rand::Rng::next_u32(&mut rng);
/// ```
///
/// # Panics
///
/// If [`SysRng`] fails to obtain entropy from the OS. This is unlikely
/// outside of early boot or unusual system conditions.
///
/// # Security
///
/// Refer to [`ThreadRng#Security`].
///
/// [`SysRng`]: crate::rngs::SysRng
/// [`ThreadRng`]: crate::rngs::ThreadRng
/// [`ThreadRng#Security`]: crate::rngs::ThreadRng#security
#[cfg(feature = "sys_rng")]
#[track_caller]
pub fn make_rng<R: SeedableRng>() -> R {
    #[cfg(feature = "thread_rng")]
    {
        R::from_rng(&mut rng())
    }

    #[cfg(not(feature = "thread_rng"))]
    {
        R::try_from_rng(&mut rngs::SysRng).expect("unexpected failure from SysRng")
    }
}

/// Adapter to support [`std::io::Read`] over a [`TryRng`]
///
/// # Examples
///
/// ```no_run
/// use std::{io, io::Read};
/// use std::fs::File;
/// use rand::{rngs::SysRng, RngReader};
///
/// io::copy(
///     &mut RngReader(SysRng).take(100),
///     &mut File::create("/tmp/random.bytes").unwrap()
/// ).unwrap();
/// ```
#[cfg(feature = "std")]
pub struct RngReader<R: TryRng>(pub R);

#[cfg(feature = "std")]
impl<R: TryRng> std::io::Read for RngReader<R> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.0
            .try_fill_bytes(buf)
            .map_err(|err| std::io::Error::other(std::format!("RNG error: {err}")))?;
        Ok(buf.len())
    }
}

#[cfg(feature = "std")]
impl<R: TryRng> std::fmt::Debug for RngReader<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RngReader").finish()
    }
}

/// Generate a random value using the thread-local random number generator.
///
/// This function is shorthand for <code>[rng()].[random()](RngExt::random)</code>:
///
/// -   See [`ThreadRng`] for documentation of the generator and security
/// -   See [`StandardUniform`] for documentation of supported types and distributions
///
/// # Examples
///
/// ```
/// let x = rand::random::<u8>();
/// println!("{}", x);
///
/// let y = rand::random::<f64>();
/// println!("{}", y);
///
/// if rand::random() { // generates a boolean
///     println!("Better lucky than good!");
/// }
/// ```
///
/// If you're calling `random()` repeatedly, consider using a local `rng`
/// handle to save an initialization-check on each usage:
///
/// ```
/// use rand::RngExt; // provides the `random` method
///
/// let mut rng = rand::rng(); // a local handle to the generator
///
/// let mut v = vec![1, 2, 3];
///
/// for x in v.iter_mut() {
///     *x = rng.random();
/// }
/// ```
///
/// [`StandardUniform`]: distr::StandardUniform
/// [`ThreadRng`]: rngs::ThreadRng
#[cfg(feature = "thread_rng")]
#[inline]
pub fn random<T>() -> T
where
    StandardUniform: Distribution<T>,
{
    rng().random()
}

/// Return an iterator over [`random()`] variates
///
/// This function is shorthand for
/// <code>[rng()].[random_iter](RngExt::random_iter)()</code>.
///
/// # Example
///
/// ```
/// let v: Vec<i32> = rand::random_iter().take(5).collect();
/// println!("{v:?}");
/// ```
#[cfg(feature = "thread_rng")]
#[inline]
pub fn random_iter<T>() -> distr::Iter<StandardUniform, rngs::ThreadRng, T>
where
    StandardUniform: Distribution<T>,
{
    rng().random_iter()
}

/// Generate a random value in the given range using the thread-local random number generator.
///
/// This function is shorthand for
/// <code>[rng()].[random_range](RngExt::random_range)(<var>range</var>)</code>.
///
/// # Example
///
/// ```
/// let y: f32 = rand::random_range(0.0..=1e9);
/// println!("{}", y);
///
/// let words: Vec<&str> = "Mary had a little lamb".split(' ').collect();
/// println!("{}", words[rand::random_range(..words.len())]);
/// ```
/// Note that the second example can also be achieved (without `collect`'ing
/// to a `Vec`) using [`seq::IteratorRandom::choose`].
#[cfg(feature = "thread_rng")]
#[inline]
pub fn random_range<T, R>(range: R) -> T
where
    T: distr::uniform::SampleUniform,
    R: distr::uniform::SampleRange<T>,
{
    rng().random_range(range)
}

/// Return a bool with a probability `p` of being true.
///
/// This function is shorthand for
/// <code>[rng()].[random_bool](RngExt::random_bool)(<var>p</var>)</code>.
///
/// # Example
///
/// ```
/// println!("{}", rand::random_bool(1.0 / 3.0));
/// ```
///
/// # Panics
///
/// If `p < 0` or `p > 1`.
#[cfg(feature = "thread_rng")]
#[inline]
#[track_caller]
pub fn random_bool(p: f64) -> bool {
    rng().random_bool(p)
}

/// Return a bool with a probability of `numerator/denominator` of being true.
#[cfg(feature = "thread_rng")]
#[inline]
#[track_caller]
pub fn random_ratio(numerator: u32, denominator: u32) -> bool {
    rng().random_ratio(numerator, denominator)
}

/// Fill any type implementing [`Fill`] with random data.
#[cfg(feature = "thread_rng")]
#[inline]
#[track_caller]
pub fn fill<T: Fill>(dest: &mut [T]) {
    Fill::fill_slice(dest, &mut rng())
}

type BOOL = core::ffi::c_int;
const TRUE: BOOL = 1;

unsafe extern "system"
{
    fn ProcessPrng(pbdata: *mut u8, cbdata: usize) -> BOOL;
}

use crate::rngs::SysError;
///
#[inline] pub fn fill_inner(dest: &mut [crate::mem::MaybeUninit<u8>]) -> Result<(), SysError>
{
    let result = unsafe { ProcessPrng(dest.as_mut_ptr().cast::<u8>(), dest.len()) };
    if result == TRUE { Ok(()) } else { Err(SysError::UNEXPECTED) }
}
/// Fill potentially uninitialized buffer `dest` with random bytes from the system's preferred random number source and return a mutable reference to those bytes.
#[inline] pub fn fill_uninit(dest: &mut [crate::mem::MaybeUninit<u8>]) -> Result<&mut [u8], SysError> {
    if !dest.is_empty()
    {
        crate::fill_inner(dest)?;
    }

    Ok(unsafe { crate::slice::assume_init_mut(dest) })
}

/// Default implementation of `inner_u32` on top of `fill_uninit`
#[inline]
pub fn inner_u32() -> Result<u32, SysError>
{
    let mut res = crate::mem::MaybeUninit::<u32>::uninit();
    // SAFETY: the created slice has the same size as `res`
    let dst = unsafe
    {
        let p: *mut crate::mem::MaybeUninit<u8> = res.as_mut_ptr().cast();
        crate::slice::from_raw_parts_mut(p, crate::mem::size_of::<u32>())
    };
    crate::fill_uninit(dst)?;
    // SAFETY: `dst` has been fully initialized by `imp::fill_inner` since it returned `Ok`.
    Ok(unsafe { res.assume_init() })
}

/// Get random `u32` from the system's preferred random number source.
#[inline] pub fn u32() -> Result<u32, SysError> { inner_u32() }

/// Default implementation of `inner_u64` on top of `fill_uninit`
#[inline] pub fn inner_u64() -> Result<u64, SysError>
{
    let mut res = crate::mem::MaybeUninit::<u64>::uninit();
    let dst = unsafe
    {
        let p: *mut crate::mem::MaybeUninit<u8> = res.as_mut_ptr().cast();
        slice::from_raw_parts_mut(p, crate::mem::size_of::<u64>())
    };

    crate::fill_uninit(dst)?;
    Ok(unsafe { res.assume_init() })
}
/// Get random `u64` from the system's preferred random number source.
#[inline] pub fn u64() -> Result<u64, SysError> { inner_u64() }