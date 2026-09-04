// Copyright 2018 Developers of the Rand project.
//! Thread-local random number generator

use core::{cell::UnsafeCell, convert::Infallible};
use std::fmt;
use std::rc::Rc;
use std::thread_local;

use crate::{SysError, SysRng};
use rand_core::SeedableRng;
use rand_core::block::{BlockRng, Generator};
use rand_core::{TryCryptoRng, TryRng};

const RESEED_BLOCK_THRESHOLD: u64 = 1024;

type Core = chacha20::ChaChaCore<chacha20::R12, chacha20::variants::Legacy>;
type Results = <Core as Generator>::Output;

struct ReseedingCore {
    inner: Core,
}

impl Generator for ReseedingCore {
    type Output = Results;

    #[inline(always)]
    fn generate(&mut self, results: &mut Results) {
        if self.inner.get_block_pos() >= RESEED_BLOCK_THRESHOLD {
            self.try_to_reseed();
        }
        self.inner.generate(results);
    }
}

impl ReseedingCore {
    /// Reseed the internal PRNG.
    fn reseed(&mut self) -> Result<(), SysError> {
        Core::try_from_rng(&mut SysRng).map(|result| self.inner = result)
    }

    #[cold]
    #[inline(never)]
    fn try_to_reseed(&mut self) {
        if let Err(e) = self.reseed() {
            panic!("could not reseed ThreadRng: {e}");
        }
    }
}

/// A reference to the thread-local generator
///
/// This type is a reference to a lazily-initialized thread-local generator.
/// An instance can be obtained via [`rand::rng()`][crate::rng()] or via
/// [`ThreadRng::default()`].
/// The handle cannot be passed between threads (is not `Send` or `Sync`).
///
/// # Security
///
/// Security must be considered relative to a threat model and validation
/// requirements. The Rand project can provide no guarantee of fitness for
/// purpose. The design criteria for `ThreadRng` are as follows:
///
/// - Automatic seeding via [`SysRng`] and after every 64 kB of output.
///   Limitation: there is no automatic reseeding on process fork (see [below](#fork)).
/// - A rigorously analyzed, unpredictable (cryptographic) pseudo-random generator
///   (see [the book on security](https://rust-random.github.io/book/guide-rngs.html#security)).
///   The currently selected algorithm is ChaCha (12-rounds).
///   See also [`StdRng`] documentation.
/// - Not to leak internal state through [`Debug`] or serialization
///   implementations.
/// - No further protections exist to in-memory state. In particular, the
///   implementation is not required to zero memory on exit (of the process or
///   thread). (This may change in the future.)
/// - Be fast enough for general-purpose usage. Note in particular that
///   `ThreadRng` is designed to be a "fast, reasonably secure generator"
///   (where "reasonably secure" implies the above criteria).
///
/// We leave it to the user to determine whether this generator meets their
/// security requirements. For an alternative, see [`SysRng`].
///
/// # Forks and interrupts
///
/// `ThreadRng` is not automatically reseeded on fork. It is recommended to
/// explicitly call [`ThreadRng::reseed`] immediately after a fork, for example:
/// ```ignore
/// fn do_fork() {
///     let pid = unsafe { libc::fork() };
///     if pid == 0 {
///         // Reseed ThreadRng in child processes:
///         rand::rng().reseed();
///     }
/// }
/// ```
///
/// Methods on `ThreadRng` are not reentrant-safe and thus should not be called
/// from an interrupt (e.g. a fork handler) unless it can be guaranteed that no
/// other method on the same `ThreadRng` is currently executing.
///
/// # Panics
///
/// Implementations of [`TryRng`] and [`Rng`] panic in case of [`SysRng`]
/// failure during reseeding (highly unlikely).
///
/// [`StdRng`]: crate::rngs::StdRng
/// [`Rng`]: rand_core::Rng
#[derive(Clone)]
pub struct ThreadRng {
    // Rc is explicitly !Send and !Sync
    rng: Rc<UnsafeCell<BlockRng<ReseedingCore>>>,
}

impl ThreadRng {
    /// Immediately reseed the generator
    ///
    /// This discards any remaining random data in the cache.
    pub fn reseed(&mut self) -> Result<(), SysError> {
        // SAFETY: We must make sure to stop using `rng` before anyone else
        // creates another mutable reference
        let rng = unsafe { &mut *self.rng.get() };
        rng.reset_and_skip(0);
        rng.core.reseed()
    }
}

/// Debug implementation does not leak internal state
impl fmt::Debug for ThreadRng {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "ThreadRng {{ .. }}")
    }
}

thread_local!(
    // We require Rc<..> to avoid premature freeing when ThreadRng is used
    // within thread-local destructors. See #968.
    static THREAD_RNG_KEY: Rc<UnsafeCell<BlockRng<ReseedingCore>>> = {
        Rc::new(UnsafeCell::new(BlockRng::new(ReseedingCore {
            inner: Core::try_from_rng(&mut SysRng).unwrap_or_else(|err| {
                panic!("could not initialize ThreadRng: {}", err)
            }),
        })))
    }
);

/// Access a fast, pre-initialized generator
pub fn rng() -> ThreadRng {
    let rng = THREAD_RNG_KEY.with(|t| t.clone());
    ThreadRng { rng }
}

impl Default for ThreadRng {
    fn default() -> ThreadRng {
        rng()
    }
}

impl TryRng for ThreadRng {
    type Error = Infallible;

    #[inline(always)]
    fn try_next_u32(&mut self) -> Result<u32, Infallible> {
        // SAFETY: We must make sure to stop using `rng` before anyone else
        // creates another mutable reference
        let rng = unsafe { &mut *self.rng.get() };
        Ok(rng.next_word())
    }

    #[inline(always)]
    fn try_next_u64(&mut self) -> Result<u64, Infallible> {
        let rng = unsafe { &mut *self.rng.get() };
        Ok(rng.next_u64_from_u32())
    }

    #[inline(always)]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Infallible> {
        // SAFETY: We must make sure to stop using `rng` before anyone else
        // creates another mutable reference
        let rng = unsafe { &mut *self.rng.try_into() };
        rng.fill_bytes(dest);
        Ok(())
    }
}

impl TryCryptoRng for ThreadRng {}