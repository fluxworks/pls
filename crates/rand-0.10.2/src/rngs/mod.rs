// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Random number generators and adapters
//!
//! ## Generators
//!
//! This crate provides a small selection of generators.
//! See also [Types of generators] and [Our RNGs] in the book.
//!
//! ##### Non-deterministic generators
//!
//! -   [`SysRng`] is a stateless interface over the operating system's random number
//!     source. This is typically secure with some form of periodic re-seeding.
//! -   [`ThreadRng`], provided by [`crate::rng()`], is a handle to a
//!     thread-local generator with periodic seeding from [`SysRng`]. Because this
//!     is local, it is typically much faster than [`SysRng`]. It should be
//!     secure, but see documentation on [`ThreadRng`].
//!
//! ##### Standard generators
//!
//! These use selected best-in-class algorithms. They are deterministic but not
//! portable: the algorithms may be changed in any release and may be
//! platform-dependent.
//!
//! -   [`StdRng`] is a CSPRNG chosen for good performance and trust of security
//!     (based on reviews, maturity and usage). The current algorithm is
//!     [`ChaCha12Rng`], which is well established and rigorously analysed.
//!     [`StdRng`] is the deterministic generator used by [`ThreadRng`] but
//!     without the periodic reseeding or thread-local management.
//! -   [`SmallRng`] is a relatively simple, insecure generator designed to be
//!     fast, use little memory, and pass various statistical tests of
//!     randomness quality. The current algorithm is one of the Xoshiro
//!     generators below, depending on the target's pointer size.
//!
//! ##### Named portable generators
//!
//! These are similar to the [standard generators](#standard-generators), but
//! with the additional [guarantees of reproducibility]:
//!
//! -   [`Xoshiro256PlusPlus`] is a very fast 64-bit insecure generator using
//!     256 bits of state with good performance in statistical tests of quality
//! -   [`Xoshiro128PlusPlus`] is a very fast 32-bit insecure generator using
//!     128 bits of state with good performance in statistical tests of quality
//! -   [`ChaCha8Rng`], [`ChaCha12Rng`] and [`ChaCha20Rng`] are generators over
//!     the ChaCha stream cipher designed by Daniel J. Bernstein[^1].
//!
//! ### Additional generators
//!
//! -   The [`rdrand`] crate provides an interface to the RDRAND and RDSEED
//!     instructions available in modern Intel and AMD CPUs.
//! -   The [`rand_jitter`] crate provides a user-space implementation of
//!     entropy harvesting from CPU timer jitter, but is very slow and has
//!     [security issues](https://github.com/rust-random/rand/issues/699).
//! -   The [`rand_pcg`] crate provides portable implementations of a subset
//!     of the [PCG] family of small, insecure generators
//! -   The [`rand_xoshiro`] crate provides portable implementations of the
//!     [xoshiro] family of small, insecure generators
//!
//! For more, search [crates with the `rng` tag].
//!
//! ## Traits and functionality
//!
//! All generators implement [`TryRng`]. Most implement [`Rng`] (i.e.
//! `TryRng<Error = Infallible>`) and thus also implement [`RngExt`].
//! See also the [Random Values] chapter in the book.
//!
//! Secure RNGs may additionally implement the [`CryptoRng`] trait.
//!
//! Use the [`rand_core`] crate when implementing your own RNGs.
//!
//! [^1]: D. J. Bernstein, [*ChaCha, a variant of Salsa20*](https://cr.yp.to/chacha.html)
//!
//! [guarantees of reproducibility]: https://rust-random.github.io/book/crate-reprod.html
//! [Types of generators]: https://rust-random.github.io/book/guide-gen.html
//! [Our RNGs]: https://rust-random.github.io/book/guide-rngs.html
//! [Random Values]: https://rust-random.github.io/book/guide-values.html
//! [`RngExt`]: crate::RngExt
//! [`TryRng`]: crate::TryRng
//! [`Rng`]: crate::Rng
//! [`CryptoRng`]: crate::CryptoRng
//! [`SeedableRng`]: crate::SeedableRng
//! [`rdrand`]: https://crates.io/crates/rdrand
//! [`rand_jitter`]: https://crates.io/crates/rand_jitter
//! [`rand_pcg`]: https://crates.io/crates/rand_pcg
//! [`rand_xoshiro`]: https://crates.io/crates/rand_xoshiro
//! [crates with the `rng` tag]: https://crates.io/keywords/rng
//! [chacha]: https://cr.yp.to/chacha.html
//! [PCG]: https://www.pcg-random.org/
//! [xoshiro]: https://prng.di.unimi.it/

mod small;
mod xoshiro128plusplus;
mod xoshiro256plusplus;

