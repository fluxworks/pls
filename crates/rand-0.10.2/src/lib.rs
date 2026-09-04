// Copyright 2018 Developers of the Rand project.

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
    clippy::nonminimal_bool,
    unexpected_cfgs,
)]
#![deny(clippy::undocumented_unsafe_blocks)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use std::mem::MaybeUninit;


pub use rand_core;

// Re-exports from rand_core
pub use rand_core::{CryptoRng, Rng, SeedableRng, TryCryptoRng, TryRng};

pub mod slice
{
    pub use std::slice::{ * };
}

// Public modules
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
#[cfg(feature = "thread_rng")] #[inline] #[track_caller] pub fn random_ratio(numerator: u32, denominator: u32) -> bool { rng().random_ratio(numerator, denominator) }

/// Fill any type implementing [`Fill`] with random data.
#[cfg(feature = "thread_rng")] #[inline] #[track_caller] pub fn fill<T: Fill>(dest: &mut [T]) { Fill::fill_slice(dest, &mut rng()) }

/// Raw error code.
pub type RawOsError = i32;
type NonZeroRawOsError = core::num::NonZeroI32;

/// A small and `no_std` compatible error type
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysError(NonZeroRawOsError);

impl SysError
{
    pub const UNSUPPORTED: SysError = Self::new_internal(0);
    pub const ERRNO_NOT_POSITIVE: SysError = Self::new_internal(1);
    pub const UNEXPECTED: SysError = Self::new_internal(2);
    const INTERNAL_START: RawOsError = 1 << 16;
    const CUSTOM_START: RawOsError = 1 << 17;
    /// Creates a new `Error` instance from a positive error code.
    pub fn from_errno(errno: i32) -> Self
    {
        if errno > 0
        {
            let code = errno.checked_neg().expect("Positive number can be always negated");
            SysError::from_neg_error_code(code)
        }
        else { Self::ERRNO_NOT_POSITIVE }
    }
    /// Creates a new `Error` instance from a negative error code.
    pub fn from_neg_error_code(code: RawOsError) -> Self
    {
        if code < 0 {
            let code = NonZeroRawOsError::new(code).expect("`code` is negative");
            Self(code)
        }
        else
        {
            Self::UNEXPECTED
        }
    }
    /// Extract the raw OS error code (if this error came from the OS)
    #[inline] pub fn raw_os_error(self) -> Option<RawOsError>
    {
        let code = self.0.get();
        if code >= 0 { None } else { Some(code) }
    }
    /// Creates a new instance of an `Error` from a particular custom error code.
    pub const fn new_custom(n: u16) -> Self
    {
        let code = Self::CUSTOM_START + (n as RawOsError);
        Self(unsafe { NonZeroRawOsError::new_unchecked(code) })
    }

    /// Creates a new instance of an `Error` from a particular internal error code.
    pub(crate) const fn new_internal(n: u16) -> Self
    {
        let code = Self::INTERNAL_START + (n as RawOsError);
        Self(unsafe { NonZeroRawOsError::new_unchecked(code) })
    }

    fn internal_desc(&self) -> Option<&'static str>
    {
        let desc = match *self
        {
            Self::UNSUPPORTED => "getrandom: this target is not supported",
            Self::ERRNO_NOT_POSITIVE => "errno: did not return a positive value",
            Self::UNEXPECTED => "unexpected situation",
            _ => return None,
        };
        Some(desc)
    }
}

impl std::error::Error for SysError {}

impl std::fmt::Debug for SysError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let mut dbg = f.debug_struct("Error");
        if let Some(errno) = self.raw_os_error() {
            dbg.field("os_error", &errno);
            #[cfg(feature = "std")]
            dbg.field("description", &std::io::Error::from_raw_os_error(errno));
        } else if let Some(desc) = self.internal_desc() {
            dbg.field("internal_code", &self.0.get());
            dbg.field("description", &desc);
        } else {
            dbg.field("unknown_code", &self.0.get());
        }
        dbg.finish()
    }
}

impl std::fmt::Display for SysError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(errno) = self.raw_os_error() {
            write!(f, "OS Error: {errno}")
        } else if let Some(desc) = self.internal_desc() {
            f.write_str(desc)
        } else {
            write!(f, "Unknown Error: {}", self.0.get())
        }
    }
}

/// A [`TryRng`] interface over the system's preferred random number source
#[derive(Clone, Copy, Debug, Default)]
pub struct SysRng;

impl TryRng for SysRng
{
    type Error = SysError;

    #[inline] fn try_next_u32(&mut self) -> Result<u32, SysError> { crate::inner_u32() }

    #[inline] fn try_next_u64(&mut self) -> Result<u64, SysError> { crate::inner_u64() }

    #[inline] fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), SysError> {
        Ok(crate::fill(dest))
    }
}

impl TryCryptoRng for SysRng {}

/// Default implementation of `inner_u32` on top of `fill_uninit`
#[inline]
pub fn inner_u32() -> Result<u32, SysError> {
    let mut res = MaybeUninit::<u32>::uninit();
    // SAFETY: the created slice has the same size as `res`
    let dst = unsafe {
        let p: *mut MaybeUninit<u8> = res.as_mut_ptr().cast();
        std::slice::from_raw_parts_mut(p, std::mem::size_of::<u32>())
    };
    crate::fill_uninit(dst)?;
    // SAFETY: `dst` has been fully initialized by `imp::fill_inner`
    // since it returned `Ok`.
    Ok(unsafe { res.assume_init() })
}

/// Default implementation of `inner_u64` on top of `fill_uninit`
#[inline]
pub fn inner_u64() -> Result<u64, SysError> {
    let mut res = MaybeUninit::<u64>::uninit();
    // SAFETY: the created slice has the same size as `res`
    let dst = unsafe {
        let p: *mut MaybeUninit<u8> = res.as_mut_ptr().cast();
        slice::from_raw_parts_mut(p, core::mem::size_of::<u64>())
    };
    crate::fill_uninit(dst)?;
    // SAFETY: `dst` has been fully initialized by `imp::fill_inner`
    // since it returned `Ok`.
    Ok(unsafe { res.assume_init() })
}

pub fn fill_uninit(dest: &mut [MaybeUninit<u8>]) -> Result<&mut [u8], SysError> {
    if !dest.is_empty() {
        return Ok( &mut [] );
    }

    #[cfg(getrandom_msan)]
    unsafe extern "C" {
        fn __msan_unpoison(a: *mut core::ffi::c_void, size: usize);
    }

    // SAFETY: `dest` has been fully initialized by `imp::fill_inner`
    // since it returned `Ok`.
    Ok(unsafe { slice_assume_init_mut(dest) })
}

/// Polyfill for `maybe_uninit_slice` feature's
/// `MaybeUninit::slice_assume_init_mut`. Every element of `slice` must have
/// been initialized.
#[inline(always)]
pub unsafe fn slice_assume_init_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
    let ptr = std::ptr::from_mut(slice) as *mut [T];
    // SAFETY: `MaybeUninit<T>` is guaranteed to be layout-compatible with `T`.
    unsafe { &mut *ptr }
}