#[cfg(feature = "std_rng")]
mod std;
#[cfg(feature = "thread_rng")]
pub(crate) mod thread;

pub use self::small::SmallRng;
pub use xoshiro128plusplus::Xoshiro128PlusPlus;
pub use xoshiro256plusplus::Xoshiro256PlusPlus;

#[cfg(feature = "std_rng")]
pub use self::std::StdRng;
#[cfg(feature = "thread_rng")]
pub use self::thread::ThreadRng;

#[cfg(feature = "chacha")]
pub use chacha20::{ChaCha8Rng, ChaCha12Rng, ChaCha20Rng};

#[cfg(feature = "sys_rng")]
//pub use getrandom::{Error as SysError, SysRng};
/// Raw error code.
pub type RawOsError = i32;
type NonZeroRawOsError = core::num::NonZeroI32;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysError(NonZeroRawOsError);

impl SysError {
    /// This target/platform is not supported by `getrandom`.
    pub const UNSUPPORTED: SysError = Self::new_internal(0);
    /// The platform-specific `errno` returned a non-positive value.
    pub const ERRNO_NOT_POSITIVE: SysError = Self::new_internal(1);
    /// Encountered an unexpected situation which should not happen in practice.
    pub const UNEXPECTED: SysError = Self::new_internal(2);

    /// Internal errors can be in the range of 2^16..2^17
    const INTERNAL_START: RawOsError = 1 << 16;
    /// Custom errors can be in the range of 2^17..(2^17 + 2^16)
    const CUSTOM_START: RawOsError = 1 << 17;

    /// Creates a new `Error` instance from a positive error code.
    ///
    /// Returns [`Error::ERRNO_NOT_POSITIVE`] for zero and negative error codes.
    #[cfg(not(target_os = "uefi"))]
    #[allow(dead_code)]
    pub(super) fn from_errno(errno: i32) -> Self {
        if errno > 0 {
            let code = errno
                .checked_neg()
                .expect("Positive number can be always negated");
            SysError::from_neg_error_code(code)
        } else {
            SysError::ERRNO_NOT_POSITIVE
        }
    }

    /// Creates a new `Error` instance from a negative error code.
    ///
    /// Returns [`Error::UNEXPECTED`] for zero and positive error codes.
    #[cfg(not(target_os = "uefi"))]
    #[allow(dead_code)]
    pub(super) fn from_neg_error_code(code: RawOsError) -> Self {
        if code < 0 {
            let code = NonZeroRawOsError::new(code).expect("`code` is negative");
            Self(code)
        } else {
            SysError::UNEXPECTED
        }
    }

    /// Creates a new instance of an `Error` from an UEFI error code.
    #[cfg(target_os = "uefi")]
    #[allow(dead_code)]
    pub(super) fn from_uefi_code(code: RawOsError) -> Self {
        if code & UEFI_ERROR_FLAG != 0 {
            let code = NonZeroRawOsError::new(code).expect("The highest bit of `code` is set to 1");
            Self(code)
        } else {
            Self::UNEXPECTED
        }
    }

    /// Extract the raw OS error code (if this error came from the OS)
    ///
    /// This method is identical to [`std::io::Error::raw_os_error()`][1], except
    /// that it works in `no_std` contexts. On most targets this method returns
    /// `Option<i32>`, but some platforms (e.g. UEFI) may use a different primitive
    /// type like `usize`. Consult with the [`RawOsError`] docs for more information.
    ///
    /// If this method returns `None`, the error value can still be formatted via
    /// the `Display` implementation.
    ///
    /// [1]: https://doc.rust-lang.org/std/io/struct.Error.html#method.raw_os_error
    /// [`RawOsError`]: https://doc.rust-lang.org/std/io/type.RawOsError.html
    #[inline]
    pub fn raw_os_error(self) -> Option<RawOsError> {
        let code = self.0.get();

        // note: in this method we need to cover only backends which rely on
        // `Error::{from_error_code, from_errno, from_uefi_code}` methods,
        // on all other backends this method always returns `None`.

        #[cfg(target_os = "uefi")]
        {
            if code & UEFI_ERROR_FLAG != 0 {
                Some(code)
            } else {
                None
            }
        }

        #[cfg(not(target_os = "uefi"))]
        {
            // On most targets `std` expects positive error codes while retrieving error strings:
            // - `libc`-based targets use `strerror_r` which expects positive error codes.
            // - Hermit relies on the `hermit-abi` crate, which expects positive error codes:
            //   https://docs.rs/hermit-abi/0.4.0/src/hermit_abi/errno.rs.html#400-532
            // - WASIp1 uses the same conventions as `libc`:
            //   https://github.com/rust-lang/rust/blob/1.85.0/library/std/src/sys/pal/wasi/os.rs#L57-L67
            //
            // The only exception is Solid, `std` expects negative system error codes, see:
            // https://github.com/rust-lang/rust/blob/1.85.0/library/std/src/sys/pal/solid/error.rs#L5-L31
            if code >= 0 {
                None
            } else if cfg!(not(target_os = "solid_asp3")) {
                code.checked_neg()
            } else {
                Some(code)
            }
        }
    }

    /// Creates a new instance of an `Error` from a particular custom error code.
    pub const fn new_custom(n: u16) -> SysError {
        // SAFETY: code > 0 as CUSTOM_START > 0 and adding `n` won't overflow `RawOsError`.
        let code = SysError::CUSTOM_START + (n as RawOsError);
        SysError(unsafe { NonZeroRawOsError::new_unchecked(code) })
    }

    /// Creates a new instance of an `Error` from a particular internal error code.
    pub(crate) const fn new_internal(n: u16) -> SysError {
        // SAFETY: code > 0 as INTERNAL_START > 0 and adding `n` won't overflow `RawOsError`.
        let code = SysError::INTERNAL_START + (n as RawOsError);
        SysError(unsafe { NonZeroRawOsError::new_unchecked(code) })
    }

    fn internal_desc(&self) -> Option<&'static str> {
        let desc = match *self {
            SysError::UNSUPPORTED => "getrandom: this target is not supported",
            SysError::ERRNO_NOT_POSITIVE => "errno: did not return a positive value",
            SysError::UNEXPECTED => "unexpected situation",
            #[cfg(any(
                target_os = "ios",
                target_os = "visionos",
                target_os = "watchos",
                target_os = "tvos",
            ))]
            Error::IOS_RANDOM_GEN => "SecRandomCopyBytes: iOS Security framework failure",
            #[cfg(all(windows, target_vendor = "win7"))]
            Error::WINDOWS_RTL_GEN_RANDOM => "RtlGenRandom: Windows system function failure",
            #[cfg(all(
                feature = "wasm_js",
                target_arch = "wasm32",
                any(target_os = "unknown", target_os = "none")
            ))]
            Error::WEB_CRYPTO => "Web Crypto API is unavailable",
            #[cfg(target_os = "vxworks")]
            Error::VXWORKS_RAND_SECURE => "randSecure: VxWorks RNG module is not initialized",

            #[cfg(any(
                getrandom_backend = "rdrand",
                all(target_arch = "x86_64", target_env = "sgx")
            ))]
            Error::FAILED_RDRAND => "RDRAND: failed multiple times: CPU issue likely",
            #[cfg(any(
                getrandom_backend = "rdrand",
                all(target_arch = "x86_64", target_env = "sgx")
            ))]
            Error::NO_RDRAND => "RDRAND: instruction not supported",

            #[cfg(getrandom_backend = "rndr")]
            Error::RNDR_FAILURE => "RNDR: Could not generate a random number",
            #[cfg(getrandom_backend = "rndr")]
            Error::RNDR_NOT_AVAILABLE => "RNDR: Register not supported",
            _ => return None,
        };
        Some(desc)
    }
}

impl core::error::Error for SysError {}

impl core::fmt::Debug for SysError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut dbg = f.debug_struct("Error");
        if let Some(errno) = self.raw_os_error() {
            dbg.field("os_error", &errno);
            #[cfg(feature = "std")]
            dbg.field("description", &core::io::Error::from_raw_os_error(errno));
        } else if let Some(desc) = self.internal_desc() {
            dbg.field("internal_code", &self.0.get());
            dbg.field("description", &desc);
        } else {
            dbg.field("unknown_code", &self.0.get());
        }
        dbg.finish()
    }
}

impl core::fmt::Display for SysError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(errno) = self.raw_os_error() {
            std::io::Error::from_raw_os_error(errno).fmt(f)
        } else if let Some(desc) = self.internal_desc() {
            f.write_str(desc)
        } else {
            write!(f, "Unknown Error: {}", self.0.get())
        }
    }
}

/// A [`TryRng`] interface over the system's preferred random number source.
#[derive(Clone, Copy, Debug, Default)]
pub struct SysRng;

use rand_core::{TryCryptoRng, TryRng};

impl TryRng for SysRng
{
    type Error = SysError;
    #[inline] fn try_next_u32(&mut self) -> Result<u32, Error> {
        crate::u32()
    }
    #[inline] fn try_next_u64(&mut self) -> Result<u64, Error> {
        crate::u64()
    }
    #[inline] fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> { crate::fill(dest) }
}

impl TryCryptoRng for SysRng {}