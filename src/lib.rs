//! pls is a bash-like Unix shell written in Rust.
#![allow
(
    dead_code,
    unknown_lints,
    unused_imports,
    unused_unsafe,
    unused_variables,
)]

extern crate regex as re;

#[macro_use] pub mod macros
{

}

pub mod api
{
    use crate::
    {
        *,
    };

    pub const API:[&str; 20] =  [ "alias", "bg", "cd", "check", "cinfo", "exec", "exit", "export", "fg", "history", "jobs", "read", "source", "ulimit", "unalias", "vox", "minfd", "set", "unset", "unpath", ];
}

pub mod collections
{
    pub use std::collections::{ * };
}

pub mod env
{
    pub use std::env::{ * };
    use crate::
    {
        *,
    };
}

pub mod hash
{
    pub use std::hash::{ * };
}

pub mod ffi
{
    pub use std::ffi::{ * };
}

pub mod intrinsics
{
    //! Compiler intrinsics.
    use crate::
    {
        ffi::{VaArgSafe, VaList},
        marker::{ConstParamTy, DiscriminantKind, PointeeSized, Tuple},
        *,
    };
    
    mod bounds
    {
        use crate::
        {
            *,
        };
    }

    pub mod fallback
    {
        use crate::
        {
            *,
        };
    }

    pub mod gpu
    {
        use crate::
        {
            *,
        };
    }

    pub mod mir
    {
        use crate::
        {
            *,
        };
    } 

    // These imports are used for simplifying intra-doc links
    #[allow(unused_imports)]
    #[cfg(all(target_has_atomic = "8", target_has_atomic = "32", target_has_atomic = "ptr"))]
    use crate::sync::atomic::{self, AtomicBool, AtomicI32, AtomicIsize, AtomicU32, Ordering};

    /// A type for atomic ordering parameters for intrinsics.
    #[allow(missing_docs)]
    //#[derive(Debug, ConstParamTy, PartialEq, Eq)]
    #[derive(Debug, PartialEq, Eq)]
    pub enum AtomicOrdering
    {
        Relaxed = 0,
        Release = 1,
        Acquire = 2,
        AcqRel = 3,
        SeqCst = 4,
    }
    /// Stores a value if the current value is the same as the `old` value.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_cxchg
    <T: Copy, const ORD_SUCC: AtomicOrdering, const ORD_FAIL: AtomicOrdering>( dst: *mut T, old: T, src: T, ) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_cxchgweak<
        T: Copy,
        const ORD_SUCC: AtomicOrdering,
        const ORD_FAIL: AtomicOrdering,
    >(
        _dst: *mut T,
        _old: T,
        _src: T,
    ) -> (T, bool);
    /// Value pointer.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_load<T: Copy, const ORD: AtomicOrdering>(src: *const T) -> T;
    /// Memory location.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_store<T: Copy, const ORD: AtomicOrdering>(dst: *mut T, val: T);
    /// Stale value.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_xchg<T: Copy, const ORD: AtomicOrdering>(dst: *mut T, src: T) -> T;
    /// Previous value.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_xadd<T: Copy, U: Copy, const ORD: AtomicOrdering>(dst: *mut T, src: U) -> T;
    /// Previous value.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_xsub<T: Copy, U: Copy, const ORD: AtomicOrdering>(dst: *mut T, src: U) -> T;
    /// Previous value.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_and<T: Copy, U: Copy, const ORD: AtomicOrdering>(dst: *mut T, src: U) -> T;
    /// Previous value.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_nand<T: Copy, U: Copy, const ORD: AtomicOrdering>(dst: *mut T, src: U) -> T;
    /// Previous value.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_or<T: Copy, U: Copy, const ORD: AtomicOrdering>(dst: *mut T, src: U) -> T;
    /// Previous value.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_xor<T: Copy, U: Copy, const ORD: AtomicOrdering>(dst: *mut T, src: U) -> T;
    /// Maximum with the current value using a signed comparison.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_max<T: Copy, const ORD: AtomicOrdering>(dst: *mut T, src: T) -> T;
    /// Minimum with the current value using a signed comparison.
    /// `T` must be a signed integer type.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] signed integer types via the `fetch_min` method. For example, [`AtomicI32::fetch_min`].
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_min<T: Copy, const ORD: AtomicOrdering>(dst: *mut T, src: T) -> T;
    /// Minimum with the current value using an unsigned comparison.
    /// `T` must be an unsigned integer type.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] unsigned integer types via the `fetch_min` method. For example, [`AtomicU32::fetch_min`].
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_umin<T: Copy, const ORD: AtomicOrdering>(dst: *mut T, src: T) -> T;
    /// Maximum with the current value using an unsigned comparison.
    /// `T` must be an unsigned integer type.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] unsigned integer types via the `fetch_max` method. For example, [`AtomicU32::fetch_max`].
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_umax<T: Copy, const ORD: AtomicOrdering>(dst: *mut T, src: T) -> T;
    /// An atomic fence.
    ///
    /// The stabilized version of this intrinsic is available in
    /// [`atomic::fence`].
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_fence<const ORD: AtomicOrdering>();

    /// An atomic fence for synchronization within a single thread.
    ///
    /// The stabilized version of this intrinsic is available in
    /// [`atomic::compiler_fence`].
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn atomic_singlethreadfence<const ORD: AtomicOrdering>();

    /// The `prefetch` intrinsic is a hint to the code generator to insert a prefetch instruction
    /// for the given address if supported; otherwise, it is a no-op.
    /// Prefetches have no effect on the behavior of the program but can change its performance
    /// characteristics.
    ///
    /// The `LOCALITY` argument is a temporal locality specifier ranging from (0) - no locality,
    /// to (3) - extremely local keep in cache.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic]
    #[rustc_nounwind]
    #[miri::intrinsic_fallback_is_spec]
    pub const fn prefetch_read_data<T, const LOCALITY: i32>(data: *const T) {
        // This operation is a no-op, unless it is overridden by the backend.
        let _ = data;
    }

    /// The `prefetch` intrinsic is a hint to the code generator to insert a prefetch instruction
    /// for the given address if supported; otherwise, it is a no-op.
    /// Prefetches have no effect on the behavior of the program but can change its performance
    /// characteristics.
    ///
    /// The `LOCALITY` argument is a temporal locality specifier ranging from (0) - no locality,
    /// to (3) - extremely local keep in cache.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic]
    #[rustc_nounwind]
    #[miri::intrinsic_fallback_is_spec]
    pub const fn prefetch_write_data<T, const LOCALITY: i32>(data: *const T) {
        // This operation is a no-op, unless it is overridden by the backend.
        let _ = data;
    }

    /// The `prefetch` intrinsic is a hint to the code generator to insert a prefetch instruction
    /// for the given address if supported; otherwise, it is a no-op.
    /// Prefetches have no effect on the behavior of the program but can change its performance
    /// characteristics.
    ///
    /// The `LOCALITY` argument is a temporal locality specifier ranging from (0) - no locality,
    /// to (3) - extremely local keep in cache.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic]
    #[rustc_nounwind]
    #[miri::intrinsic_fallback_is_spec]
    pub const fn prefetch_read_instruction<T, const LOCALITY: i32>(data: *const T) {
        // This operation is a no-op, unless it is overridden by the backend.
        let _ = data;
    }

    /// The `prefetch` intrinsic is a hint to the code generator to insert a prefetch instruction
    /// for the given address if supported; otherwise, it is a no-op.
    /// Prefetches have no effect on the behavior of the program but can change its performance
    /// characteristics.
    ///
    /// The `LOCALITY` argument is a temporal locality specifier ranging from (0) - no locality,
    /// to (3) - extremely local keep in cache.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic]
    #[rustc_nounwind]
    #[miri::intrinsic_fallback_is_spec]
    pub const fn prefetch_write_instruction<T, const LOCALITY: i32>(data: *const T) {
        // This operation is a no-op, unless it is overridden by the backend.
        let _ = data;
    }

    /// Executes a breakpoint trap, for inspection by a debugger.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn breakpoint();

    /// Magic intrinsic that derives its meaning from attributes
    /// attached to the function.
    ///
    /// For example, dataflow uses this to inject static assertions so
    /// that `rustc_peek(potentially_uninitialized)` would actually
    /// double-check that dataflow did indeed compute that it is
    /// uninitialized at that point in the control flow.
    ///
    /// This intrinsic should not be used outside of the compiler.
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub fn rustc_peek<T>(_: T) -> T;
    /// Aborts the execution of the process.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// [`std::process::abort`](../../std/process/fn.abort.html) is to be preferred if possible,
    /// as its behavior is more user-friendly and more stable.
    ///
    /// The current implementation of `intrinsics::abort` is to invoke an invalid instruction,
    /// on most platforms.
    /// On Unix, the
    /// process will probably terminate with a signal like `SIGABRT`, `SIGILL`, `SIGTRAP`, `SIGSEGV` or
    /// `SIGBUS`.  The precise behavior is not guaranteed and not stable.
    ///
    /// The stabilization-track version of this intrinsic is [`core::process::abort_immediate`].
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub fn abort() -> !;

    /// Informs the optimizer that this point in the code is not reachable,
    /// enabling further optimizations.
    ///
    /// N.B., this is very different from the `unreachable!()` macro: Unlike the
    /// macro, which panics when it is executed, it is *undefined behavior* to
    /// reach code marked with this function.
    ///
    /// The stabilized version of this intrinsic is [`core::hint::unreachable_unchecked`].
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn unreachable() -> !;

    /// Informs the optimizer that a condition is always true.
    /// If the condition is false, the behavior is undefined.
    ///
    /// No code is generated for this intrinsic, but the optimizer will try
    /// to preserve it (and its condition) between passes, which may interfere
    /// with optimization of surrounding code and reduce performance. It should
    /// not be used if the invariant can be discovered by the optimizer on its
    /// own, or if it does not enable any significant optimizations.
    ///
    /// The stabilized version of this intrinsic is [`core::hint::assert_unchecked`].
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_intrinsic]
    pub const unsafe fn assume(b: bool) {
        if !b {
            // SAFETY: the caller must guarantee the argument is never `false`
            unsafe { unreachable() }
        }
    }

    /// Hints to the compiler that current code path is cold.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is [`core::hint::cold_path`].
    #[rustc_intrinsic]
    #[rustc_nounwind]
    #[miri::intrinsic_fallback_is_spec]
    #[cold]
    pub const fn cold_path() {}

    /// Hints to the compiler that branch condition is likely to be true.
    /// Returns the value passed to it.
    ///
    /// Any use other than with `if` statements will probably not have an effect.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_nounwind]
    #[inline(always)]
    pub const fn likely(b: bool) -> bool {
        if b {
            true
        } else {
            cold_path();
            false
        }
    }

    /// Hints to the compiler that branch condition is likely to be false.
    /// Returns the value passed to it.
    ///
    /// Any use other than with `if` statements will probably not have an effect.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_nounwind]
    #[inline(always)]
    pub const fn unlikely(b: bool) -> bool {
        if b {
            cold_path();
            true
        } else {
            false
        }
    }

    /// Returns either `true_val` or `false_val` depending on condition `b` with a
    /// hint to the compiler that this condition is unlikely to be correctly
    /// predicted by a CPU's branch predictor (e.g. a binary search).
    ///
    /// This is otherwise functionally equivalent to `if b { true_val } else { false_val }`.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The public form of this intrinsic is [`core::hint::select_unpredictable`].
    /// However unlike the public form, the intrinsic will not drop the value that
    /// is not selected.
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_const_unstable(feature = "const_select_unpredictable", issue = "145938")]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    #[miri::intrinsic_fallback_is_spec]
    #[inline]
    pub const fn select_unpredictable<T>(b: bool, true_val: T, false_val: T) -> T {
        if b {
            forget(false_val);
            true_val
        } else {
            forget(true_val);
            false_val
        }
    }

    /// A guard for unsafe functions that cannot ever be executed if `T` is uninhabited:
    /// This will statically either panic, or do nothing. It does not *guarantee* to ever panic,
    /// and should only be called if an assertion failure will imply language UB in the following code.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn assert_inhabited<T>();

    /// A guard for unsafe functions that cannot ever be executed if `T` does not permit
    /// zero-initialization: This will statically either panic, or do nothing. It does not *guarantee*
    /// to ever panic, and should only be called if an assertion failure will imply language UB in the
    /// following code.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn assert_zero_valid<T>();

    /// A guard for `std::mem::uninitialized`. This will statically either panic, or do nothing. It does
    /// not *guarantee* to ever panic, and should only be called if an assertion failure will imply
    /// language UB in the following code.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn assert_mem_uninitialized_valid<T>();

    /// Gets a reference to a static `Location` indicating where it was called.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// Consider using [`core::panic::Location::caller`] instead.
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn caller_location() -> &'static crate::panic::Location<'static>;

    /// Moves a value out of scope without running drop glue.
    ///
    /// This exists solely for [`crate::mem::forget_unsized`]; normal `forget` uses
    /// `ManuallyDrop` instead.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn forget<T: ?Sized>(_: T);

    /// Reinterprets the bits of a value of one type as another type.
    ///
    /// Both types must have the same size. Compilation will fail if this is not guaranteed.
    ///
    /// `transmute` is semantically equivalent to a bitwise move of one type
    /// into another. It copies the bits from the source value into the
    /// destination value, then forgets the original. Note that source and destination
    /// are passed by-value, which means if `Src` or `Dst` contain padding, that padding
    /// is *not* guaranteed to be preserved by `transmute`.
    ///
    /// Both the argument and the result must be [valid](../../nomicon/what-unsafe-does.html) at
    /// their given type. Violating this condition leads to [undefined behavior][ub]. The compiler
    /// will generate code *assuming that you, the programmer, ensure that there will never be
    /// undefined behavior*. It is therefore your responsibility to guarantee that every value
    /// passed to `transmute` is valid at both types `Src` and `Dst`. Failing to uphold this condition
    /// may lead to unexpected and unstable compilation results. This makes `transmute` **incredibly
    /// unsafe**. `transmute` should be the absolute last resort.
    ///
    /// Because `transmute` is a by-value operation, alignment of the *transmuted values
    /// themselves* is not a concern. As with any other function, the compiler already ensures
    /// both `Src` and `Dst` are properly aligned. However, when transmuting values that *point
    /// elsewhere* (such as pointers, references, boxes…), the caller has to ensure proper
    /// alignment of the pointed-to values.
    ///
    /// The [nomicon](../../nomicon/transmutes.html) has additional documentation.
    ///
    /// [ub]: ../../reference/behavior-considered-undefined.html
    ///
    /// # Transmutation between pointers and integers
    ///
    /// Special care has to be taken when transmuting between pointers and integers, e.g.
    /// transmuting between `*const ()` and `usize`.
    ///
    /// Transmuting *pointers to integers* in a `const` context is [undefined behavior][ub], unless
    /// the pointer was originally created *from* an integer. (That includes this function
    /// specifically, integer-to-pointer casts, and helpers like [`dangling`][crate::ptr::dangling],
    /// but also semantically-equivalent conversions such as punning through `repr(C)` union
    /// fields.) Any attempt to use the resulting value for integer operations will abort
    /// const-evaluation. (And even outside `const`, such transmutation is touching on many
    /// unspecified aspects of the Rust memory model and should be avoided. See below for
    /// alternatives.)
    ///
    /// Transmuting *integers to pointers* is a largely unspecified operation. It is likely *not*
    /// equivalent to an `as` cast. Doing non-zero-sized memory accesses with a pointer constructed
    /// this way is currently considered undefined behavior.
    ///
    /// All this also applies when the integer is nested inside an array, tuple, struct, or enum.
    /// However, `MaybeUninit<usize>` is not considered an integer type for the purpose of this
    /// section. Transmuting `*const ()` to `MaybeUninit<usize>` is fine---but then calling
    /// `assume_init()` on that result is considered as completing the pointer-to-integer transmute
    /// and thus runs into the issues discussed above.
    ///
    /// In particular, doing a pointer-to-integer-to-pointer roundtrip via `transmute` is *not* a
    /// lossless process. If you want to round-trip a pointer through an integer in a way that you
    /// can get back the original pointer, you need to use `as` casts, or replace the integer type
    /// by `MaybeUninit<$int>` (and never call `assume_init()`). If you are looking for a way to
    /// store data of arbitrary type, also use `MaybeUninit<T>` (that will also handle uninitialized
    /// memory due to padding). If you specifically need to store something that is "either an
    /// integer or a pointer", use `*mut ()`: integers can be converted to pointers and back without
    /// any loss (via `as` casts or via `transmute`).
    ///
    /// # Examples
    ///
    /// There are a few things that `transmute` is really useful for.
    ///
    /// Turning a pointer into a function pointer. This is *not* portable to
    /// machines where function pointers and data pointers have different sizes.
    ///
    /// ```
    /// fn foo() -> i32 {
    ///     0
    /// }
    /// // Crucially, we `as`-cast to a raw pointer before `transmute`ing to a function pointer.
    /// // This avoids an integer-to-pointer `transmute`, which can be problematic.
    /// // Transmuting between raw pointers and function pointers (i.e., two pointer types) is fine.
    /// let pointer = foo as fn() -> i32 as *const ();
    /// let function = unsafe {
    ///     std::mem::transmute::<*const (), fn() -> i32>(pointer)
    /// };
    /// assert_eq!(function(), 0);
    /// ```
    ///
    /// Extending a lifetime, or shortening an invariant lifetime. This is
    /// advanced, very unsafe Rust!
    ///
    /// ```
    /// struct R<'a>(&'a i32);
    /// unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
    ///     unsafe { std::mem::transmute::<R<'b>, R<'static>>(r) }
    /// }
    ///
    /// unsafe fn shorten_invariant_lifetime<'b, 'c>(r: &'b mut R<'static>)
    ///                                              -> &'b mut R<'c> {
    ///     unsafe { std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r) }
    /// }
    /// ```
    ///
    /// # Alternatives
    ///
    /// Don't despair: many uses of `transmute` can be achieved through other means.
    /// Below are common applications of `transmute` which can be replaced with safer
    /// constructs.
    ///
    /// Turning raw bytes (`[u8; SZ]`) into `u32`, `f64`, etc.:
    ///
    /// ```
    /// # #![allow(unnecessary_transmutes)]
    /// let raw_bytes = [0x78, 0x56, 0x34, 0x12];
    ///
    /// let num = unsafe {
    ///     std::mem::transmute::<[u8; 4], u32>(raw_bytes)
    /// };
    ///
    /// // use `u32::from_ne_bytes` instead
    /// let num = u32::from_ne_bytes(raw_bytes);
    /// // or use `u32::from_le_bytes` or `u32::from_be_bytes` to specify the endianness
    /// let num = u32::from_le_bytes(raw_bytes);
    /// assert_eq!(num, 0x12345678);
    /// let num = u32::from_be_bytes(raw_bytes);
    /// assert_eq!(num, 0x78563412);
    /// ```
    ///
    /// Turning a pointer into a `usize`:
    ///
    /// ```no_run
    /// let ptr = &0;
    /// let ptr_num_transmute = unsafe {
    ///     std::mem::transmute::<&i32, usize>(ptr)
    /// };
    ///
    /// // Use an `as` cast instead
    /// let ptr_num_cast = ptr as *const i32 as usize;
    /// ```
    ///
    /// Note that using `transmute` to turn a pointer to a `usize` is (as noted above) [undefined
    /// behavior][ub] in `const` contexts. Also outside of consts, this operation might not behave
    /// as expected -- this is touching on many unspecified aspects of the Rust memory model.
    /// Depending on what the code is doing, the following alternatives are preferable to
    /// pointer-to-integer transmutation:
    /// - If the code just wants to store data of arbitrary type in some buffer and needs to pick a
    ///   type for that buffer, it can use [`MaybeUninit`][crate::mem::MaybeUninit].
    /// - If the code actually wants to work on the address the pointer points to, it can use `as`
    ///   casts or [`ptr.addr()`][pointer::addr].
    ///
    /// Turning a `*mut T` into a `&mut T`:
    ///
    /// ```
    /// let ptr: *mut i32 = &mut 0;
    /// let ref_transmuted = unsafe {
    ///     std::mem::transmute::<*mut i32, &mut i32>(ptr)
    /// };
    ///
    /// // Use a reborrow instead
    /// let ref_casted = unsafe { &mut *ptr };
    /// ```
    ///
    /// Turning a `&mut T` into a `&mut U`:
    ///
    /// ```
    /// let ptr = &mut 0;
    /// let val_transmuted = unsafe {
    ///     std::mem::transmute::<&mut i32, &mut u32>(ptr)
    /// };
    ///
    /// // Now, put together `as` and reborrowing - note the chaining of `as`
    /// // `as` is not transitive
    /// let val_casts = unsafe { &mut *(ptr as *mut i32 as *mut u32) };
    /// ```
    ///
    /// Turning a `&str` into a `&[u8]`:
    ///
    /// ```
    /// // this is not a good way to do this.
    /// let slice = unsafe { std::mem::transmute::<&str, &[u8]>("Rust") };
    /// assert_eq!(slice, &[82, 117, 115, 116]);
    ///
    /// // You could use `str::as_bytes`
    /// let slice = "Rust".as_bytes();
    /// assert_eq!(slice, &[82, 117, 115, 116]);
    ///
    /// // Or, just use a byte string, if you have control over the string
    /// // literal
    /// assert_eq!(b"Rust", &[82, 117, 115, 116]);
    /// ```
    ///
    /// Turning a `Vec<&T>` into a `Vec<Option<&T>>`.
    ///
    /// To transmute the inner type of the contents of a container, you must make sure to not
    /// violate any of the container's invariants. For `Vec`, this means that both the size
    /// *and alignment* of the inner types have to match. Other containers might rely on the
    /// size of the type, alignment, or even the `TypeId`, in which case transmuting wouldn't
    /// be possible at all without violating the container invariants.
    ///
    /// ```
    /// let store = [0, 1, 2, 3];
    /// let v_orig = store.iter().collect::<Vec<&i32>>();
    ///
    /// // clone the vector as we will reuse them later
    /// let v_clone = v_orig.clone();
    ///
    /// // Using transmute: this relies on the unspecified data layout of `Vec`, which is a
    /// // bad idea and could cause Undefined Behavior.
    /// // However, it is no-copy.
    /// let v_transmuted = unsafe {
    ///     std::mem::transmute::<Vec<&i32>, Vec<Option<&i32>>>(v_clone)
    /// };
    ///
    /// let v_clone = v_orig.clone();
    ///
    /// // This is the suggested, safe way.
    /// // It may copy the entire vector into a new one though, but also may not.
    /// let v_collected = v_clone.into_iter()
    ///                          .map(Some)
    ///                          .collect::<Vec<Option<&i32>>>();
    ///
    /// let v_clone = v_orig.clone();
    ///
    /// // This is the proper no-copy, unsafe way of "transmuting" a `Vec`, without relying on the
    /// // data layout. Instead of literally calling `transmute`, we perform a pointer cast, but
    /// // in terms of converting the original inner type (`&i32`) to the new one (`Option<&i32>`),
    /// // this has all the same caveats. Besides the information provided above, also consult the
    /// // [`from_raw_parts`] documentation.
    /// let (ptr, len, capacity) = v_clone.into_raw_parts();
    /// let v_from_raw = unsafe {
    ///     Vec::from_raw_parts(ptr.cast::<*mut Option<&i32>>(), len, capacity)
    /// };
    /// ```
    ///
    /// [`from_raw_parts`]: ../../std/vec/struct.Vec.html#method.from_raw_parts
    ///
    /// Implementing `split_at_mut`:
    ///
    /// ```
    /// use std::{slice, mem};
    ///
    /// // There are multiple ways to do this, and there are multiple problems
    /// // with the following (transmute) way.
    /// fn split_at_mut_transmute<T>(slice: &mut [T], mid: usize)
    ///                              -> (&mut [T], &mut [T]) {
    ///     let len = slice.len();
    ///     assert!(mid <= len);
    ///     unsafe {
    ///         let slice2 = mem::transmute::<&mut [T], &mut [T]>(slice);
    ///         // first: transmute is not type safe; all it checks is that T and
    ///         // U are of the same size. Second, right here, you have two
    ///         // mutable references pointing to the same memory.
    ///         (&mut slice[0..mid], &mut slice2[mid..len])
    ///     }
    /// }
    ///
    /// // This gets rid of the type safety problems; `&mut *` will *only* give
    /// // you a `&mut T` from a `&mut T` or `*mut T`.
    /// fn split_at_mut_casts<T>(slice: &mut [T], mid: usize)
    ///                          -> (&mut [T], &mut [T]) {
    ///     let len = slice.len();
    ///     assert!(mid <= len);
    ///     unsafe {
    ///         let slice2 = &mut *(slice as *mut [T]);
    ///         // however, you still have two mutable references pointing to
    ///         // the same memory.
    ///         (&mut slice[0..mid], &mut slice2[mid..len])
    ///     }
    /// }
    ///
    /// // This is how the standard library does it. This is the best method, if
    /// // you need to do something like this
    /// fn split_at_stdlib<T>(to_split: &mut [T], mid: usize)
    ///                       -> (&mut [T], &mut [T]) {
    ///     let len = to_split.len();
    ///     assert!(mid <= len);
    ///     unsafe {
    ///         let ptr = to_split.as_mut_ptr();
    ///         let fst = slice::from_raw_parts_mut(ptr, mid);
    ///         let snd = slice::from_raw_parts_mut(ptr.add(mid), len - mid);
    ///         // The function now has three mutable references to overlapping memory:
    ///         // `to_split`, `fst`, and `snd`.
    ///         // `to_split` is never used after `let ptr = ...` so it can be treated as "dead".
    ///         // This leaves two "live" mutable slice references, `fst` and `snd`, with no overlap.
    ///         (fst, snd)
    ///     }
    /// }
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_allowed_through_unstable_modules = "import this function via `std::mem` instead"]
    #[rustc_const_stable(feature = "const_transmute", since = "1.56.0")]
    #[rustc_diagnostic_item = "transmute"]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn transmute<Src, Dst>(src: Src) -> Dst;

    /// Like [`transmute`], but even less checked at compile-time: rather than
    /// giving an error for `size_of::<Src>() != size_of::<Dst>()`, it's
    /// **Undefined Behavior** at runtime.
    ///
    /// Prefer normal `transmute` where possible, for the extra checking, since
    /// both do exactly the same thing at runtime, if they both compile.
    ///
    /// This is not expected to ever be exposed directly to users, rather it
    /// may eventually be exposed through some more-constrained API.
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn transmute_unchecked<Src, Dst>(src: Src) -> Dst;

    /// Returns `true` if the actual type given as `T` requires drop
    /// glue; returns `false` if the actual type provided for `T`
    /// implements `Copy`.
    ///
    /// If the actual type neither requires drop glue nor implements
    /// `Copy`, then the return value of this function is unspecified.
    ///
    /// Note that, unlike most intrinsics, this can only be called at compile-time
    /// as backends do not have an implementation for it. The only caller (its
    /// stable counterpart) wraps this intrinsic call in a `const` block so that
    /// backends only see an evaluated constant.
    ///
    /// The stabilized version of this intrinsic is [`mem::needs_drop`](crate::mem::needs_drop).
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn needs_drop<T: ?Sized>() -> bool;

    /// Calculates the offset from a pointer.
    ///
    /// This is implemented as an intrinsic to avoid converting to and from an
    /// integer, since the conversion would throw away aliasing information.
    ///
    /// This can only be used with `Ptr` as a raw pointer type (`*mut` or `*const`)
    /// to a `Sized` pointee and with `Delta` as `usize` or `isize`.  Any other
    /// instantiations may arbitrarily misbehave, and that's *not* a compiler bug.
    ///
    /// # Safety
    ///
    /// If the computed offset is non-zero, then both the starting and resulting pointer must be
    /// either in bounds or at the end of an allocation. If either pointer is out
    /// of bounds or arithmetic overflow occurs then this operation is undefined behavior.
    ///
    /// The stabilized version of this intrinsic is [`pointer::offset`].
    #[must_use = "returns a new pointer rather than modifying its argument"]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn offset<Ptr: bounds::BuiltinDeref, Delta>(dst: Ptr, offset: Delta) -> Ptr;

    /// Calculates the offset from a pointer, potentially wrapping.
    ///
    /// This is implemented as an intrinsic to avoid converting to and from an
    /// integer, since the conversion inhibits certain optimizations.
    ///
    /// # Safety
    ///
    /// Unlike the `offset` intrinsic, this intrinsic does not restrict the
    /// resulting pointer to point into or at the end of an allocated
    /// object, and it wraps with two's complement arithmetic. The resulting
    /// value is not necessarily valid to be used to actually access memory.
    ///
    /// The stabilized version of this intrinsic is [`pointer::wrapping_offset`].
    #[must_use = "returns a new pointer rather than modifying its argument"]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn arith_offset<T>(dst: *const T, offset: isize) -> *const T;

    /// Projects to the `index`-th element of `slice_ptr`, as the same kind of pointer
    /// as the slice was provided -- so `&mut [T] → &mut T`, `&[T] → &T`,
    /// `*mut [T] → *mut T`, or `*const [T] → *const T` -- without a bounds check.
    ///
    /// This is exposed via `<usize as SliceIndex>::get(_unchecked)(_mut)`,
    /// and isn't intended to be used elsewhere.
    ///
    /// Expands in MIR to `{&, &mut, &raw const, &raw mut} (*slice_ptr)[index]`,
    /// depending on the types involved, so no backend support is needed.
    ///
    /// # Safety
    ///
    /// - `index < PtrMetadata(slice_ptr)`, so the indexing is in-bounds for the slice
    /// - the resulting offsetting is in-bounds of the allocation, which is
    ///   always the case for references, but needs to be upheld manually for pointers
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn slice_get_unchecked<
        ItemPtr: bounds::ChangePointee<[T], Pointee = T, Output = SlicePtr>,
        SlicePtr,
        T,
    >(
        slice_ptr: SlicePtr,
        index: usize,
    ) -> ItemPtr;

    /// Masks out bits of the pointer according to a mask.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// Consider using [`pointer::mask`] instead.
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub fn ptr_mask<T>(ptr: *const T, mask: usize) -> *const T;

    /// Equivalent to the appropriate `llvm.memcpy.p0i8.0i8.*` intrinsic, with
    /// a size of `count` * `size_of::<T>()` and an alignment of `align_of::<T>()`.
    ///
    /// This intrinsic does not have a stable counterpart.
    /// # Safety
    ///
    /// The safety requirements are consistent with [`copy_nonoverlapping`]
    /// while the read and write behaviors are volatile,
    /// which means it will not be optimized out unless `_count` or `size_of::<T>()` is equal to zero.
    ///
    /// [`copy_nonoverlapping`]: ptr::copy_nonoverlapping
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn volatile_copy_nonoverlapping_memory<T>(dst: *mut T, src: *const T, count: usize);
    /// Equivalent to the appropriate `llvm.memmove.p0i8.0i8.*` intrinsic, with
    /// a size of `count * size_of::<T>()` and an alignment of `align_of::<T>()`.
    ///
    /// The volatile parameter is set to `true`, so it will not be optimized out
    /// unless size is equal to zero.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn volatile_copy_memory<T>(dst: *mut T, src: *const T, count: usize);
    /// Equivalent to the appropriate `llvm.memset.p0i8.*` intrinsic, with a
    /// size of `count * size_of::<T>()` and an alignment of `align_of::<T>()`.
    ///
    /// This intrinsic does not have a stable counterpart.
    /// # Safety
    ///
    /// The safety requirements are consistent with [`write_bytes`] while the write behavior is volatile,
    /// which means it will not be optimized out unless `_count` or `size_of::<T>()` is equal to zero.
    ///
    /// [`write_bytes`]: ptr::write_bytes
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn volatile_set_memory<T>(dst: *mut T, val: u8, count: usize);

    /// Performs a volatile load from the `src` pointer.
    ///
    /// The stabilized version of this intrinsic is [`core::ptr::read_volatile`].
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn volatile_load<T>(src: *const T) -> T;
    /// Performs a volatile store to the `dst` pointer.
    ///
    /// The stabilized version of this intrinsic is [`core::ptr::write_volatile`].
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn volatile_store<T>(dst: *mut T, val: T);

    /// Performs a volatile load from the `src` pointer
    /// The pointer is not required to be aligned.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic]
    #[rustc_nounwind]
    #[rustc_diagnostic_item = "intrinsics_unaligned_volatile_load"]
    pub unsafe fn unaligned_volatile_load<T>(src: *const T) -> T;
    /// Performs a volatile store to the `dst` pointer.
    /// The pointer is not required to be aligned.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic]
    #[rustc_nounwind]
    #[rustc_diagnostic_item = "intrinsics_unaligned_volatile_store"]
    pub unsafe fn unaligned_volatile_store<T>(dst: *mut T, val: T);

    /// Returns the square root of an `f16`
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::sqrt`](../../std/primitive.f16.html#method.sqrt)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn sqrtf16(x: f16) -> f16;
    /// Returns the square root of an `f32`
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::sqrt`](../../std/primitive.f32.html#method.sqrt)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn sqrtf32(x: f32) -> f32;
    /// Returns the square root of an `f64`
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::sqrt`](../../std/primitive.f64.html#method.sqrt)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn sqrtf64(x: f64) -> f64;
    /// Returns the square root of an `f128`
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::sqrt`](../../std/primitive.f128.html#method.sqrt)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn sqrtf128(x: f128) -> f128;

    /// Raises an `f16` to an integer power.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::powi`](../../std/primitive.f16.html#method.powi)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn powif16(a: f16, x: i32) -> f16;
    /// Raises an `f32` to an integer power.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::powi`](../../std/primitive.f32.html#method.powi)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn powif32(a: f32, x: i32) -> f32;
    /// Raises an `f64` to an integer power.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::powi`](../../std/primitive.f64.html#method.powi)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn powif64(a: f64, x: i32) -> f64;
    /// Raises an `f128` to an integer power.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::powi`](../../std/primitive.f128.html#method.powi)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn powif128(a: f128, x: i32) -> f128;

    /// Returns the sine of an `f16`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::sin`](../../std/primitive.f16.html#method.sin)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn sinf16(x: f16) -> f16;
    /// Returns the sine of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::sin`](../../std/primitive.f32.html#method.sin)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn sinf32(x: f32) -> f32;
    /// Returns the sine of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::sin`](../../std/primitive.f64.html#method.sin)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn sinf64(x: f64) -> f64;
    /// Returns the sine of an `f128`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::sin`](../../std/primitive.f128.html#method.sin)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn sinf128(x: f128) -> f128;

    /// Returns the cosine of an `f16`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::cos`](../../std/primitive.f16.html#method.cos)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn cosf16(x: f16) -> f16;
    /// Returns the cosine of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::cos`](../../std/primitive.f32.html#method.cos)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn cosf32(x: f32) -> f32;
    /// Returns the cosine of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::cos`](../../std/primitive.f64.html#method.cos)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn cosf64(x: f64) -> f64;
    /// Returns the cosine of an `f128`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::cos`](../../std/primitive.f128.html#method.cos)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn cosf128(x: f128) -> f128;

    /// Raises an `f16` to an `f16` power.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::powf`](../../std/primitive.f16.html#method.powf)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn powf16(a: f16, x: f16) -> f16;
    /// Raises an `f32` to an `f32` power.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::powf`](../../std/primitive.f32.html#method.powf)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn powf32(a: f32, x: f32) -> f32;
    /// Raises an `f64` to an `f64` power.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::powf`](../../std/primitive.f64.html#method.powf)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn powf64(a: f64, x: f64) -> f64;
    /// Raises an `f128` to an `f128` power.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::powf`](../../std/primitive.f128.html#method.powf)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn powf128(a: f128, x: f128) -> f128;

    /// Returns the exponential of an `f16`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::exp`](../../std/primitive.f16.html#method.exp)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn expf16(x: f16) -> f16;
    /// Returns the exponential of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::exp`](../../std/primitive.f32.html#method.exp)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn expf32(x: f32) -> f32;
    /// Returns the exponential of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::exp`](../../std/primitive.f64.html#method.exp)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn expf64(x: f64) -> f64;
    /// Returns the exponential of an `f128`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::exp`](../../std/primitive.f128.html#method.exp)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn expf128(x: f128) -> f128;

    /// Returns 2 raised to the power of an `f16`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::exp2`](../../std/primitive.f16.html#method.exp2)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn exp2f16(x: f16) -> f16;
    /// Returns 2 raised to the power of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::exp2`](../../std/primitive.f32.html#method.exp2)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn exp2f32(x: f32) -> f32;
    /// Returns 2 raised to the power of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::exp2`](../../std/primitive.f64.html#method.exp2)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn exp2f64(x: f64) -> f64;
    /// Returns 2 raised to the power of an `f128`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::exp2`](../../std/primitive.f128.html#method.exp2)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn exp2f128(x: f128) -> f128;

    /// Returns the natural logarithm of an `f16`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::ln`](../../std/primitive.f16.html#method.ln)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn logf16(x: f16) -> f16;
    /// Returns the natural logarithm of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::ln`](../../std/primitive.f32.html#method.ln)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn logf32(x: f32) -> f32;
    /// Returns the natural logarithm of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::ln`](../../std/primitive.f64.html#method.ln)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn logf64(x: f64) -> f64;
    /// Returns the natural logarithm of an `f128`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::ln`](../../std/primitive.f128.html#method.ln)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn logf128(x: f128) -> f128;

    /// Returns the base 10 logarithm of an `f16`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::log10`](../../std/primitive.f16.html#method.log10)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn log10f16(x: f16) -> f16;
    /// Returns the base 10 logarithm of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::log10`](../../std/primitive.f32.html#method.log10)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn log10f32(x: f32) -> f32;
    /// Returns the base 10 logarithm of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::log10`](../../std/primitive.f64.html#method.log10)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn log10f64(x: f64) -> f64;
    /// Returns the base 10 logarithm of an `f128`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::log10`](../../std/primitive.f128.html#method.log10)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn log10f128(x: f128) -> f128;

    /// Returns the base 2 logarithm of an `f16`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::log2`](../../std/primitive.f16.html#method.log2)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn log2f16(x: f16) -> f16;
    /// Returns the base 2 logarithm of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::log2`](../../std/primitive.f32.html#method.log2)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn log2f32(x: f32) -> f32;
    /// Returns the base 2 logarithm of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::log2`](../../std/primitive.f64.html#method.log2)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn log2f64(x: f64) -> f64;
    /// Returns the base 2 logarithm of an `f128`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::log2`](../../std/primitive.f128.html#method.log2)
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn log2f128(x: f128) -> f128;

    /// Returns `a * b + c` for `f16` values.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::mul_add`](../../std/primitive.f16.html#method.mul_add)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn fmaf16(a: f16, b: f16, c: f16) -> f16;
    /// Returns `a * b + c` for `f32` values.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::mul_add`](../../std/primitive.f32.html#method.mul_add)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn fmaf32(a: f32, b: f32, c: f32) -> f32;
    /// Returns `a * b + c` for `f64` values.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::mul_add`](../../std/primitive.f64.html#method.mul_add)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn fmaf64(a: f64, b: f64, c: f64) -> f64;
    /// Returns `a * b + c` for `f128` values.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::mul_add`](../../std/primitive.f128.html#method.mul_add)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn fmaf128(a: f128, b: f128, c: f128) -> f128;

    /// Returns `a * b + c` for `f16` values, non-deterministically executing
    /// either a fused multiply-add or two operations with rounding of the
    /// intermediate result.
    ///
    /// The operation is fused if the code generator determines that target
    /// instruction set has support for a fused operation, and that the fused
    /// operation is more efficient than the equivalent, separate pair of mul
    /// and add instructions. It is unspecified whether or not a fused operation
    /// is selected, and that may depend on optimization level and context, for
    /// example.
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn fmuladdf16(a: f16, b: f16, c: f16) -> f16;
    /// Returns `a * b + c` for `f32` values, non-deterministically executing
    /// either a fused multiply-add or two operations with rounding of the
    /// intermediate result.
    ///
    /// The operation is fused if the code generator determines that target
    /// instruction set has support for a fused operation, and that the fused
    /// operation is more efficient than the equivalent, separate pair of mul
    /// and add instructions. It is unspecified whether or not a fused operation
    /// is selected, and that may depend on optimization level and context, for
    /// example.
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn fmuladdf32(a: f32, b: f32, c: f32) -> f32;
    /// Returns `a * b + c` for `f64` values, non-deterministically executing
    /// either a fused multiply-add or two operations with rounding of the
    /// intermediate result.
    ///
    /// The operation is fused if the code generator determines that target
    /// instruction set has support for a fused operation, and that the fused
    /// operation is more efficient than the equivalent, separate pair of mul
    /// and add instructions. It is unspecified whether or not a fused operation
    /// is selected, and that may depend on optimization level and context, for
    /// example.
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn fmuladdf64(a: f64, b: f64, c: f64) -> f64;
    /// Returns `a * b + c` for `f128` values, non-deterministically executing
    /// either a fused multiply-add or two operations with rounding of the
    /// intermediate result.
    ///
    /// The operation is fused if the code generator determines that target
    /// instruction set has support for a fused operation, and that the fused
    /// operation is more efficient than the equivalent, separate pair of mul
    /// and add instructions. It is unspecified whether or not a fused operation
    /// is selected, and that may depend on optimization level and context, for
    /// example.
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn fmuladdf128(a: f128, b: f128, c: f128) -> f128;

    /// Returns the largest integer less than or equal to an `f16`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::floor`](../../std/primitive.f16.html#method.floor)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn floorf16(x: f16) -> f16;
    /// Returns the largest integer less than or equal to an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::floor`](../../std/primitive.f32.html#method.floor)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn floorf32(x: f32) -> f32;
    /// Returns the largest integer less than or equal to an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::floor`](../../std/primitive.f64.html#method.floor)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn floorf64(x: f64) -> f64;
    /// Returns the largest integer less than or equal to an `f128`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::floor`](../../std/primitive.f128.html#method.floor)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn floorf128(x: f128) -> f128;

    /// Returns the smallest integer greater than or equal to an `f16`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::ceil`](../../std/primitive.f16.html#method.ceil)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn ceilf16(x: f16) -> f16;
    /// Returns the smallest integer greater than or equal to an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::ceil`](../../std/primitive.f32.html#method.ceil)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn ceilf32(x: f32) -> f32;
    /// Returns the smallest integer greater than or equal to an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::ceil`](../../std/primitive.f64.html#method.ceil)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn ceilf64(x: f64) -> f64;
    /// Returns the smallest integer greater than or equal to an `f128`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::ceil`](../../std/primitive.f128.html#method.ceil)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn ceilf128(x: f128) -> f128;

    /// Returns the integer part of an `f16`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::trunc`](../../std/primitive.f16.html#method.trunc)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn truncf16(x: f16) -> f16;
    /// Returns the integer part of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::trunc`](../../std/primitive.f32.html#method.trunc)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn truncf32(x: f32) -> f32;
    /// Returns the integer part of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::trunc`](../../std/primitive.f64.html#method.trunc)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn truncf64(x: f64) -> f64;
    /// Returns the integer part of an `f128`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::trunc`](../../std/primitive.f128.html#method.trunc)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn truncf128(x: f128) -> f128;

    /// Returns the nearest integer to an `f16`. Rounds half-way cases to the number with an even
    /// least significant digit.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::round_ties_even`](../../std/primitive.f16.html#method.round_ties_even)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn round_ties_even_f16(x: f16) -> f16;

    /// Returns the nearest integer to an `f32`. Rounds half-way cases to the number with an even
    /// least significant digit.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::round_ties_even`](../../std/primitive.f32.html#method.round_ties_even)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn round_ties_even_f32(x: f32) -> f32;

    /// Returns the nearest integer to an `f64`. Rounds half-way cases to the number with an even
    /// least significant digit.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::round_ties_even`](../../std/primitive.f64.html#method.round_ties_even)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn round_ties_even_f64(x: f64) -> f64;

    /// Returns the nearest integer to an `f128`. Rounds half-way cases to the number with an even
    /// least significant digit.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::round_ties_even`](../../std/primitive.f128.html#method.round_ties_even)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn round_ties_even_f128(x: f128) -> f128;

    /// Returns the nearest integer to an `f16`. Rounds half-way cases away from zero.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::round`](../../std/primitive.f16.html#method.round)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn roundf16(x: f16) -> f16;
    /// Returns the nearest integer to an `f32`. Rounds half-way cases away from zero.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::round`](../../std/primitive.f32.html#method.round)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn roundf32(x: f32) -> f32;
    /// Returns the nearest integer to an `f64`. Rounds half-way cases away from zero.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::round`](../../std/primitive.f64.html#method.round)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn roundf64(x: f64) -> f64;
    /// Returns the nearest integer to an `f128`. Rounds half-way cases away from zero.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::round`](../../std/primitive.f128.html#method.round)
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn roundf128(x: f128) -> f128;

    /// Float addition that allows optimizations based on algebraic rules.
    /// Requires that inputs and output of the operation are finite, causing UB otherwise.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn fadd_fast<T: bounds::FloatPrimitive>(a: T, b: T) -> T;
    /// Float subtraction that allows optimizations based on algebraic rules.
    /// Requires that inputs and output of the operation are finite, causing UB otherwise.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn fsub_fast<T: bounds::FloatPrimitive>(a: T, b: T) -> T;
    /// Float multiplication that allows optimizations based on algebraic rules.
    /// Requires that inputs and output of the operation are finite, causing UB otherwise.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn fmul_fast<T: bounds::FloatPrimitive>(a: T, b: T) -> T;
    /// Float division that allows optimizations based on algebraic rules.
    /// Requires that inputs and output of the operation are finite, causing UB otherwise.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn fdiv_fast<T: bounds::FloatPrimitive>(a: T, b: T) -> T;
    /// Float remainder that allows optimizations based on algebraic rules.
    /// Requires that inputs and output of the operation are finite, causing UB otherwise.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn frem_fast<T: bounds::FloatPrimitive>(a: T, b: T) -> T;
    /// Converts with LLVM’s fptoui/fptosi, which may return undef for values out of range
    /// (<https://github.com/rust-lang/rust/issues/10184>)
    ///
    /// Stabilized as [`f32::to_int_unchecked`] and [`f64::to_int_unchecked`].
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn float_to_int_unchecked<Float: bounds::FloatPrimitive, Int: Copy>(value: Float)
    -> Int;

    /// Float addition that allows optimizations based on algebraic rules.
    ///
    /// Stabilized as [`f16::algebraic_add`], [`f32::algebraic_add`], [`f64::algebraic_add`] and [`f128::algebraic_add`].
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn fadd_algebraic<T: bounds::FloatPrimitive>(a: T, b: T) -> T;
    /// Float subtraction that allows optimizations based on algebraic rules.
    ///
    /// Stabilized as [`f16::algebraic_sub`], [`f32::algebraic_sub`], [`f64::algebraic_sub`] and [`f128::algebraic_sub`].
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn fsub_algebraic<T: bounds::FloatPrimitive>(a: T, b: T) -> T;
    /// Float multiplication that allows optimizations based on algebraic rules.
    ///
    /// Stabilized as [`f16::algebraic_mul`], [`f32::algebraic_mul`], [`f64::algebraic_mul`] and [`f128::algebraic_mul`].
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn fmul_algebraic<T: bounds::FloatPrimitive>(a: T, b: T) -> T;
    /// Float division that allows optimizations based on algebraic rules.
    ///
    /// Stabilized as [`f16::algebraic_div`], [`f32::algebraic_div`], [`f64::algebraic_div`] and [`f128::algebraic_div`].
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn fdiv_algebraic<T: bounds::FloatPrimitive>(a: T, b: T) -> T;
    /// Float remainder that allows optimizations based on algebraic rules.
    ///
    /// Stabilized as [`f16::algebraic_rem`], [`f32::algebraic_rem`], [`f64::algebraic_rem`] and [`f128::algebraic_rem`].
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn frem_algebraic<T: bounds::FloatPrimitive>(a: T, b: T) -> T;
    /// Returns the number of bits set in an integer type `T`
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `count_ones` method. For example,
    /// [`u32::count_ones`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn ctpop<T: Copy>(x: T) -> u32;

    /// Returns the number of leading unset bits (zeroes) in an integer type `T`.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `leading_zeros` method. For example,
    /// [`u32::leading_zeros`]
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    ///
    /// use std::intrinsics::ctlz;
    ///
    /// let x = 0b0001_1100_u8;
    /// let num_leading = ctlz(x);
    /// assert_eq!(num_leading, 3);
    /// ```
    ///
    /// An `x` with value `0` will return the bit width of `T`.
    ///
    /// ```
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    ///
    /// use std::intrinsics::ctlz;
    ///
    /// let x = 0u16;
    /// let num_leading = ctlz(x);
    /// assert_eq!(num_leading, 16);
    /// ```
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn ctlz<T: Copy>(x: T) -> u32;

    /// Like `ctlz`, but extra-unsafe as it returns `undef` when
    /// given an `x` with value `0`.
    ///
    /// This intrinsic does not have a stable counterpart.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    ///
    /// use std::intrinsics::ctlz_nonzero;
    ///
    /// let x = 0b0001_1100_u8;
    /// let num_leading = unsafe { ctlz_nonzero(x) };
    /// assert_eq!(num_leading, 3);
    /// ```
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn ctlz_nonzero<T: Copy>(x: T) -> u32;

    /// Returns the number of trailing unset bits (zeroes) in an integer type `T`.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `trailing_zeros` method. For example,
    /// [`u32::trailing_zeros`]
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    ///
    /// use std::intrinsics::cttz;
    ///
    /// let x = 0b0011_1000_u8;
    /// let num_trailing = cttz(x);
    /// assert_eq!(num_trailing, 3);
    /// ```
    ///
    /// An `x` with value `0` will return the bit width of `T`:
    ///
    /// ```
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    ///
    /// use std::intrinsics::cttz;
    ///
    /// let x = 0u16;
    /// let num_trailing = cttz(x);
    /// assert_eq!(num_trailing, 16);
    /// ```
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn cttz<T: Copy>(x: T) -> u32;

    /// Like `cttz`, but extra-unsafe as it returns `undef` when
    /// given an `x` with value `0`.
    ///
    /// This intrinsic does not have a stable counterpart.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    ///
    /// use std::intrinsics::cttz_nonzero;
    ///
    /// let x = 0b0011_1000_u8;
    /// let num_trailing = unsafe { cttz_nonzero(x) };
    /// assert_eq!(num_trailing, 3);
    /// ```
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn cttz_nonzero<T: Copy>(x: T) -> u32;

    /// Reverses the bytes in an integer type `T`.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `swap_bytes` method. For example,
    /// [`u32::swap_bytes`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn bswap<T: Copy>(x: T) -> T;
    /// Reverses the bits in an integer type `T`.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `reverse_bits` method. For example,
    /// [`u32::reverse_bits`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn bitreverse<T: Copy>(x: T) -> T;
    /// Does a three-way comparison between the two arguments,
    /// which must be of character or integer (signed or unsigned) type.
    ///
    /// This was originally added because it greatly simplified the MIR in `cmp`
    /// implementations, and then LLVM 20 added a backend intrinsic for it too.
    ///
    /// The stabilized version of this intrinsic is [`Ord::cmp`].
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn three_way_compare<T: Copy>(lhs: T, rhss: T) -> crate::cmp::Ordering;

    /// Combine two values which have no bits in common.
    ///
    /// This allows the backend to implement it as `a + b` *or* `a | b`,
    /// depending which is easier to implement on a specific target.
    ///
    /// # Safety
    ///
    /// Requires that `(a & b) == 0`, or equivalently that `(a | b) == (a + b)`.
    ///
    /// Otherwise it's immediate UB.
    #[rustc_const_unstable(feature = "disjoint_bitor", issue = "135758")]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    #[track_caller]
    #[miri::intrinsic_fallback_is_spec] // the fallbacks all `assume` to tell Miri
    pub const unsafe fn disjoint_bitor<T: [const] fallback::DisjointBitOr>(a: T, b: T) -> T {
        // SAFETY: same preconditions as this function.
        unsafe { fallback::DisjointBitOr::disjoint_bitor(a, b) }
    }

    /// Performs checked integer addition.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `overflowing_add` method. For example,
    /// [`u32::overflowing_add`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn add_with_overflow<T: Copy>(x: T, y: T) -> (T, bool);

    /// Performs checked integer subtraction
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `overflowing_sub` method. For example,
    /// [`u32::overflowing_sub`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn sub_with_overflow<T: Copy>(x: T, y: T) -> (T, bool);

    /// Performs checked integer multiplication
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `overflowing_mul` method. For example,
    /// [`u32::overflowing_mul`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn mul_with_overflow<T: Copy>(x: T, y: T) -> (T, bool);

    /// Performs full-width multiplication and addition with a carry:
    /// `multiplier * multiplicand + addend + carry`.
    ///
    /// This is possible without any overflow.  For `uN`:
    ///    MAX * MAX + MAX + MAX
    /// => (2ⁿ-1) × (2ⁿ-1) + (2ⁿ-1) + (2ⁿ-1)
    /// => (2²ⁿ - 2ⁿ⁺¹ + 1) + (2ⁿ⁺¹ - 2)
    /// => 2²ⁿ - 1
    ///
    /// For `iN`, the upper bound is MIN * MIN + MAX + MAX => 2²ⁿ⁻² + 2ⁿ - 2,
    /// and the lower bound is MAX * MIN + MIN + MIN => -2²ⁿ⁻² - 2ⁿ + 2ⁿ⁺¹.
    ///
    /// This currently supports unsigned integers *only*, no signed ones.
    /// The stabilized versions of this intrinsic are available on integers.
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_const_unstable(feature = "const_carrying_mul_add", issue = "85532")]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    #[miri::intrinsic_fallback_is_spec]
    pub const fn carrying_mul_add<T: [const] fallback::CarryingMulAdd<Unsigned = U>, U>(
        multiplier: T,
        multiplicand: T,
        addend: T,
        carry: T,
    ) -> (U, T) {
        multiplier.carrying_mul_add(multiplicand, addend, carry)
    }

    /// Performs an exact division, resulting in undefined behavior where
    /// `x % y != 0` or `y == 0` or `x == T::MIN && y == -1`
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn exact_div<T: Copy>(x: T, y: T) -> T;
    /// Performs an unchecked division, resulting in undefined behavior
    /// where `y == 0` or `x == T::MIN && y == -1`
    ///
    /// Safe wrappers for this intrinsic are available on the integer
    /// primitives via the `checked_div` method. For example,
    /// [`u32::checked_div`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn unchecked_div<T: Copy>(x: T, y: T) -> T;
    /// Returns the remainder of an unchecked division, resulting in
    /// undefined behavior when `y == 0` or `x == T::MIN && y == -1`
    ///
    /// Safe wrappers for this intrinsic are available on the integer
    /// primitives via the `checked_rem` method. For example,
    /// [`u32::checked_rem`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn unchecked_rem<T: Copy>(x: T, y: T) -> T;
    /// Performs an unchecked left shift, resulting in undefined behavior when
    /// `y < 0` or `y >= N`, where N is the width of T in bits.
    ///
    /// Safe wrappers for this intrinsic are available on the integer
    /// primitives via the `checked_shl` method. For example,
    /// [`u32::checked_shl`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn unchecked_shl<T: Copy, U: Copy>(x: T, y: U) -> T;
    /// Performs an unchecked right shift, resulting in undefined behavior when
    /// `y < 0` or `y >= N`, where N is the width of T in bits.
    ///
    /// Safe wrappers for this intrinsic are available on the integer
    /// primitives via the `checked_shr` method. For example,
    /// [`u32::checked_shr`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn unchecked_shr<T: Copy, U: Copy>(x: T, y: U) -> T;
    /// Returns the result of an unchecked addition, resulting in
    /// undefined behavior when `x + y > T::MAX` or `x + y < T::MIN`.
    ///
    /// The stable counterpart of this intrinsic is `unchecked_add` on the various
    /// integer types, such as [`u16::unchecked_add`] and [`i64::unchecked_add`].
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn unchecked_add<T: Copy>(x: T, y: T) -> T;
    /// Returns the result of an unchecked subtraction, resulting in
    /// undefined behavior when `x - y > T::MAX` or `x - y < T::MIN`.
    ///
    /// The stable counterpart of this intrinsic is `unchecked_sub` on the various
    /// integer types, such as [`u16::unchecked_sub`] and [`i64::unchecked_sub`].
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn unchecked_sub<T: Copy>(x: T, y: T) -> T;
    /// Returns the result of an unchecked multiplication, resulting in
    /// undefined behavior when `x * y > T::MAX` or `x * y < T::MIN`.
    ///
    /// The stable counterpart of this intrinsic is `unchecked_mul` on the various
    /// integer types, such as [`u16::unchecked_mul`] and [`i64::unchecked_mul`].
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn unchecked_mul<T: Copy>(x: T, y: T) -> T;
    /// Performs rotate left.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `rotate_left` method. For example,
    /// [`u32::rotate_left`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    #[rustc_allow_const_fn_unstable(const_trait_impl, funnel_shifts)]
    #[miri::intrinsic_fallback_is_spec]
    pub const fn rotate_left<T: [const] fallback::FunnelShift>(x: T, shift: u32) -> T {
        // Make sure to call the intrinsic for `funnel_shl`, not the fallback impl.
        // SAFETY: we modulo `shift` so that the result is definitely less than the size of
        // `T` in bits.
        unsafe { unchecked_funnel_shl(x, x, shift % (mem::size_of::<T>() as u32 * 8)) }
    }

    /// Performs rotate right.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `rotate_right` method. For example,
    /// [`u32::rotate_right`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    #[rustc_allow_const_fn_unstable(const_trait_impl, funnel_shifts)]
    #[miri::intrinsic_fallback_is_spec]
    pub const fn rotate_right<T: [const] fallback::FunnelShift>(x: T, shift: u32) -> T {
        // Make sure to call the intrinsic for `funnel_shr`, not the fallback impl.
        // SAFETY: we modulo `shift` so that the result is definitely less than the size of
        // `T` in bits.
        unsafe { unchecked_funnel_shr(x, x, shift % (mem::size_of::<T>() as u32 * 8)) }
    }

    /// Wrapping (modular) addition. Computes `a + b`,
    /// wrapping around at the boundary of the type.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `wrapping_add` method. For example,
    /// [`u32::wrapping_add`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn wrapping_add<T: Copy>(a: T, b: T) -> T;
    /// Wrapping (modular) subtraction. Computes `a - b`,
    /// wrapping around at the boundary of the type.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `wrapping_sub` method. For example,
    /// [`u32::wrapping_sub`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn wrapping_sub<T: Copy>(a: T, b: T) -> T;
    /// Wrapping (modular) multiplication. Computes `a *
    /// b`, wrapping around at the boundary of the type.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `wrapping_mul` method. For example,
    /// [`u32::wrapping_mul`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn wrapping_mul<T: Copy>(a: T, b: T) -> T;
    /// Computes `a + b`, saturating at numeric bounds.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `saturating_add` method. For example,
    /// [`u32::saturating_add`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn saturating_add<T: Copy>(a: T, b: T) -> T;
    /// Computes `a - b`, saturating at numeric bounds.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `saturating_sub` method. For example,
    /// [`u32::saturating_sub`]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn saturating_sub<T: Copy>(a: T, b: T) -> T;
    /// Funnel Shift left.
    ///
    /// Concatenates `a` and `b` (with `a` in the most significant half),
    /// creating an integer twice as wide. Then shift this integer left
    /// by `shift`), and extract the most significant half. If `a` and `b`
    /// are the same, this is equivalent to a rotate left operation.
    ///
    /// It is undefined behavior if `shift` is greater than or equal to the
    /// bit size of `T`.
    ///
    /// Safe versions of this intrinsic are available on the integer primitives
    /// via the `funnel_shl` method. For example, [`u32::funnel_shl`].
    #[rustc_intrinsic]
    #[rustc_nounwind]
    #[rustc_const_unstable(feature = "funnel_shifts", issue = "145686")]
    #[unstable(feature = "funnel_shifts", issue = "145686")]
    #[track_caller]
    #[miri::intrinsic_fallback_is_spec]
    pub const unsafe fn unchecked_funnel_shl<T: [const] fallback::FunnelShift>(
        a: T,
        b: T,
        shift: u32,
    ) -> T {
        // SAFETY: caller ensures that `shift` is in-range
        unsafe { a.unchecked_funnel_shl(b, shift) }
    }

    /// Funnel Shift right.
    ///
    /// Concatenates `a` and `b` (with `a` in the most significant half),
    /// creating an integer twice as wide. Then shift this integer right
    /// by `shift` (taken modulo the bit size of `T`), and extract the
    /// least significant half. If `a` and `b` are the same, this is equivalent
    /// to a rotate right operation.
    ///
    /// It is undefined behavior if `shift` is greater than or equal to the
    /// bit size of `T`.
    ///
    /// Safer versions of this intrinsic are available on the integer primitives
    /// via the `funnel_shr` method. For example, [`u32::funnel_shr`]
    #[rustc_intrinsic]
    #[rustc_nounwind]
    #[rustc_const_unstable(feature = "funnel_shifts", issue = "145686")]
    #[unstable(feature = "funnel_shifts", issue = "145686")]
    #[track_caller]
    #[miri::intrinsic_fallback_is_spec]
    pub const unsafe fn unchecked_funnel_shr<T: [const] fallback::FunnelShift>(
        a: T,
        b: T,
        shift: u32,
    ) -> T {
        // SAFETY: caller ensures that `shift` is in-range
        unsafe { a.unchecked_funnel_shr(b, shift) }
    }

    /// Carryless multiply.
    ///
    /// Safe versions of this intrinsic are available on the integer primitives
    /// via the `carryless_mul` method. For example, [`u32::carryless_mul`].
    #[rustc_intrinsic]
    #[rustc_nounwind]
    #[rustc_const_unstable(feature = "uint_carryless_mul", issue = "152080")]
    #[unstable(feature = "uint_carryless_mul", issue = "152080")]
    #[miri::intrinsic_fallback_is_spec]
    pub const fn carryless_mul<T: [const] fallback::CarrylessMul>(a: T, b: T) -> T {
        a.carryless_mul(b)
    }

    /// This is an implementation detail of [`crate::ptr::read`] and should
    /// not be used anywhere else.  See its comments for why this exists.
    ///
    /// This intrinsic can *only* be called where the pointer is a local without
    /// projections (`read_via_copy(ptr)`, not `read_via_copy(*ptr)`) so that it
    /// trivially obeys runtime-MIR rules about derefs in operands.
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn read_via_copy<T>(ptr: *const T) -> T;
    /// This is an implementation detail of [`crate::ptr::write`] and should
    /// not be used anywhere else.  See its comments for why this exists.
    ///
    /// This intrinsic can *only* be called where the pointer is a local without
    /// projections (`write_via_move(ptr, x)`, not `write_via_move(*ptr, x)`) so
    /// that it trivially obeys runtime-MIR rules about derefs in operands.
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn write_via_move<T>(ptr: *mut T, value: T);

    /// Returns the value of the discriminant for the variant in 'v';
    /// if `T` has no discriminant, returns `0`.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is [`core::mem::discriminant`].
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn discriminant_value<T>(v: &T) -> <T as DiscriminantKind>::Discriminant;

    /// Rust's "try catch" construct for unwinding. Invokes the function pointer `try_fn` with the
    /// data pointer `data`, and calls `catch_fn` if unwinding occurs while `try_fn` runs.
    /// Returns `true` if unwinding occurred and `catch_fn` was called; returns `false` otherwise.
    ///
    /// `catch_fn` must not unwind.
    ///
    /// The third argument is a function called if an unwind occurs (both Rust `panic` and foreign
    /// unwinds). This function takes the data pointer and a pointer to the target- and
    /// runtime-specific exception object that was caught.
    ///
    /// Note that in the case of a foreign unwinding operation, the exception object data may not be
    /// safely usable from Rust, and should not be directly exposed via the standard library. To
    /// prevent unsafe access, the library implementation may either abort the process or present an
    /// opaque error type to the user.
    ///
    /// For more information, see the compiler's source, as well as the documentation for the stable
    /// version of this intrinsic, `std::panic::catch_unwind`.
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn catch_unwind<Data: ptr::Thin>(
        _try_fn: unsafe fn(*mut Data),
        _data: *mut Data,
        _catch_fn: unsafe fn(*mut Data, *mut u8),
    ) -> bool;

    /// Emits a `nontemporal` store, which gives a hint to the CPU that the data should not be held
    /// in cache. Except for performance, this is fully equivalent to `ptr.write(val)`.
    ///
    /// Not all architectures provide such an operation. For instance, x86 does not: while `MOVNT`
    /// exists, that operation is *not* equivalent to `ptr.write(val)` (`MOVNT` writes can be reordered
    /// in ways that are not allowed for regular writes).
    #[rustc_intrinsic] #[rustc_nounwind] pub unsafe fn nontemporal_store<T>(ptr: *mut T, val: T);

    /// See documentation of `<*const T>::offset_from` for details.
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn ptr_offset_from<T>(ptr: *const T, base: *const T) -> isize;

    /// See documentation of `<*const T>::offset_from_unsigned` for details.
    #[rustc_nounwind]
    #[rustc_intrinsic]
    #[rustc_intrinsic_const_stable_indirect]
    pub const unsafe fn ptr_offset_from_unsigned<T>(ptr: *const T, base: *const T) -> usize;

    /// See documentation of `<*const T>::guaranteed_eq` for details.
    /// Returns `2` if the result is unknown.
    /// Returns `1` if the pointers are guaranteed equal.
    /// Returns `0` if the pointers are guaranteed inequal.
    #[rustc_intrinsic]
    #[rustc_nounwind]
    #[rustc_do_not_const_check]
    #[inline]
    #[miri::intrinsic_fallback_is_spec]
    pub const fn ptr_guaranteed_cmp<T>(ptr: *const T, other: *const T) -> u8 {
        (ptr == other) as u8
    }

    /// Determines whether the raw bytes of the two values are equal.
    ///
    /// This is particularly handy for arrays, since it allows things like just
    /// comparing `i96`s instead of forcing `alloca`s for `[6 x i16]`.
    ///
    /// Above some backend-decided threshold this will emit calls to `memcmp`,
    /// like slice equality does, instead of causing massive code size.
    ///
    /// Since this works by comparing the underlying bytes, the actual `T` is
    /// not particularly important.  It will be used for its size and alignment,
    /// but any validity restrictions will be ignored, not enforced.
    ///
    /// # Safety
    ///
    /// It's UB to call this if any of the *bytes* in `*a` or `*b` are uninitialized.
    /// Note that this is a stricter criterion than just the *values* being
    /// fully-initialized: if `T` has padding, it's UB to call this intrinsic.
    ///
    /// At compile-time, it is furthermore UB to call this if any of the bytes
    /// in `*a` or `*b` have provenance.
    ///
    /// (The implementation is allowed to branch on the results of comparisons,
    /// which is UB if any of their inputs are `undef`.)
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn raw_eq<T>(a: &T, b: &T) -> bool;

    /// Lexicographically compare `[left, left + bytes)` and `[right, right + bytes)`
    /// as unsigned bytes, returning negative if `left` is less, zero if all the
    /// bytes match, or positive if `left` is greater.
    ///
    /// This underlies things like `<[u8]>::cmp`, and will usually lower to `memcmp`.
    ///
    /// # Safety
    ///
    /// `left` and `right` must each be [valid] for reads of `bytes` bytes.
    ///
    /// Note that this applies to the whole range, not just until the first byte
    /// that differs.  That allows optimizations that can read in large chunks.
    ///
    /// [valid]: crate::ptr#safety
    #[rustc_nounwind]
    #[rustc_intrinsic]
    #[rustc_const_unstable(feature = "const_cmp", issue = "143800")]
    pub const unsafe fn compare_bytes(left: *const u8, right: *const u8, bytes: usize) -> i32;

    /// See documentation of [`std::hint::black_box`] for details.
    ///
    /// [`std::hint::black_box`]: crate::hint::black_box
    #[rustc_nounwind]
    #[rustc_intrinsic]
    #[rustc_intrinsic_const_stable_indirect]
    pub const fn black_box<T>(dummy: T) -> T;
    /// Selects which function to call depending on the context.
    ///
    /// If this function is evaluated at compile-time, then a call to this
    /// intrinsic will be replaced with a call to `called_in_const`. It gets
    /// replaced with a call to `called_at_rt` otherwise.
    ///
    /// This function is safe to call, but note the stability concerns below.
    ///
    /// # Type Requirements
    ///
    /// The two functions must be both function items. They cannot be function
    /// pointers or closures. The first function must be a `const fn`.
    ///
    /// `arg` will be the tupled arguments that will be passed to either one of
    /// the two functions, therefore, both functions must accept the same type of
    /// arguments. Both functions must return RET.
    ///
    /// # Stability concerns
    ///
    /// Rust has not yet decided that `const fn` are allowed to tell whether
    /// they run at compile-time or at runtime. Therefore, when using this
    /// intrinsic anywhere that can be reached from stable, it is crucial that
    /// the end-to-end behavior of the stable `const fn` is the same for both
    /// modes of execution. (Here, Undefined Behavior is considered "the same"
    /// as any other behavior, so if the function exhibits UB at runtime then
    /// it may do whatever it wants at compile-time.)
    ///
    /// Here is an example of how this could cause a problem:
    /// ```no_run
    /// #![feature(const_eval_select)]
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    /// use std::intrinsics::const_eval_select;
    ///
    /// // Standard library
    /// pub const fn inconsistent() -> i32 {
    ///     fn runtime() -> i32 { 1 }
    ///     const fn compiletime() -> i32 { 2 }
    ///
    ///     // ⚠ This code violates the required equivalence of `compiletime`
    ///     // and `runtime`.
    ///     const_eval_select((), compiletime, runtime)
    /// }
    ///
    /// // User Crate
    /// const X: i32 = inconsistent();
    /// let x = inconsistent();
    /// assert_eq!(x, X);
    /// ```
    ///
    /// Currently such an assertion would always succeed; until Rust decides
    /// otherwise, that principle should not be violated.
    #[rustc_const_unstable(feature = "const_eval_select", issue = "124625")]
    #[rustc_intrinsic]
    pub const fn const_eval_select<ARG: Tuple, F, G, RET>(
        _arg: ARG,
        _called_in_const: F,
        _called_at_rt: G,
    ) -> RET
    where
        G: FnOnce<ARG, Output = RET>,
        F: const FnOnce<ARG, Output = RET>;

    /// A macro to make it easier to invoke const_eval_select. Use as follows:
    /// ```rust,ignore (just a macro example)
    /// const_eval_select!(
    ///     @capture { arg1: i32 = some_expr, arg2: T = other_expr } -> U:
    ///     if const #[attributes_for_const_arm] {
    ///         // Compile-time code goes here.
    ///     } else #[attributes_for_runtime_arm] {
    ///         // Run-time code goes here.
    ///     }
    /// )
    /// ```
    /// The `@capture` block declares which surrounding variables / expressions can be
    /// used inside the `if const`.
    /// Note that the two arms of this `if` really each become their own function, which is why the
    /// macro supports setting attributes for those functions. Both functions are marked as `#[inline]`.
    ///
    /// See [`const_eval_select()`] for the rules and requirements around that intrinsic.
    pub(crate) macro const_eval_select {
        (
            @capture$([$($binders:tt)*])? { $($arg:ident : $ty:ty = $val:expr),* $(,)? } $( -> $ret:ty )? :
            if const
                $(#[$compiletime_attr:meta])* $compiletime:block
            else
                $(#[$runtime_attr:meta])* $runtime:block
        ) => {{
            #[inline]
            $(#[$runtime_attr])*
            fn runtime$(<$($binders)*>)?($($arg: $ty),*) $( -> $ret )? {
                $runtime
            }

            #[inline]
            $(#[$compiletime_attr])*
            const fn compiletime$(<$($binders)*>)?($($arg: $ty),*) $( -> $ret )? {
                // Don't warn if one of the arguments is unused.
                $(let _ = $arg;)*

                $compiletime
            }

            const_eval_select(($($val,)*), compiletime, runtime)
        }},
        // We support leaving away the `val` expressions for *all* arguments
        // (but not for *some* arguments, that's too tricky).
        (
            @capture$([$($binders:tt)*])? { $($arg:ident : $ty:ty),* $(,)? } $( -> $ret:ty )? :
            if const
                $(#[$compiletime_attr:meta])* $compiletime:block
            else
                $(#[$runtime_attr:meta])* $runtime:block
        ) => {
            $crate::intrinsics::const_eval_select!(
                @capture$([$($binders)*])? { $($arg : $ty = $arg),* } $(-> $ret)? :
                if const
                    $(#[$compiletime_attr])* $compiletime
                else
                    $(#[$runtime_attr])* $runtime
            )
        },
    }

    /// Returns whether the argument's value is statically known at
    /// compile-time.
    ///
    /// This is useful when there is a way of writing the code that will
    /// be *faster* when some variables have known values, but *slower*
    /// in the general case: an `if is_val_statically_known(var)` can be used
    /// to select between these two variants. The `if` will be optimized away
    /// and only the desired branch remains.
    ///
    /// Formally speaking, this function non-deterministically returns `true`
    /// or `false`, and the caller has to ensure sound behavior for both cases.
    /// In other words, the following code has *Undefined Behavior*:
    ///
    /// ```no_run
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    /// use std::hint::unreachable_unchecked;
    /// use std::intrinsics::is_val_statically_known;
    ///
    /// if !is_val_statically_known(0) { unsafe { unreachable_unchecked(); } }
    /// ```
    ///
    /// This also means that the following code's behavior is unspecified; it
    /// may panic, or it may not:
    ///
    /// ```no_run
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    /// use std::intrinsics::is_val_statically_known;
    ///
    /// assert_eq!(is_val_statically_known(0), is_val_statically_known(0));
    /// ```
    ///
    /// Unsafe code may not rely on `is_val_statically_known` returning any
    /// particular value, ever. However, the compiler will generally make it
    /// return `true` only if the value of the argument is actually known.
    ///
    /// # Type Requirements
    ///
    /// `T` must be either a `bool`, a `char`, a primitive numeric type (e.g. `f32`,
    /// but not `NonZeroISize`), or any thin pointer (e.g. `*mut String`).
    /// Any other argument types *may* cause a compiler error.
    ///
    /// ## Pointers
    ///
    /// When the input is a pointer, only the pointer itself is
    /// ever considered. The pointee has no effect. Currently, these functions
    /// behave identically:
    ///
    /// ```
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    /// use std::intrinsics::is_val_statically_known;
    ///
    /// fn foo(x: &i32) -> bool {
    ///     is_val_statically_known(x)
    /// }
    ///
    /// fn bar(x: &i32) -> bool {
    ///     is_val_statically_known(
    ///         (x as *const i32).addr()
    ///     )
    /// }
    /// # _ = foo(&5_i32);
    /// # _ = bar(&5_i32);
    /// ```
    #[rustc_const_stable_indirect]
    #[rustc_nounwind]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_intrinsic]
    pub const fn is_val_statically_known<T: Copy>(_arg: T) -> bool {
        false
    }

    /// Non-overlapping *typed* swap of a single value.
    ///
    /// The codegen backends will replace this with a better implementation when
    /// `T` is a simple type that can be loaded and stored as an immediate.
    ///
    /// The stabilized form of this intrinsic is [`crate::mem::swap`].
    ///
    /// # Safety
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// * Both `x` and `y` must be [valid] for both reads and writes.
    ///
    /// * Both `x` and `y` must be properly aligned.
    ///
    /// * The region of memory beginning at `x` must *not* overlap with the region of memory
    ///   beginning at `y`.
    ///
    /// * The memory pointed by `x` and `y` must both contain values of type `T`.
    ///
    /// [valid]: crate::ptr#safety
    #[rustc_nounwind]
    #[inline]
    #[rustc_intrinsic]
    #[rustc_intrinsic_const_stable_indirect]
    pub const unsafe fn typed_swap_nonoverlapping<T>(x: *mut T, y: *mut T) {
        // SAFETY: The caller provided single non-overlapping items behind
        // pointers, so swapping them with `count: 1` is fine.
        unsafe { ptr::swap_nonoverlapping(x, y, 1) };
    }

    /// Returns whether we should perform some UB-checking at runtime.
    #[rustc_intrinsic_const_stable_indirect] #[inline(always)] #[rustc_intrinsic] pub const fn ub_checks() -> bool { true }

    /// Returns whether we should perform some overflow-checking at runtime. This eventually evaluates to
    /// `cfg!(overflow_checks)`, but behaves different from `cfg!` when mixing crates built with different
    /// flags: if the crate has overflow checks enabled or carries the `#[rustc_inherit_overflow_checks]`
    /// attribute, evaluation is delayed until monomorphization (or until the call gets inlined into
    /// a crate that does not delay evaluation further); otherwise it can happen any time.
    ///
    /// The common case here is a user program built with overflow_checks linked against the distributed
    /// sysroot which is built without overflow_checks but with `#[rustc_inherit_overflow_checks]`.
    /// For code that gets monomorphized in the user crate (i.e., generic functions and functions with
    /// `#[inline]`), gating assertions on `overflow_checks()` rather than `cfg!(overflow_checks)` means that
    /// assertions are enabled whenever the *user crate* has overflow checks enabled. However if the
    /// user has overflow checks disabled, the checks will still get optimized out.
    ///
    /// # Consteval
    ///
    /// In consteval, this function currently returns `true`. This is because the value of the `overflow_checks`
    /// configuration can differ across crates, but we need this function to always return the same
    /// value in consteval in order to avoid unsoundness.
    #[inline(always)]
    #[rustc_intrinsic]
    pub const fn overflow_checks() -> bool {
        cfg!(debug_assertions)
    }

    /// Allocates a block of memory at compile time.
    /// At runtime, just returns a null pointer.
    ///
    /// # Safety
    ///
    /// - The `align` argument must be a power of two.
    ///    - At compile time, a compile error occurs if this constraint is violated.
    ///    - At runtime, it is not checked.
    #[rustc_const_unstable(feature = "const_heap", issue = "79597")]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    #[miri::intrinsic_fallback_is_spec]
    pub const unsafe fn const_allocate(_size: usize, _align: usize) -> *mut u8 {
        // const eval overrides this function, but runtime code for now just returns null pointers.
        // See <https://github.com/rust-lang/rust/issues/93935>.
        crate::ptr::null_mut()
    }

    /// Deallocates a memory which allocated by `intrinsics::const_allocate` at compile time.
    /// At runtime, it does nothing.
    ///
    /// # Safety
    ///
    /// - The `align` argument must be a power of two.
    ///    - At compile time, a compile error occurs if this constraint is violated.
    ///    - At runtime, it is not checked.
    /// - If the `ptr` is created in an another const, this intrinsic doesn't deallocate it.
    /// - If the `ptr` is pointing to a local variable, this intrinsic doesn't deallocate it.
    #[rustc_const_unstable(feature = "const_heap", issue = "79597")]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    #[miri::intrinsic_fallback_is_spec]
    pub const unsafe fn const_deallocate(_ptr: *mut u8, _size: usize, _align: usize) {
        // Runtime NOP
    }

    /// Convert the allocation this pointer points to into immutable global memory.
    /// The pointer must point to the beginning of a heap allocation.
    /// This operation only makes sense during compile time. At runtime, it does nothing.
    #[rustc_const_unstable(feature = "const_heap", issue = "79597")]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    #[miri::intrinsic_fallback_is_spec]
    pub const unsafe fn const_make_global(ptr: *mut u8) -> *const u8 {
        // const eval overrides this function; at runtime, it is a NOP.
        ptr
    }

    /// Check if the pre-condition `cond` has been met.
    ///
    /// By default, if `contract_checks` is enabled, this will panic with no unwind if the condition
    /// returns false.
    ///
    /// Note that this function is a no-op during constant evaluation.
    #[unstable(feature = "contracts_internals", issue = "128044")]
    // Calls to this function get inserted by an AST expansion pass, which uses the equivalent of
    // `#[allow_internal_unstable]` to allow using `contracts_internals` functions. Const-checking
    // doesn't honor `#[allow_internal_unstable]`, so for the const feature gate we use the user-facing
    // `contracts` feature rather than the perma-unstable `contracts_internals`
    #[rustc_const_unstable(feature = "contracts", issue = "128044")]
    #[lang = "contract_check_requires"]
    #[rustc_intrinsic]
    pub const fn contract_check_requires<C: Fn() -> bool + Copy>(cond: C) {
        const_eval_select!(
            @capture[C: Fn() -> bool + Copy] { cond: C } :
            if const {
                    // Do nothing
            } else {
                if !cond() {
                    // Emit no unwind panic in case this was a safety requirement.
                    crate::panicking::panic_nounwind("failed requires check");
                }
            }
        )
    }

    /// Check if the post-condition `cond` has been met.
    ///
    /// By default, if `contract_checks` is enabled, this will panic with no unwind if the condition
    /// returns false.
    ///
    /// If `cond` is `None`, then no postcondition checking is performed.
    ///
    /// Note that this function is a no-op during constant evaluation.
    #[unstable(feature = "contracts_internals", issue = "128044")]
    // Similar to `contract_check_requires`, we need to use the user-facing
    // `contracts` feature rather than the perma-unstable `contracts_internals`.
    // Const-checking doesn't honor allow_internal_unstable logic used by contract expansion.
    #[rustc_const_unstable(feature = "contracts", issue = "128044")]
    #[lang = "contract_check_ensures"]
    #[rustc_intrinsic]
    pub const fn contract_check_ensures<C: Fn(&Ret) -> bool + Copy, Ret>(
        cond: Option<C>,
        ret: Ret,
    ) -> Ret {
        const_eval_select!(
            @capture[C: Fn(&Ret) -> bool + Copy, Ret] { cond: Option<C>, ret: Ret } -> Ret :
            if const {
                // Do nothing
                ret
            } else {
                match cond {
                    crate::option::Option::Some(cond) => {
                        if !cond(&ret) {
                            // Emit no unwind panic in case this was a safety requirement.
                            crate::panicking::panic_nounwind("failed ensures check");
                        }
                    },
                    crate::option::Option::None => {},
                }
                ret
            }
        )
    }

    /// The intrinsic will return the size stored in that vtable.
    ///
    /// # Safety
    ///
    /// `ptr` must point to a vtable.
    #[rustc_nounwind]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_intrinsic]
    pub unsafe fn vtable_size(ptr: *const ()) -> usize;

    /// The intrinsic will return the alignment stored in that vtable.
    ///
    /// # Safety
    ///
    /// `ptr` must point to a vtable.
    #[rustc_nounwind]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_intrinsic]
    pub unsafe fn vtable_align(ptr: *const ()) -> usize;

    /// The size of a type in bytes.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// More specifically, this is the offset in bytes between successive
    /// items of the same type, including alignment padding.
    ///
    /// Note that, unlike most intrinsics, this can only be called at compile-time
    /// as backends do not have an implementation for it. The only caller (its
    /// stable counterpart) wraps this intrinsic call in a `const` block so that
    /// backends only see an evaluated constant.
    ///
    /// The stabilized version of this intrinsic is [`core::mem::size_of`].
    #[rustc_nounwind]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    pub const fn size_of<T>() -> usize;

    /// The minimum alignment of a type.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// Note that, unlike most intrinsics, this can only be called at compile-time
    /// as backends do not have an implementation for it. The only caller (its
    /// stable counterpart) wraps this intrinsic call in a `const` block so that
    /// backends only see an evaluated constant.
    ///
    /// The stabilized version of this intrinsic is [`core::mem::align_of`].
    #[rustc_nounwind]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    pub const fn align_of<T>() -> usize;

    /// The offset of a field inside a type.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// This intrinsic can only be evaluated at compile-time, and should only appear in
    /// constants or inline const blocks.
    ///
    /// The stabilized version of this intrinsic is [`core::mem::offset_of`].
    /// This intrinsic is also a lang item so `offset_of!` can desugar to calls to it.
    #[rustc_nounwind]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_const_unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    #[lang = "offset_of"]
    pub const fn offset_of<T: PointeeSized>(variant: u32, field: u32) -> usize;

    /// The offset of a field queried by its field representing type.
    ///
    /// Returns the offset of the field represented by `F`. This function essentially does the same as
    /// the [`offset_of`] intrinsic, but expects the field to be represented by a generic rather than
    /// the variant and field indices. This also is a safe intrinsic and can only be evaluated at
    /// compile-time, so it should only appear in constants or inline const blocks.
    ///
    /// There should be no need to call this intrinsic manually, as its value is used to define
    /// [`Field::OFFSET`](crate::field::Field::OFFSET), which is publicly accessible.
    #[rustc_intrinsic]
    #[unstable(feature = "field_projections", issue = "145383")]
    #[rustc_const_unstable(feature = "field_projections", issue = "145383")]
    pub const fn field_offset<F: crate::field::Field>() -> usize;

    /// Returns the number of variants of the type `T` cast to a `usize`;
    /// if `T` has no variants, returns `0`. Uninhabited variants will be counted.
    ///
    /// Note that, unlike most intrinsics, this can only be called at compile-time
    /// as backends do not have an implementation for it. The only caller (its
    /// stable counterpart) wraps this intrinsic call in a `const` block so that
    /// backends only see an evaluated constant.
    ///
    /// The to-be-stabilized version of this intrinsic is [`crate::mem::variant_count`].
    #[rustc_nounwind]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_intrinsic]
    pub const fn variant_count<T>() -> usize;

    /// The size of the referenced value in bytes.
    ///
    /// The stabilized version of this intrinsic is [`core::mem::size_of_val`].
    ///
    /// # Safety
    ///
    /// See [`crate::mem::size_of_val_raw`] for safety conditions.
    #[rustc_nounwind]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_intrinsic]
    #[rustc_intrinsic_const_stable_indirect]
    pub const unsafe fn size_of_val<T: ?Sized>(ptr: *const T) -> usize;

    /// The required alignment of the referenced value.
    ///
    /// The stabilized version of this intrinsic is [`core::mem::align_of_val`].
    ///
    /// # Safety
    ///
    /// See [`crate::mem::align_of_val_raw`] for safety conditions.
    #[rustc_nounwind]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_intrinsic]
    #[rustc_intrinsic_const_stable_indirect]
    pub const unsafe fn align_of_val<T: ?Sized>(ptr: *const T) -> usize;

    #[rustc_intrinsic]
    #[rustc_comptime]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    /// Check if a type represented by a `TypeId` implements a trait represented by a `TypeId`.
    /// It can only be called at compile time, the backends do
    /// not implement it. If it implements the trait the dyn metadata gets returned for vtable access.
    pub fn type_id_vtable(
        _id: crate::any::TypeId,
        _trait: crate::any::TypeId,
    ) -> Option<ptr::DynMetadata<*const ()>> {
        panic!(
            "`TypeId::trait_info_of` and `trait_info_of_trait_type_id` can only be called at compile-time"
        )
    }

    /// Compute the type information of a concrete type.
    /// It can only be called at compile time, the backends do
    /// not implement it.
    #[rustc_intrinsic]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    pub const fn type_of(_id: crate::any::TypeId) -> crate::mem::type_info::Type {
        panic!("`TypeId::info` can only be called at compile-time")
    }

    /// Gets a static string slice containing the name of a type.
    ///
    /// Note that, unlike most intrinsics, this can only be called at compile-time
    /// as backends do not have an implementation for it. The only caller (its
    /// stable counterpart) wraps this intrinsic call in a `const` block so that
    /// backends only see an evaluated constant.
    ///
    /// The stabilized version of this intrinsic is [`core::any::type_name`].
    #[rustc_nounwind]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_intrinsic]
    pub const fn type_name<T: ?Sized>() -> &'static str;

    /// Gets an identifier which is globally unique to the specified type. This
    /// function will return the same value for a type regardless of whichever
    /// crate it is invoked in.
    ///
    /// Note that, unlike most intrinsics, this can only be called at compile-time
    /// as backends do not have an implementation for it. The only caller (its
    /// stable counterpart) wraps this intrinsic call in a `const` block so that
    /// backends only see an evaluated constant.
    ///
    /// The stabilized version of this intrinsic is [`core::any::TypeId::of`].
    #[rustc_nounwind]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_intrinsic]
    #[rustc_comptime]
    pub fn type_id<T: ?Sized>() -> crate::any::TypeId;

    /// Tests (at compile-time) if two [`crate::any::TypeId`] instances identify the
    /// same type. This is necessary because at const-eval time the actual discriminating
    /// data is opaque and cannot be inspected directly.
    ///
    /// The stabilized version of this intrinsic is the [PartialEq] impl for [`core::any::TypeId`].
    #[rustc_nounwind]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_intrinsic]
    #[rustc_do_not_const_check]
    pub const fn type_id_eq(a: crate::any::TypeId, b: crate::any::TypeId) -> bool {
        // SAFETY: we know `TypeId` is 16 bytes of initialized data.
        // This is runtime-only code so we do not have to worry about provenance.
        unsafe { crate::mem::transmute::<_, u128>(a) == crate::mem::transmute::<_, u128>(b) }
    }

    /// Gets the size of the type represented by this `TypeId`.
    ///
    /// The more user-friendly version of this intrinsic is [`core::any::TypeId::size`].
    #[rustc_intrinsic]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_comptime]
    pub fn size_of_type_id(_id: crate::any::TypeId) -> Option<usize> {
        panic!("`TypeId::size` can only be called at compile-time")
    }

    /// Gets the number of variants of the type represented by this `TypeId`.
    ///
    /// The more user-friendly version of this intrinsic is [`core::any::TypeId::variants`].
    #[rustc_intrinsic]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_comptime]
    pub fn type_id_variants(_id: crate::any::TypeId) -> usize {
        panic!("`TypeId::variants` can only be called at compile-time")
    }

    /// Gets the number of fields at the given `variant_index` represented by this `TypeId`.
    ///
    /// The more user-friendly version of this intrinsic is [`core::any::TypeId::fields`].
    #[rustc_intrinsic]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_comptime]
    pub fn type_id_fields(_id: crate::any::TypeId, _variant_index: usize) -> usize {
        panic!("`TypeId::fields` can only be called at compile-time")
    }

    /// Gets the [`FieldRepresentingType`]'s `TypeId` at the given index of the type represented by this `TypeId`.
    ///
    /// The more user-friendly version of this intrinsic is [`core::any::TypeId::field`].
    ///
    /// [`FieldRepresentingType`]: crate::field::FieldRepresentingType
    #[rustc_intrinsic]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_comptime]
    pub fn type_id_field_representing_type(
        _id: crate::any::TypeId,
        _variant_index: usize,
        _field_index: usize,
    ) -> crate::any::TypeId {
        panic!("`TypeId::field` can only be called at compile-time")
    }

    /// Gets the actual field `TypeId` of the [`FieldRepresentingType`]'s `TypeId`.
    ///
    /// The more user-friendly version of this intrinsic is [`core::mem::type_info::FieldId::type_id`].
    ///
    /// [`FieldRepresentingType`]: crate::field::FieldRepresentingType
    #[rustc_intrinsic]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_comptime]
    pub fn field_representing_type_actual_type_id(
        _frt_type_id: crate::any::TypeId,
    ) -> crate::any::TypeId {
        panic!("`FieldId::type_id` can only be called at compile-time")
    }

    /// Lowers in MIR to `Rvalue::Aggregate` with `AggregateKind::RawPtr`.
    ///
    /// This is used to implement functions like `slice::from_raw_parts_mut` and
    /// `ptr::from_raw_parts` in a way compatible with the compiler being able to
    /// change the possible layouts of pointers.
    #[rustc_nounwind]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    pub const fn aggregate_raw_ptr<P: bounds::BuiltinDeref, D, M>(data: D, meta: M) -> P
    where
        <P as bounds::BuiltinDeref>::Pointee: ptr::Pointee<Metadata = M>;

    /// Lowers in MIR to `Rvalue::UnaryOp` with `UnOp::PtrMetadata`.
    ///
    /// This is used to implement functions like `ptr::metadata`.
    #[rustc_nounwind]
    #[unstable(feature = "core_intrinsics", issue = "none")]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    pub const fn ptr_metadata<P: ptr::Pointee<Metadata = M> + PointeeSized, M>(ptr: *const P) -> M;

    /// This is an accidentally-stable alias to [`ptr::copy_nonoverlapping`]; use that instead.
    // Note (intentionally not in the doc comment): `ptr::copy_nonoverlapping` adds some extra
    // debug assertions; if you are writing compiler tests or code inside the standard library
    // that wants to avoid those debug assertions, directly call this intrinsic instead.
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_allowed_through_unstable_modules = "import this function via `std::ptr` instead"]
    #[rustc_const_stable(feature = "const_intrinsic_copy", since = "1.83.0")]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize);

    /// This is an accidentally-stable alias to [`ptr::copy`]; use that instead.
    // Note (intentionally not in the doc comment): `ptr::copy` adds some extra
    // debug assertions; if you are writing compiler tests or code inside the standard library
    // that wants to avoid those debug assertions, directly call this intrinsic instead.
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_allowed_through_unstable_modules = "import this function via `std::ptr` instead"]
    #[rustc_const_stable(feature = "const_intrinsic_copy", since = "1.83.0")]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn copy<T>(src: *const T, dst: *mut T, count: usize);

    /// This is an accidentally-stable alias to [`ptr::write_bytes`]; use that instead.
    // Note (intentionally not in the doc comment): `ptr::write_bytes` adds some extra
    // debug assertions; if you are writing compiler tests or code inside the standard library
    // that wants to avoid those debug assertions, directly call this intrinsic instead.
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_allowed_through_unstable_modules = "import this function via `std::ptr` instead"]
    #[rustc_const_stable(feature = "const_intrinsic_copy", since = "1.83.0")]
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const unsafe fn write_bytes<T>(dst: *mut T, val: u8, count: usize);

    /// Returns the minimum of two `f16` values, ignoring NaN.
    ///
    /// This behaves like IEEE 754-2019 minimumNumber, *except* that it does not order signed
    /// zeros deterministically. In particular:
    /// If one of the arguments is NaN (quiet or signaling), then the other argument is returned. If
    /// both arguments are NaN, returns NaN. If the inputs compare equal (such as for the case of `+0.0`
    /// and `-0.0`), either input may be returned non-deterministically.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is [`f16::min`].
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn minimum_number_nsz_f16(x: f16, y: f16) -> f16 {
        if x.is_nan() || y <= x {
            y
        } else {
            // Either y > x or y is a NaN.
            x
        }
    }

    /// Returns the minimum of two `f32` values, ignoring NaN.
    ///
    /// This behaves like IEEE 754-2019 minimumNumber, *except* that it does not order signed
    /// zeros deterministically. In particular:
    /// If one of the arguments is NaN (quiet or signaling), then the other argument is returned. If
    /// both arguments are NaN, returns NaN. If the inputs compare equal (such as for the case of `+0.0`
    /// and `-0.0`), either input may be returned non-deterministically.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is [`f32::min`].
    #[rustc_nounwind]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    pub const fn minimum_number_nsz_f32(x: f32, y: f32) -> f32 {
        if x.is_nan() || y <= x {
            y
        } else {
            // Either y > x or y is a NaN.
            x
        }
    }

    /// Returns the minimum of two `f64` values, ignoring NaN.
    ///
    /// This behaves like IEEE 754-2019 minimumNumber, *except* that it does not order signed
    /// zeros deterministically. In particular:
    /// If one of the arguments is NaN (quiet or signaling), then the other argument is returned. If
    /// both arguments are NaN, returns NaN. If the inputs compare equal (such as for the case of `+0.0`
    /// and `-0.0`), either input may be returned non-deterministically.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is [`f64::min`].
    #[rustc_nounwind]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    pub const fn minimum_number_nsz_f64(x: f64, y: f64) -> f64 {
        if x.is_nan() || y <= x {
            y
        } else {
            // Either y > x or y is a NaN.
            x
        }
    }

    /// Returns the minimum of two `f128` values, ignoring NaN.
    ///
    /// This behaves like IEEE 754-2019 minimumNumber, *except* that it does not order signed
    /// zeros deterministically. In particular:
    /// If one of the arguments is NaN (quiet or signaling), then the other argument is returned. If
    /// both arguments are NaN, returns NaN. If the inputs compare equal (such as for the case of `+0.0`
    /// and `-0.0`), either input may be returned non-deterministically.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is [`f128::min`].
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn minimum_number_nsz_f128(x: f128, y: f128) -> f128 {
        if x.is_nan() || y <= x {
            y
        } else {
            // Either y > x or y is a NaN.
            x
        }
    }

    /// Returns the minimum of two `f16` values, propagating NaN.
    ///
    /// This behaves like IEEE 754-2019 minimum. In particular:
    /// If one of the arguments is NaN, then a NaN is returned using the usual NaN propagation rules.
    /// For this operation, -0.0 is considered to be strictly less than +0.0.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn minimumf16(x: f16, y: f16) -> f16 {
        if x < y {
            x
        } else if y < x {
            y
        } else if x == y {
            if x.is_sign_negative() && y.is_sign_positive() { x } else { y }
        } else {
            // At least one input is NaN. Use `+` to perform NaN propagation and quieting.
            x + y
        }
    }

    /// Returns the minimum of two `f32` values, propagating NaN.
    ///
    /// This behaves like IEEE 754-2019 minimum. In particular:
    /// If one of the arguments is NaN, then a NaN is returned using the usual NaN propagation rules.
    /// For this operation, -0.0 is considered to be strictly less than +0.0.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn minimumf32(x: f32, y: f32) -> f32 {
        if x < y {
            x
        } else if y < x {
            y
        } else if x == y {
            if x.is_sign_negative() && y.is_sign_positive() { x } else { y }
        } else {
            // At least one input is NaN. Use `+` to perform NaN propagation and quieting.
            x + y
        }
    }

    /// Returns the minimum of two `f64` values, propagating NaN.
    ///
    /// This behaves like IEEE 754-2019 minimum. In particular:
    /// If one of the arguments is NaN, then a NaN is returned using the usual NaN propagation rules.
    /// For this operation, -0.0 is considered to be strictly less than +0.0.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn minimumf64(x: f64, y: f64) -> f64 {
        if x < y {
            x
        } else if y < x {
            y
        } else if x == y {
            if x.is_sign_negative() && y.is_sign_positive() { x } else { y }
        } else {
            // At least one input is NaN. Use `+` to perform NaN propagation and quieting.
            x + y
        }
    }

    /// Returns the minimum of two `f128` values, propagating NaN.
    ///
    /// This behaves like IEEE 754-2019 minimum. In particular:
    /// If one of the arguments is NaN, then a NaN is returned using the usual NaN propagation rules.
    /// For this operation, -0.0 is considered to be strictly less than +0.0.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn minimumf128(x: f128, y: f128) -> f128 {
        if x < y {
            x
        } else if y < x {
            y
        } else if x == y {
            if x.is_sign_negative() && y.is_sign_positive() { x } else { y }
        } else {
            // At least one input is NaN. Use `+` to perform NaN propagation and quieting.
            x + y
        }
    }

    /// Returns the maximum of two `f16` values, ignoring NaN.
    ///
    /// This behaves like IEEE 754-2019 maximumNumber, *except* that it does not order signed
    /// zeros deterministically. In particular:
    /// If one of the arguments is NaN (quiet or signaling), then the other argument is returned. If
    /// both arguments are NaN, returns NaN. If the inputs compare equal (such as for the case of `+0.0`
    /// and `-0.0`), either input may be returned non-deterministically.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is [`f16::max`].
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn maximum_number_nsz_f16(x: f16, y: f16) -> f16 {
        if x.is_nan() || y >= x {
            y
        } else {
            // Either y < x or y is a NaN.
            x
        }
    }

    /// Returns the maximum of two `f32` values, ignoring NaN.
    ///
    /// This behaves like IEEE 754-2019 maximumNumber, *except* that it does not order signed
    /// zeros deterministically. In particular:
    /// If one of the arguments is NaN (quiet or signaling), then the other argument is returned. If
    /// both arguments are NaN, returns NaN. If the inputs compare equal (such as for the case of `+0.0`
    /// and `-0.0`), either input may be returned non-deterministically.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is [`f32::max`].
    #[rustc_nounwind]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    pub const fn maximum_number_nsz_f32(x: f32, y: f32) -> f32 {
        if x.is_nan() || y >= x {
            y
        } else {
            // Either y < x or y is a NaN.
            x
        }
    }

    /// Returns the maximum of two `f64` values, ignoring NaN.
    ///
    /// This behaves like IEEE 754-2019 maximumNumber, *except* that it does not order signed
    /// zeros deterministically. In particular:
    /// If one of the arguments is NaN (quiet or signaling), then the other argument is returned. If
    /// both arguments are NaN, returns NaN. If the inputs compare equal (such as for the case of `+0.0`
    /// and `-0.0`), either input may be returned non-deterministically.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is [`f64::max`].
    #[rustc_nounwind]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    pub const fn maximum_number_nsz_f64(x: f64, y: f64) -> f64 {
        if x.is_nan() || y >= x {
            y
        } else {
            // Either y < x or y is a NaN.
            x
        }
    }

    /// Returns the maximum of two `f128` values, ignoring NaN.
    ///
    /// This behaves like IEEE 754-2019 maximumNumber, *except* that it does not order signed
    /// zeros deterministically. In particular:
    /// If one of the arguments is NaN (quiet or signaling), then the other argument is returned. If
    /// both arguments are NaN, returns NaN. If the inputs compare equal (such as for the case of `+0.0`
    /// and `-0.0`), either input may be returned non-deterministically.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is [`f128::max`].
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn maximum_number_nsz_f128(x: f128, y: f128) -> f128 {
        if x.is_nan() || y >= x {
            y
        } else {
            // Either y < x or y is a NaN.
            x
        }
    }

    /// Returns the maximum of two `f16` values, propagating NaN.
    ///
    /// This behaves like IEEE 754-2019 maximum. In particular:
    /// If one of the arguments is NaN, then a NaN is returned using the usual NaN propagation rules.
    /// For this operation, -0.0 is considered to be strictly less than +0.0.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn maximumf16(x: f16, y: f16) -> f16 {
        if x > y {
            x
        } else if y > x {
            y
        } else if x == y {
            if x.is_sign_positive() && y.is_sign_negative() { x } else { y }
        } else {
            x + y
        }
    }

    /// Returns the maximum of two `f32` values, propagating NaN.
    ///
    /// This behaves like IEEE 754-2019 maximum. In particular:
    /// If one of the arguments is NaN, then a NaN is returned using the usual NaN propagation rules.
    /// For this operation, -0.0 is considered to be strictly less than +0.0.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn maximumf32(x: f32, y: f32) -> f32 {
        if x > y {
            x
        } else if y > x {
            y
        } else if x == y {
            if x.is_sign_positive() && y.is_sign_negative() { x } else { y }
        } else {
            x + y
        }
    }

    /// Returns the maximum of two `f64` values, propagating NaN.
    ///
    /// This behaves like IEEE 754-2019 maximum. In particular:
    /// If one of the arguments is NaN, then a NaN is returned using the usual NaN propagation rules.
    /// For this operation, -0.0 is considered to be strictly less than +0.0.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn maximumf64(x: f64, y: f64) -> f64 {
        if x > y {
            x
        } else if y > x {
            y
        } else if x == y {
            if x.is_sign_positive() && y.is_sign_negative() { x } else { y }
        } else {
            x + y
        }
    }

    /// Returns the maximum of two `f128` values, propagating NaN.
    ///
    /// This behaves like IEEE 754-2019 maximum. In particular:
    /// If one of the arguments is NaN, then a NaN is returned using the usual NaN propagation rules.
    /// For this operation, -0.0 is considered to be strictly less than +0.0.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn maximumf128(x: f128, y: f128) -> f128 {
        if x > y {
            x
        } else if y > x {
            y
        } else if x == y {
            if x.is_sign_positive() && y.is_sign_negative() { x } else { y }
        } else {
            x + y
        }
    }

    /// Returns the absolute value of a floating-point value.
    ///
    /// The stabilized versions of this intrinsic are available on the float
    /// primitives via the `abs` method. For example, [`f32::abs`].
    #[rustc_nounwind]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    pub const fn fabs<T: bounds::FloatPrimitive>(x: T) -> T;
    /// Copies the sign from `y` to `x` for `f16` values.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f16::copysign`](../../std/primitive.f16.html#method.copysign)
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn copysignf16(x: f16, y: f16) -> f16;

    /// Copies the sign from `y` to `x` for `f32` values.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::copysign`](../../std/primitive.f32.html#method.copysign)
    #[rustc_nounwind]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    pub const fn copysignf32(x: f32, y: f32) -> f32;
    /// Copies the sign from `y` to `x` for `f64` values.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::copysign`](../../std/primitive.f64.html#method.copysign)
    #[rustc_nounwind]
    #[rustc_intrinsic_const_stable_indirect]
    #[rustc_intrinsic]
    pub const fn copysignf64(x: f64, y: f64) -> f64;

    /// Copies the sign from `y` to `x` for `f128` values.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f128::copysign`](../../std/primitive.f128.html#method.copysign)
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn copysignf128(x: f128, y: f128) -> f128;

    /// Generates the LLVM body for the automatic differentiation of `f` using Enzyme,
    /// with `df` as the derivative function and `args` as its arguments.
    ///
    /// Used internally as the body of `df` when expanding the `#[autodiff_forward]`
    /// and `#[autodiff_reverse]` attribute macros.
    ///
    /// Type Parameters:
    /// - `F`: The original function to differentiate. Must be a function item.
    /// - `G`: The derivative function. Must be a function item.
    /// - `T`: A tuple of arguments passed to `df`.
    /// - `R`: The return type of the derivative function.
    ///
    /// This shows where the `autodiff` intrinsic is used during macro expansion:
    ///
    /// ```rust,ignore (macro example)
    /// #[autodiff_forward(df1, Dual, Const, Dual)]
    /// pub fn f1(x: &[f64], y: f64) -> f64 {
    ///     unimplemented!()
    /// }
    /// ```
    ///
    /// expands to:
    ///
    /// ```rust,ignore (macro example)
    /// #[rustc_autodiff]
    /// #[inline(never)]
    /// pub fn f1(x: &[f64], y: f64) -> f64 {
    ///     ::core::panicking::panic("not implemented")
    /// }
    /// #[rustc_autodiff(Forward, 1, Dual, Const, Dual)]
    /// pub fn df1(x: &[f64], bx_0: &[f64], y: f64) -> (f64, f64) {
    ///     ::core::intrinsics::autodiff(f1::<>, df1::<>, (x, bx_0, y))
    /// }
    /// ```
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn autodiff<F, G, T: crate::marker::Tuple, R>(f: F, df: G, args: T) -> R;

    /// Generates the LLVM body of a wrapper function to offload a kernel `f`.
    ///
    /// Type Parameters:
    /// - `F`: The kernel to offload. Must be a function item.
    /// - `T`: A tuple of arguments passed to `f`.
    /// - `R`: The return type of the kernel.
    ///
    /// Arguments:
    /// - `f`: The kernel function to offload.
    /// - `workgroup_dim`: A 3D size specifying the number of workgroups to launch.
    /// - `thread_dim`: A 3D size specifying the number of threads per workgroup.
    /// - `args`: A tuple of arguments forwarded to `f`.
    ///
    /// Example usage (pseudocode):
    ///
    /// ```rust,ignore (pseudocode)
    /// fn kernel(x: *mut [f64; 128]) {
    ///     core::intrinsics::offload(kernel_1, [256, 1, 1], [32, 1, 1], (x,))
    /// }
    ///
    /// #[cfg(target_os = "linux")]
    /// extern "C" {
    ///     pub fn kernel_1(array_b: *mut [f64; 128]);
    /// }
    ///
    /// #[cfg(not(target_os = "linux"))]
    /// #[rustc_offload_kernel]
    /// extern "gpu-kernel" fn kernel_1(x: *mut [f64; 128]) {
    ///     unsafe { (*x)[0] = 21.0 };
    /// }
    /// ```
    ///
    /// For reference, see the Clang documentation on offloading:
    /// <https://clang.llvm.org/docs/OffloadingDesign.html>.
    #[rustc_nounwind]
    #[rustc_intrinsic]
    pub const fn offload<F, T: crate::marker::Tuple, R>(
        f: F,
        workgroup_dim: [u32; 3],
        thread_dim: [u32; 3],
        dyn_cache: u32,
        args: T,
    ) -> R;

    /// Inform Miri that a given pointer definitely has a certain alignment.
    #[cfg(miri)]
    #[rustc_allow_const_fn_unstable(const_eval_select)]
    pub(crate) const fn miri_promise_symbolic_alignment(ptr: *const (), align: usize) {
        unsafe extern "Rust" {
            /// Miri-provided extern function to promise that a given pointer is properly aligned for
            /// "symbolic" alignment checks. Will fail if the pointer is not actually aligned or `align` is
            /// not a power of two. Has no effect when alignment checks are concrete (which is the default).
            fn miri_promise_symbolic_alignment(ptr: *const (), align: usize);
        }

        const_eval_select!(
            @capture { ptr: *const (), align: usize}:
            if const {
                // Do nothing.
            } else {
                // SAFETY: this call is always safe.
                unsafe {
                    miri_promise_symbolic_alignment(ptr, align);
                }
            }
        )
    }

    /// Loads an argument of type `T` from the `va_list` `ap` and increment the
    /// argument `ap` points to.
    ///
    /// # Safety
    ///
    /// This function is only sound to call when:
    ///
    /// - there is a next variable argument available.
    /// - the next argument's type must be ABI-compatible with the type `T`.
    /// - the next argument must have a properly initialized value of type `T`.
    ///
    /// Calling this function with an incompatible type, an invalid value, or when there
    /// are no more variable arguments, is unsound.
    ///
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const unsafe fn va_arg<T: VaArgSafe>(ap: &mut VaList<'_>) -> T;
    /// Duplicates a variable argument list. The returned list is initially at the same position as
    /// the one in `src`, but can be advanced independently.
    ///
    /// Codegen backends should not have custom behavior for this intrinsic, they should always use
    /// this fallback implementation. This intrinsic *does not* map to the LLVM `va_copy` intrinsic.
    ///
    /// This intrinsic exists only as a hook for Miri and constant evaluation, and is used to detect UB
    /// when a variable argument list is used incorrectly.
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const fn va_copy<'f>(src: &VaList<'f>) -> VaList<'f> {
        // This fallback body exploits the fact that our codegen backends all just use
        // a plain memcpy to duplicate VaList. This assumption is wrong for Miri.
        assert!(!cfg!(miri), "fallback body is incorrect under Miri");

        src.duplicate()
    }

    /// Destroy the variable argument list `ap` after initialization with `va_start` (part of the
    /// desugaring of `...`) or `va_copy`.
    ///
    /// Code generation backends should not provide a custom implementation for this intrinsic. This
    /// intrinsic *does not* map to the LLVM `va_end` intrinsic.
    ///
    /// This function is a no-op on all current targets, but used as a hook for const evaluation to
    /// detect UB when a variable argument list is used incorrectly.
    ///
    /// # Safety
    ///
    /// `ap` must not be used to access variable arguments after this call.
    ///
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub const unsafe fn va_end(ap: &mut VaList<'_>) {
        /* deliberately does nothing */
    }

    /// Returns the return address of the caller function (after inlining) in a best-effort manner or a null pointer if it is not supported on the current backend.
    /// Returning an accurate value is a quality-of-implementation concern, but no hard guarantees are
    /// made about the return value: formally, the intrinsic non-deterministically returns
    /// an arbitrary pointer without provenance.
    ///
    /// Note that unlike most intrinsics, this is safe to call. This is because it only finds the return address of the immediate caller, which is guaranteed to be possible.
    /// Other forms of the corresponding gcc or llvm intrinsic (which can have wildly unpredictable results or even crash at runtime) are not exposed.
    #[rustc_intrinsic]
    #[rustc_nounwind]
    pub fn return_address() -> *const () {
        core::ptr::null()
    }

    pub mod simd
    {
        use crate::
        { 
            *,
        };
            use crate::marker::ConstParamTy;

            /// Inserts an element into a vector, returning the updated vector.
            ///
            /// `T` must be a vector with element type `U`, and `idx` must be `const`.
            ///
            /// # Safety
            ///
            /// `idx` must be in-bounds of the vector.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_insert<T, U>(x: T, idx: u32, val: U) -> T;

            /// Extracts an element from a vector.
            ///
            /// `T` must be a vector with element type `U`, and `idx` must be `const`.
            ///
            /// # Safety
            ///
            /// `idx` must be const and in-bounds of the vector.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_extract<T, U>(x: T, idx: u32) -> U;

            /// Inserts an element into a vector, returning the updated vector.
            ///
            /// `T` must be a vector with element type `U`.
            ///
            /// If the index is `const`, [`simd_insert`] may emit better assembly.
            ///
            /// # Safety
            ///
            /// `idx` must be in-bounds of the vector.
            #[rustc_nounwind]
            #[rustc_intrinsic]
            pub const unsafe fn simd_insert_dyn<T, U>(x: T, idx: u32, val: U) -> T;

            /// Extracts an element from a vector.
            ///
            /// `T` must be a vector with element type `U`.
            ///
            /// If the index is `const`, [`simd_extract`] may emit better assembly.
            ///
            /// # Safety
            ///
            /// `idx` must be in-bounds of the vector.
            #[rustc_nounwind]
            #[rustc_intrinsic]
            pub const unsafe fn simd_extract_dyn<T, U>(x: T, idx: u32) -> U;

            /// Creates a vector where every lane has the provided value.
            ///
            /// `T` must be a vector with element type `U`.
            #[rustc_nounwind]
            #[rustc_intrinsic]
            pub const unsafe fn simd_splat<T, U>(value: U) -> T;

            /// Adds two simd vectors elementwise.
            ///
            /// `T` must be a vector of integers or floats.
            /// For integers, wrapping arithmetic is used.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_add<T>(x: T, y: T) -> T;

            /// Subtracts `rhs` from `lhs` elementwise.
            ///
            /// `T` must be a vector of integers or floats.
            /// For integers, wrapping arithmetic is used.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_sub<T>(lhs: T, rhs: T) -> T;

            /// Multiplies two simd vectors elementwise.
            ///
            /// `T` must be a vector of integers or floats.
            /// For integers, wrapping arithmetic is used.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_mul<T>(x: T, y: T) -> T;

            /// Divides `lhs` by `rhs` elementwise.
            ///
            /// `T` must be a vector of integers or floats.
            ///
            /// # Safety
            /// For integers, `rhs` must not contain any zero elements.
            /// Additionally for signed integers, `<int>::MIN / -1` is undefined behavior.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_div<T>(lhs: T, rhs: T) -> T;

            /// Returns remainder of two vectors elementwise.
            ///
            /// `T` must be a vector of integers or floats.
            ///
            /// # Safety
            /// For integers, `rhs` must not contain any zero elements.
            /// Additionally for signed integers, `<int>::MIN / -1` is undefined behavior.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_rem<T>(lhs: T, rhs: T) -> T;

            /// Shifts vector left elementwise, with UB on overflow.
            ///
            /// Shifts `lhs` left by `rhs`, shifting in sign bits for signed types.
            ///
            /// `T` must be a vector of integers.
            ///
            /// # Safety
            ///
            /// Each element of `rhs` must be less than `<int>::BITS`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_shl<T>(lhs: T, rhs: T) -> T;

            /// Shifts vector right elementwise, with UB on overflow.
            ///
            /// `T` must be a vector of integers.
            ///
            /// Shifts `lhs` right by `rhs`, shifting in sign bits for signed types.
            ///
            /// # Safety
            ///
            /// Each element of `rhs` must be less than `<int>::BITS`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_shr<T>(lhs: T, rhs: T) -> T;

            /// Funnel Shifts vector left elementwise, with UB on overflow.
            ///
            /// Concatenates `a` and `b` elementwise (with `a` in the most significant half),
            /// creating a vector of the same length, but with each element being twice as
            /// wide. Then shift this vector left elementwise by `shift`, shifting in zeros,
            /// and extract the most significant half of each of the elements. If `a` and `b`
            /// are the same, this is equivalent to an elementwise rotate left operation.
            ///
            /// `T` must be a vector of integers.
            ///
            /// # Safety
            ///
            /// Each element of `shift` must be less than `<int>::BITS`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_funnel_shl<T>(a: T, b: T, shift: T) -> T;

            /// Funnel Shifts vector right elementwise, with UB on overflow.
            ///
            /// Concatenates `a` and `b` elementwise (with `a` in the most significant half),
            /// creating a vector of the same length, but with each element being twice as
            /// wide. Then shift this vector right elementwise by `shift`, shifting in zeros,
            /// and extract the least significant half of each of the elements. If `a` and `b`
            /// are the same, this is equivalent to an elementwise rotate right operation.
            ///
            /// `T` must be a vector of integers.
            ///
            /// # Safety
            ///
            /// Each element of `shift` must be less than `<int>::BITS`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_funnel_shr<T>(a: T, b: T, shift: T) -> T;

            /// Compute the carry-less product.
            ///
            /// This is similar to long multiplication except that the carry is discarded.
            ///
            /// This operation can be used to model multiplication in `GF(2)[X]`, the polynomial
            /// ring over `GF(2)`.
            ///
            /// `T` must be a vector of integers.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub unsafe fn simd_carryless_mul<T>(a: T, b: T) -> T;

            /// "And"s vectors elementwise.
            ///
            /// `T` must be a vector of integers.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_and<T>(x: T, y: T) -> T;

            /// "Ors" vectors elementwise.
            ///
            /// `T` must be a vector of integers.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_or<T>(x: T, y: T) -> T;

            /// "Exclusive ors" vectors elementwise.
            ///
            /// `T` must be a vector of integers.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_xor<T>(x: T, y: T) -> T;

            /// Numerically casts a vector, elementwise.
            ///
            /// `T` and `U` must be vectors of integers or floats, and must have the same length.
            ///
            /// When casting floats to integers, the result is truncated. Out-of-bounds result lead to UB.
            /// When casting integers to floats, the result is rounded.
            /// Otherwise, truncates or extends the value, maintaining the sign for signed integers.
            ///
            /// # Safety
            /// Casting from integer types is always safe.
            /// Casting between two float types is also always safe.
            ///
            /// Casting floats to integers truncates, following the same rules as `to_int_unchecked`.
            /// Specifically, each element must:
            /// * Not be `NaN`
            /// * Not be infinite
            /// * Be representable in the return type, after truncating off its fractional part
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_cast<T, U>(x: T) -> U;

            /// Numerically casts a vector, elementwise.
            ///
            /// `T` and `U` be a vectors of integers or floats, and must have the same length.
            ///
            /// Like `simd_cast`, but saturates float-to-integer conversions (NaN becomes 0).
            /// This matches regular `as` and is always safe.
            ///
            /// When casting floats to integers, the result is truncated.
            /// When casting integers to floats, the result is rounded.
            /// Otherwise, truncates or extends the value, maintaining the sign for signed integers.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_as<T, U>(x: T) -> U;

            /// Negates a vector elementwise.
            ///
            /// `T` must be a vector of integers or floats.
            /// For integers, wrapping arithmetic is used.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_neg<T>(x: T) -> T;

            /// Returns absolute value of a vector, elementwise.
            ///
            /// `T` must be a vector of floating-point primitive types.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_fabs<T>(x: T) -> T;

            /// Returns the minimum of two vectors, elementwise.
            ///
            /// `T` must be a vector of floating-point primitive types.
            ///
            /// This behaves like IEEE 754-2019 minimumNumber, *except* that it does not order signed
            /// zeros deterministically. In particular, for each vector lane:
            /// If one of the arguments is NaN (quiet or signaling), then the other argument is returned. If
            /// both arguments are NaN, returns NaN. If the inputs compare equal (such as for the case of `+0.0`
            /// and `-0.0`), either input may be returned non-deterministically.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_minimum_number_nsz<T>(x: T, y: T) -> T;

            /// Returns the maximum of two vectors, elementwise.
            ///
            /// `T` must be a vector of floating-point primitive types.
            ///
            /// This behaves like IEEE 754-2019 maximumNumber, *except* that it does not order signed
            /// zeros deterministically. In particular, for each vector lane:
            /// If one of the arguments is NaN (quiet or signaling), then the other argument is returned. If
            /// both arguments are NaN, returns NaN. If the inputs compare equal (such as for the case of `+0.0`
            /// and `-0.0`), either input may be returned non-deterministically.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_maximum_number_nsz<T>(x: T, y: T) -> T;

            /// Tests elementwise equality of two vectors.
            ///
            /// `T` must be a vector of integers or floats.
            ///
            /// `U` must be a vector of integers with the same number of elements and element size as `T`.
            ///
            /// Returns `0` for false and `!0` for true.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_eq<T, U>(x: T, y: T) -> U;

            /// Tests elementwise inequality equality of two vectors.
            ///
            /// `T` must be a vector of integers or floats.
            ///
            /// `U` must be a vector of integers with the same number of elements and element size as `T`.
            ///
            /// Returns `0` for false and `!0` for true.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_ne<T, U>(x: T, y: T) -> U;

            /// Tests if `x` is less than `y`, elementwise.
            ///
            /// `T` must be a vector of integers or floats.
            ///
            /// `U` must be a vector of integers with the same number of elements and element size as `T`.
            ///
            /// Returns `0` for false and `!0` for true.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_lt<T, U>(x: T, y: T) -> U;

            /// Tests if `x` is less than or equal to `y`, elementwise.
            ///
            /// `T` must be a vector of integers or floats.
            ///
            /// `U` must be a vector of integers with the same number of elements and element size as `T`.
            ///
            /// Returns `0` for false and `!0` for true.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_le<T, U>(x: T, y: T) -> U;

            /// Tests if `x` is greater than `y`, elementwise.
            ///
            /// `T` must be a vector of integers or floats.
            ///
            /// `U` must be a vector of integers with the same number of elements and element size as `T`.
            ///
            /// Returns `0` for false and `!0` for true.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_gt<T, U>(x: T, y: T) -> U;

            /// Tests if `x` is greater than or equal to `y`, elementwise.
            ///
            /// `T` must be a vector of integers or floats.
            ///
            /// `U` must be a vector of integers with the same number of elements and element size as `T`.
            ///
            /// Returns `0` for false and `!0` for true.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_ge<T, U>(x: T, y: T) -> U;

            /// Shuffles two vectors by const indices.
            ///
            /// `T` must be a vector.
            ///
            /// `U` must be a **const** vector of `u32`s. This means it must either refer to a named
            /// const or be given as an inline const expression (`const { ... }`).
            ///
            /// `V` must be a vector with the same element type as `T` and the same length as `U`.
            ///
            /// Returns a new vector such that element `i` is selected from `xy[idx[i]]`, where `xy`
            /// is the concatenation of `x` and `y`. It is a compile-time error if `idx[i]` is out-of-bounds
            /// of `xy`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_shuffle<T, U, V>(x: T, y: T, idx: U) -> V;

            /// Reads a vector of pointers.
            ///
            /// `T` must be a vector.
            ///
            /// `U` must be a vector of pointers to the element type of `T`, with the same length as `T`.
            ///
            /// `V` must be a vector of integers with the same length as `T` (but any element size).
            ///
            /// For each pointer in `ptr`, if the corresponding value in `mask` is `!0`, read the pointer.
            /// Otherwise if the corresponding value in `mask` is `0`, return the corresponding value from
            /// `val`.
            ///
            /// # Safety
            /// Unmasked values in `T` must be readable as if by `<ptr>::read` (e.g. aligned to the element
            /// type).
            ///
            /// `mask` must only contain `0` or `!0` values.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_gather<T, U, V>(val: T, ptr: U, mask: V) -> T;

            /// Writes to a vector of pointers.
            ///
            /// `T` must be a vector.
            ///
            /// `U` must be a vector of pointers to the element type of `T`, with the same length as `T`.
            ///
            /// `V` must be a vector of integers with the same length as `T` (but any element size).
            ///
            /// For each pointer in `ptr`, if the corresponding value in `mask` is `!0`, write the
            /// corresponding value in `val` to the pointer.
            /// Otherwise if the corresponding value in `mask` is `0`, do nothing.
            ///
            /// The stores happen in left-to-right order.
            /// (This is relevant in case two of the stores overlap.)
            ///
            /// # Safety
            /// Unmasked values in `T` must be writeable as if by `<ptr>::write` (e.g. aligned to the element
            /// type).
            ///
            /// `mask` must only contain `0` or `!0` values.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_scatter<T, U, V>(val: T, ptr: U, mask: V);

            /// A type for alignment options for SIMD masked load/store intrinsics.
            // #[derive(Debug, ConstParamTy, PartialEq, Eq)]
            #[derive(Debug, PartialEq, Eq)]
            pub enum SimdAlign
            {
                Unaligned = 0,
                /// The pointer must be aligned to the element type of the SIMD vector
                Element = 1,
                /// The pointer must be aligned to the SIMD vector type
                Vector = 2,
            }

            /// Reads a vector of pointers.
            ///
            /// `T` must be a vector.
            ///
            /// `U` must be a pointer to the element type of `T`
            ///
            /// `V` must be a vector of integers with the same length as `T` (but any element size).
            ///
            /// For each element, if the corresponding value in `mask` is `!0`, read the corresponding
            /// pointer offset from `ptr`.
            /// The first element is loaded from `ptr`, the second from `ptr.wrapping_offset(1)` and so on.
            /// Otherwise if the corresponding value in `mask` is `0`, return the corresponding value from
            /// `val`.
            ///
            /// # Safety
            /// `ptr` must be aligned according to the `ALIGN` parameter, see [`SimdAlign`] for details.
            ///
            /// `mask` must only contain `0` or `!0` values.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_masked_load<V, U, T, const ALIGN: SimdAlign>(mask: V, ptr: U, val: T)
            -> T;

            /// Writes to a vector of pointers.
            ///
            /// `T` must be a vector.
            ///
            /// `U` must be a pointer to the element type of `T`
            ///
            /// `V` must be a vector of integers with the same length as `T` (but any element size).
            ///
            /// For each element, if the corresponding value in `mask` is `!0`, write the corresponding
            /// value in `val` to the pointer offset from `ptr`.
            /// The first element is written to `ptr`, the second to `ptr.wrapping_offset(1)` and so on.
            /// Otherwise if the corresponding value in `mask` is `0`, do nothing.
            ///
            /// # Safety
            /// `ptr` must be aligned according to the `ALIGN` parameter, see [`SimdAlign`] for details.
            ///
            /// `mask` must only contain `0` or `!0` values.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_masked_store<V, U, T, const ALIGN: SimdAlign>(mask: V, ptr: U, val: T);

            /// Adds two simd vectors elementwise, with saturation.
            ///
            /// `T` must be a vector of integer primitive types.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_saturating_add<T>(x: T, y: T) -> T;

            /// Subtracts two simd vectors elementwise, with saturation.
            ///
            /// `T` must be a vector of integer primitive types.
            ///
            /// Subtract `rhs` from `lhs`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_saturating_sub<T>(lhs: T, rhs: T) -> T;

            /// Adds elements within a vector from left to right.
            ///
            /// `T` must be a vector of integers or floats.
            ///
            /// `U` must be the element type of `T`.
            ///
            /// Starting with the value `y`, add the elements of `x` and accumulate.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_reduce_add_ordered<T, U>(x: T, y: U) -> U;

            /// Adds elements within a vector in arbitrary order. May also be re-associated with
            /// unordered additions on the inputs/outputs.
            ///
            /// `T` must be a vector of integers or floats.
            ///
            /// `U` must be the element type of `T`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub unsafe fn simd_reduce_add_unordered<T, U>(x: T) -> U;

            /// Multiplies elements within a vector from left to right.
            ///
            /// `T` must be a vector of integers or floats.
            ///
            /// `U` must be the element type of `T`.
            ///
            /// Starting with the value `y`, multiply the elements of `x` and accumulate.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_reduce_mul_ordered<T, U>(x: T, y: U) -> U;

            /// Multiplies elements within a vector in arbitrary order. May also be re-associated with
            /// unordered additions on the inputs/outputs.
            ///
            /// `T` must be a vector of integers or floats.
            ///
            /// `U` must be the element type of `T`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub unsafe fn simd_reduce_mul_unordered<T, U>(x: T) -> U;

            /// Checks if all mask values are true.
            ///
            /// `T` must be a vector of integers.
            ///
            /// # Safety
            /// `x` must contain only `0` or `!0`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_reduce_all<T>(x: T) -> bool;

            /// Checks if any mask value is true.
            ///
            /// `T` must be a vector of integers.
            ///
            /// # Safety
            /// `x` must contain only `0` or `!0`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_reduce_any<T>(x: T) -> bool;

            /// Returns the maximum element of a vector.
            ///
            /// `T` must be a vector of integers.
            ///
            /// `U` must be the element type of `T`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_reduce_max<T, U>(x: T) -> U;

            /// Returns the minimum element of a vector.
            ///
            /// `T` must be a vector of integers.
            ///
            /// `U` must be the element type of `T`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_reduce_min<T, U>(x: T) -> U;

            /// Logical "and"s all elements together.
            ///
            /// `T` must be a vector of integers.
            ///
            /// `U` must be the element type of `T`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_reduce_and<T, U>(x: T) -> U;

            /// Logical "ors" all elements together.
            ///
            /// `T` must be a vector of integers.
            ///
            /// `U` must be the element type of `T`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_reduce_or<T, U>(x: T) -> U;

            /// Logical "exclusive ors" all elements together.
            ///
            /// `T` must be a vector of integers.
            ///
            /// `U` must be the element type of `T`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_reduce_xor<T, U>(x: T) -> U;

            /// Truncates an integer vector to a bitmask.
            ///
            /// `T` must be an integer vector.
            ///
            /// `U` must be either the smallest unsigned integer with at least as many bits as the length
            /// of `T`, or the smallest array of `u8` with at least as many bits as the length of `T`.
            ///
            /// Each element is truncated to a single bit and packed into the result.
            ///
            /// No matter whether the output is an array or an unsigned integer, it is treated as a single
            /// contiguous list of bits. The bitmask is always packed on the least-significant side of the
            /// output, and padded with 0s in the most-significant bits. The order of the bits depends on
            /// endianness:
            ///
            /// * On little endian, the least significant bit corresponds to the first vector element.
            /// * On big endian, the least significant bit corresponds to the last vector element.
            ///
            /// For example, `[!0, 0, !0, !0]` packs to
            /// - `0b1101u8` or `[0b1101]` on little endian, and
            /// - `0b1011u8` or `[0b1011]` on big endian.
            ///
            /// To consider a larger example,
            /// `[!0, 0, 0, 0, 0, 0, 0, 0, !0, !0, 0, 0, 0, 0, !0, 0]` packs to
            /// - `0b0100001100000001u16` or `[0b00000001, 0b01000011]` on little endian, and
            /// - `0b1000000011000010u16` or `[0b10000000, 0b11000010]` on big endian.
            ///
            /// And finally, a non-power-of-2 example with multiple bytes:
            /// `[!0, !0, 0, !0, 0, 0, !0, 0, !0, 0]` packs to
            /// - `0b0101001011u16` or `[0b01001011, 0b01]` on little endian, and
            /// - `0b1101001010u16` or `[0b11, 0b01001010]` on big endian.
            ///
            /// # Safety
            /// `x` must contain only `0` and `!0`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_bitmask<T, U>(x: T) -> U;

            /// Selects elements from a mask.
            ///
            /// `T` must be a vector.
            ///
            /// `M` must be an integer vector with the same length as `T` (but any element size).
            ///
            /// For each element, if the corresponding value in `mask` is `!0`, select the element from
            /// `if_true`.  If the corresponding value in `mask` is `0`, select the element from
            /// `if_false`.
            ///
            /// # Safety
            /// `mask` must only contain `0` and `!0`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_select<M, T>(mask: M, if_true: T, if_false: T) -> T;

            /// Selects elements from a bitmask.
            ///
            /// `M` must be an unsigned integer or array of `u8`, matching `simd_bitmask`.
            ///
            /// `T` must be a vector.
            ///
            /// For each element, if the bit in `mask` is `1`, select the element from
            /// `if_true`.  If the corresponding bit in `mask` is `0`, select the element from
            /// `if_false`.
            /// The remaining bits of the mask are ignored.
            ///
            /// The bitmask bit order matches `simd_bitmask`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_select_bitmask<M, T>(m: M, yes: T, no: T) -> T;

            /// Calculates the offset from a pointer vector elementwise, potentially
            /// wrapping.
            ///
            /// `T` must be a vector of pointers.
            ///
            /// `U` must be a vector of `isize` or `usize` with the same number of elements as `T`.
            ///
            /// Operates as if by `<ptr>::wrapping_offset`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_arith_offset<T, U>(ptr: T, offset: U) -> T;

            /// Casts a vector of pointers.
            ///
            /// `T` and `U` must be vectors of pointers with the same number of elements.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_cast_ptr<T, U>(ptr: T) -> U;

            /// Exposes a vector of pointers as a vector of addresses.
            ///
            /// `T` must be a vector of pointers.
            ///
            /// `U` must be a vector of `usize` with the same length as `T`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub unsafe fn simd_expose_provenance<T, U>(ptr: T) -> U;

            /// Creates a vector of pointers from a vector of addresses.
            ///
            /// `T` must be a vector of `usize`.
            ///
            /// `U` must be a vector of pointers, with the same length as `T`.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_with_exposed_provenance<T, U>(addr: T) -> U;

            /// Swaps bytes of each element.
            ///
            /// `T` must be a vector of integers.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_bswap<T>(x: T) -> T;

            /// Reverses bits of each element.
            ///
            /// `T` must be a vector of integers.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_bitreverse<T>(x: T) -> T;

            /// Counts the leading zeros of each element.
            ///
            /// `T` must be a vector of integers.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_ctlz<T>(x: T) -> T;

            /// Counts the number of ones in each element.
            ///
            /// `T` must be a vector of integers.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_ctpop<T>(x: T) -> T;

            /// Counts the trailing zeros of each element.
            ///
            /// `T` must be a vector of integers.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_cttz<T>(x: T) -> T;

            /// Rounds up each element to the next highest integer-valued float.
            ///
            /// `T` must be a vector of floats.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_ceil<T>(x: T) -> T;

            /// Rounds down each element to the next lowest integer-valued float.
            ///
            /// `T` must be a vector of floats.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_floor<T>(x: T) -> T;

            /// Rounds each element to the closest integer-valued float.
            /// Ties are resolved by rounding away from 0.
            ///
            /// `T` must be a vector of floats.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_round<T>(x: T) -> T;

            /// Rounds each element to the closest integer-valued float.
            /// Ties are resolved by rounding to the number with an even least significant digit
            ///
            /// `T` must be a vector of floats.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_round_ties_even<T>(x: T) -> T;

            /// Returns the integer part of each element as an integer-valued float.
            /// In other words, non-integer values are truncated towards zero.
            ///
            /// `T` must be a vector of floats.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_trunc<T>(x: T) -> T;

            /// Takes the square root of each element.
            ///
            /// `T` must be a vector of floats.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub unsafe fn simd_fsqrt<T>(x: T) -> T;

            /// Computes `(x*y) + z` for each element, but without any intermediate rounding.
            ///
            /// `T` must be a vector of floats.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_fma<T>(x: T, y: T, z: T) -> T;

            /// Computes `(x*y) + z` for each element, non-deterministically executing either
            /// a fused multiply-add or two operations with rounding of the intermediate result.
            ///
            /// The operation is fused if the code generator determines that target instruction
            /// set has support for a fused operation, and that the fused operation is more efficient
            /// than the equivalent, separate pair of mul and add instructions. It is unspecified
            /// whether or not a fused operation is selected, and that may depend on optimization
            /// level and context, for example. It may even be the case that some SIMD lanes get fused
            /// and others do not.
            ///
            /// `T` must be a vector of floats.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub const unsafe fn simd_relaxed_fma<T>(x: T, y: T, z: T) -> T;

            // Computes the sine of each element.
            ///
            /// `T` must be a vector of floats.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub unsafe fn simd_fsin<T>(a: T) -> T;

            // Computes the cosine of each element.
            ///
            /// `T` must be a vector of floats.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub unsafe fn simd_fcos<T>(a: T) -> T;

            // Computes the exponential function of each element.
            ///
            /// `T` must be a vector of floats.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub unsafe fn simd_fexp<T>(a: T) -> T;

            // Computes 2 raised to the power of each element.
            ///
            /// `T` must be a vector of floats.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub unsafe fn simd_fexp2<T>(a: T) -> T;

            // Computes the base 10 logarithm of each element.
            ///
            /// `T` must be a vector of floats.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub unsafe fn simd_flog10<T>(a: T) -> T;

            // Computes the base 2 logarithm of each element.
            ///
            /// `T` must be a vector of floats.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub unsafe fn simd_flog2<T>(a: T) -> T;

            // Computes the natural logarithm of each element.
            ///
            /// `T` must be a vector of floats.
            #[rustc_intrinsic]
            #[rustc_nounwind]
            pub unsafe fn simd_flog<T>(a: T) -> T;
    }    

}

pub mod is
{
    use crate::
    {
        *,
    };

    //pub fn is_builtin(s: &str) -> bool
    pub fn api(s: &str) -> bool
    {
        unsafe
            {
                api::API.contains( &s )
            }
    }

    //pub fn is_arithmetic(line: &str) -> bool
    pub fn arithmetic(line: &str) -> bool
    {
        if !regex::contains(line, r"[0-9]+") { return false; }

        if !regex::contains(line, r"\+|\-|\*|/|\^") { return false; }

        regex::contains(line, r"^[ 0-9\.\(\)\+\-\*/\^]+[\.0-9 \)]$")
    }

    //pub fn is_env(line: &str) -> bool
    pub fn environment(line: &str) -> bool { regex::contains(line, r"^[a-zA-Z_][a-zA-Z0-9_]*=.*$") }

    //pub fn is_shell_altering_command(line: &str) -> bool {
    pub fn shell_altering_command(line: &str) -> bool
    {
        let line = line.trim();

        if regex::contains(line, r"^[A-Za-z_][A-Za-z0-9_]*=.*$") { return true; }

        line.starts_with("alias ")
        || line.starts_with("export ")
        || line.starts_with("unalias ")
        || line.starts_with("unset ")
        || line.starts_with("source ")
    }
    //pub fn is_signal_handler_enabled() -> bool
    pub fn signal_handler_enabled() -> bool { env::var("PLS_ENABLE_SIG_HANDLER").is_ok_and(|x| x == "1") }
}

pub mod lines
{
    use crate::
    {
        tokens::{ Tokens },
        *,
    };

    #[derive(Debug)]
    pub struct LineInfo
    {
        // e.g. echo 'foo
        // is not a completed line, need to turn to multiple-line mode.
        pub tokens: tokens::Tokens,
        pub is_complete: bool,
    }

    impl LineInfo
    {
        pub const fn new() -> Self
        {
            Self
            {
                tokens: Tokens::new(),
                is_complete: false,
            }
        }
        pub fn create(tokens: tokens::Tokens) -> Self {
            LineInfo {
                tokens,
                is_complete: true,
            }
        }
    }
}

pub mod marker
{
    pub use std::marker::{ * };
}


pub mod mem
{
    pub use std::mem::{ * };
}

pub mod now
{
    use crate::
    {
        *,
    };
}

pub mod ops
{
    pub use std::ops::{ * };
}

pub mod panic
{
    pub use std::panic::{ * };
}

pub mod path
{
    pub use std::path::{ * };
    use crate::
    {
        collections::{ HashSet },
        *,
    };

    pub const DRIVE:&str = r#"C:"#;
    pub const OPERATING_SYSTEM:&str = r#"\\WINDOWS"#;
    pub const SYSTEM:&str = r#"\\SYSTEM32"#;

    pub fn initialize_environment()
    {
        unsafe
        {
            let mut all_paths: HashSet<PathBuf> = HashSet::new();
            for x in
                [
                    format!( r#"{}{}"#, DRIVE, OPERATING_SYSTEM ),
                    format!( r#"{}{}{}"#, DRIVE, OPERATING_SYSTEM, SYSTEM ),
                    format!( r#"{}{}{}\\Drivers"#, DRIVE, OPERATING_SYSTEM, SYSTEM ),
                    format!( r#"{}{}{}\\Drivers\\DriverData"#, DRIVE, OPERATING_SYSTEM, SYSTEM ),
                    format!( r#"{}{}{}\\OpenSSH"#, DRIVE, OPERATING_SYSTEM, SYSTEM ),
                    format!( r#"{}{}{}\\Wbem"#, DRIVE, OPERATING_SYSTEM, SYSTEM ),
                    format!( r#"{}{}{}\\WindowsPowerShell"#, DRIVE, OPERATING_SYSTEM, SYSTEM ),
                    format!( r#"{}{}{}\\WindowsPowerShell\\v1.0"#, DRIVE, OPERATING_SYSTEM, SYSTEM ),
            ]
            {
            let path_buf = PathBuf::from(x);
            if path_buf.exists() {
            all_paths.insert(path_buf);
            }
            }

            if let Ok(env_path) = env::var("PATH") {
                for one_path in env::split_paths(&env_path) {
                    if !all_paths.contains(&one_path) {
                        all_paths.insert(one_path);
                    }
                }
            }
            let path_var = env::join_paths(all_paths).unwrap_or_default();
            env::set_var("PATH", path_var);
        }
    }

    pub fn is_shell_altering_command(line: &str) -> bool {
        let line = line.trim();
        if regex::contains(line, r"^[A-Za-z_][A-Za-z0-9_]*=.*$") {
            return true;
        }
        line.starts_with("alias ")
            || line.starts_with("export ")
            || line.starts_with("unalias ")
            || line.starts_with("unset ")
            || line.starts_with("source ")
    }
}

pub mod parses
{
    use crate::
    {
        *,
    };

    pub mod line
    {
        use crate::
        {
            lines::{ LineInfo },
            *,
        };

        pub fn parses(line: &str) -> LineInfo
        {
            let parsed = LineInfo::new();
            let mut result = Vec::new();

            if is::arithmetic( line )
            {
                for x in line.split(' ')
                {
                    result.push((String::from(""), x.to_string()));
                }

                return LineInfo::create( result );
            }

            parsed
        }
    }
}

pub mod pattern
{
    use crate::{*};

    macro_rules! pattern_methods {
        ($a:lifetime, $t:ty, $pmap:expr, $smap:expr) => {
            type Searcher<$a> = $t;

            #[inline]
            fn into_searcher<$a>(self, haystack: &$a str) -> $t {
                ($smap)(($pmap)(self).into_searcher(haystack))
            }

            #[inline]
            fn is_contained_in<$a>(self, haystack: &$a str) -> bool {
                ($pmap)(self).is_contained_in(haystack)
            }

            #[inline]
            fn is_prefix_of<$a>(self, haystack: &$a str) -> bool {
                ($pmap)(self).is_prefix_of(haystack)
            }

            #[inline]
            fn strip_prefix_of<$a>(self, haystack: &$a str) -> Option<&$a str> {
                ($pmap)(self).strip_prefix_of(haystack)
            }

            #[inline]
            fn is_suffix_of<$a>(self, haystack: &$a str) -> bool
            where
                $t: ReverseSearcher<$a>,
            {
                ($pmap)(self).is_suffix_of(haystack)
            }

            #[inline]
            fn strip_suffix_of<$a>(self, haystack: &$a str) -> Option<&$a str>
            where
                $t: ReverseSearcher<$a>,
            {
                ($pmap)(self).strip_suffix_of(haystack)
            }
        };
    }

    macro_rules! searcher_methods {
        (forward) => {
            #[inline]
            fn haystack(&self) -> &'a str {
                self.0.haystack()
            }
            #[inline]
            fn next(&mut self) -> SearchStep {
                self.0.next()
            }
            #[inline]
            fn next_match(&mut self) -> Option<(usize, usize)> {
                self.0.next_match()
            }
            #[inline]
            fn next_reject(&mut self) -> Option<(usize, usize)> {
                self.0.next_reject()
            }
        };
        (reverse) => {
            #[inline]
            fn next_back(&mut self) -> SearchStep {
                self.0.next_back()
            }
            #[inline]
            fn next_match_back(&mut self) -> Option<(usize, usize)> {
                self.0.next_match_back()
            }
            #[inline]
            fn next_reject_back(&mut self) -> Option<(usize, usize)> {
                self.0.next_reject_back()
            }
        };
    }
    /// A string pattern.
    pub trait Pattern: Sized {
        /// Associated searcher for this pattern
        type Searcher<'a>: Searcher<'a>;

        /// Constructs the associated searcher from
        /// `self` and the `haystack` to search in.
        fn into_searcher(self, haystack: &str) -> Self::Searcher<'_>;

        /// Checks whether the pattern matches anywhere in the haystack
        #[inline]
        fn is_contained_in(self, haystack: &str) -> bool {
            self.into_searcher(haystack).next_match().is_some()
        }

        /// Checks whether the pattern matches at the front of the haystack
        #[inline]
        fn is_prefix_of(self, haystack: &str) -> bool {
            matches!(self.into_searcher(haystack).next(), SearchStep::Match(0, _))
        }

        /// Checks whether the pattern matches at the back of the haystack
        #[inline]
        fn is_suffix_of<'a>(self, haystack: &'a str) -> bool
        where
            Self::Searcher<'a>: ReverseSearcher<'a>,
        {
            matches!(self.into_searcher(haystack).next_back(), SearchStep::Match(_, j) if haystack.len() == j)
        }

        /// Removes the pattern from the front of haystack, if it matches.
        #[inline]
        fn strip_prefix_of(self, haystack: &str) -> Option<&str> {
            if let SearchStep::Match(start, len) = self.into_searcher(haystack).next() {
                debug_assert_eq!(
                    start, 0,
                    "The first search step from Searcher \
                    must include the first character"
                );
                // SAFETY: `Searcher` is known to return valid indices.
                unsafe { Some(haystack.get_unchecked(len..)) }
            } else {
                None
            }
        }

        /// Removes the pattern from the back of haystack, if it matches.
        #[inline]
        fn strip_suffix_of<'a>(self, haystack: &'a str) -> Option<&'a str>
        where
            Self::Searcher<'a>: ReverseSearcher<'a>,
        {
            if let SearchStep::Match(start, end) = self.into_searcher(haystack).next_back() {
                debug_assert_eq!(
                    end,
                    haystack.len(),
                    "The first search step from ReverseSearcher \
                    must include the last character"
                );
                // SAFETY: `Searcher` is known to return valid indices.
                unsafe { Some(haystack.get_unchecked(..start)) }
            } else {
                None
            }
        }

        /// Returns the pattern as UTF-8 if possible.
        fn as_utf8_pattern(&self) -> Option<Utf8Pattern<'_>> {
            None
        }
    }
    /// Result of calling [`Pattern::as_utf8_pattern()`].
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Utf8Pattern<'a> {
        /// Type returned by String and str types.
        StringPattern(&'a str),
        /// Type returned by char types.
        CharPattern(char),
    }

    // Searcher

    /// Result of calling [`Searcher::next()`] or [`ReverseSearcher::next_back()`].
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum SearchStep {
        /// Expresses that a match of the pattern has been found at `haystack[a..b]`.
        Match(usize, usize),
        /// Expresses that `haystack[a..b]` has been rejected as a possible match of the pattern.
        Reject(usize, usize),
        /// Expresses that every byte of the haystack has been visited, ending the iteration.
        Done,
    }

    /// A searcher for a string pattern.
    pub unsafe trait Searcher<'a> {
        /// Getter for the underlying string to be searched in
        fn haystack(&self) -> &'a str;

        /// Performs the next search step starting from the front.
        fn next(&mut self) -> SearchStep;

        /// Finds the next [`Match`][SearchStep::Match] result. See [`next()`][Searcher::next].
        #[inline]
        fn next_match(&mut self) -> Option<(usize, usize)> {
            loop {
                match self.next() {
                    SearchStep::Match(a, b) => return Some((a, b)),
                    SearchStep::Done => return None,
                    _ => continue,
                }
            }
        }

        /// Finds the next [`Reject`][SearchStep::Reject] result. See [`next()`][Searcher::next]
        /// and [`next_match()`][Searcher::next_match].
        #[inline]
        fn next_reject(&mut self) -> Option<(usize, usize)> {
            loop {
                match self.next() {
                    SearchStep::Reject(a, b) => return Some((a, b)),
                    SearchStep::Done => return None,
                    _ => continue,
                }
            }
        }
    }

    /// A reverse searcher for a string pattern.
    pub unsafe trait ReverseSearcher<'a>: Searcher<'a> {
        /// Performs the next search step starting from the back.
        fn next_back(&mut self) -> SearchStep;

        /// Finds the next [`Match`][SearchStep::Match] result.
        #[inline]
        fn next_match_back(&mut self) -> Option<(usize, usize)> {
            loop {
                match self.next_back() {
                    SearchStep::Match(a, b) => return Some((a, b)),
                    SearchStep::Done => return None,
                    _ => continue,
                }
            }
        }

        /// Finds the next [`Reject`][SearchStep::Reject] result.
        #[inline]
        fn next_reject_back(&mut self) -> Option<(usize, usize)> {
            loop {
                match self.next_back() {
                    SearchStep::Reject(a, b) => return Some((a, b)),
                    SearchStep::Done => return None,
                    _ => continue,
                }
            }
        }
    }

    /// A marker trait to express that a [`ReverseSearcher`] can be used for a [`DoubleEndedIterator`] implementation.
    pub trait DoubleEndedSearcher<'a>: ReverseSearcher<'a> {}
    
    #[derive(Clone, Debug)]
    pub struct CharSearcher<'a> {
        haystack: &'a str,
        finger: usize,
        finger_back: usize,
        needle: char,
        utf8_size: u8,
        utf8_encoded: [u8; 4],
    }

    impl CharSearcher<'_> {
        fn utf8_size(&self) -> usize {
            self.utf8_size.into()
        }
    }

    unsafe impl<'a> Searcher<'a> for CharSearcher<'a> {
        #[inline]
        fn haystack(&self) -> &'a str {
            self.haystack
        }
        #[inline]
        fn next(&mut self) -> SearchStep {
            let old_finger = self.finger;
            let slice = unsafe { self.haystack.get_unchecked(old_finger..self.finger_back) };
            let mut iter = slice.chars();
            let old_len = iter.iter.len();
            if let Some(ch) = iter.next() {
                self.finger += old_len - iter.iter.len();
                if ch == self.needle {
                    SearchStep::Match(old_finger, self.finger)
                } else {
                    SearchStep::Reject(old_finger, self.finger)
                }
            } else {
                SearchStep::Done
            }
        }
        #[inline]
        fn next_match(&mut self) -> Option<(usize, usize)> {
            loop {
                // get the haystack after the last character found
                let bytes = self.haystack.as_bytes().get(self.finger..self.finger_back)?;
                let last_byte = unsafe { *self.utf8_encoded.get_unchecked(self.utf8_size() - 1) };
                if let Some(index) = memchr::memchr(last_byte, bytes) {
                    self.finger += index + 1;
                    if self.finger >= self.utf8_size() {
                        let found_char = self.finger - self.utf8_size();
                        if let Some(slice) = self.haystack.as_bytes().get(found_char..self.finger) {
                            if slice == &self.utf8_encoded[0..self.utf8_size()] {
                                return Some((found_char, self.finger));
                            }
                        }
                    }
                } else {
                    self.finger = self.finger_back;
                    return None;
                }
            }
        }
    }

    unsafe impl<'a> ReverseSearcher<'a> for CharSearcher<'a> 
    {
        #[inline]
        fn next_back(&mut self) -> SearchStep {
            let old_finger = self.finger_back;
            let slice = unsafe { self.haystack.get_unchecked(self.finger..old_finger) };
            let mut iter = slice.chars();
            let old_len = iter.iter.len();
            if let Some(ch) = iter.next_back() {
                self.finger_back -= old_len - iter.iter.len();
                if ch == self.needle {
                    SearchStep::Match(self.finger_back, old_finger)
                } else {
                    SearchStep::Reject(self.finger_back, old_finger)
                }
            } else {
                SearchStep::Done
            }
        }
        #[inline]
        fn next_match_back(&mut self) -> Option<(usize, usize)> {
            let haystack = self.haystack.as_bytes();
            loop {
                let bytes = haystack.get(self.finger..self.finger_back)?;
                let last_byte = unsafe { *self.utf8_encoded.get_unchecked(self.utf8_size() - 1) };
                if let Some(index) = memchr::memrchr(last_byte, bytes) {
                    let index = self.finger + index;
                    let shift = self.utf8_size() - 1;
                    if index >= shift {
                        let found_char = index - shift;
                        if let Some(slice) = haystack.get(found_char..(found_char + self.utf8_size())) {
                            if slice == &self.utf8_encoded[0..self.utf8_size()] {
                                self.finger_back = found_char;
                                return Some((self.finger_back, self.finger_back + self.utf8_size()));
                            }
                        }
                    }
                    
                    self.finger_back = index;
                } else {
                    self.finger_back = self.finger;
                    return None;
                }
            }
        }
    }

    impl<'a> DoubleEndedSearcher<'a> for CharSearcher<'a> {}

    /// Searches for chars that are equal to a given [`char`].
    impl Pattern for char {
        type Searcher<'a> = CharSearcher<'a>;

        #[inline]
        fn into_searcher<'a>(self, haystack: &'a str) -> Self::Searcher<'a> {
            let mut utf8_encoded = [0; char::MAX_LEN_UTF8];
            let utf8_size = self
                .encode_utf8(&mut utf8_encoded)
                .len()
                .try_into()
                .expect("char len should be less than 255");

            CharSearcher {
                haystack,
                finger: 0,
                finger_back: haystack.len(),
                needle: self,
                utf8_size,
                utf8_encoded,
            }
        }

        #[inline]
        fn is_contained_in(self, haystack: &str) -> bool {
            if (self as u32) < 128 {
                haystack.as_bytes().contains(&(self as u8))
            } else {
                let mut buffer = [0u8; 4];
                self.encode_utf8(&mut buffer).is_contained_in(haystack)
            }
        }

        #[inline]
        fn is_prefix_of(self, haystack: &str) -> bool {
            self.encode_utf8(&mut [0u8; 4]).is_prefix_of(haystack)
        }

        #[inline]
        fn strip_prefix_of(self, haystack: &str) -> Option<&str> {
            self.encode_utf8(&mut [0u8; 4]).strip_prefix_of(haystack)
        }

        #[inline]
        fn is_suffix_of<'a>(self, haystack: &'a str) -> bool
        where
            Self::Searcher<'a>: ReverseSearcher<'a>,
        {
            self.encode_utf8(&mut [0u8; 4]).is_suffix_of(haystack)
        }

        #[inline]
        fn strip_suffix_of<'a>(self, haystack: &'a str) -> Option<&'a str>
        where
            Self::Searcher<'a>: ReverseSearcher<'a>,
        {
            self.encode_utf8(&mut [0u8; 4]).strip_suffix_of(haystack)
        }

        #[inline]
        fn as_utf8_pattern(&self) -> Option<Utf8Pattern<'_>> {
            Some(Utf8Pattern::CharPattern(*self))
        }
    }

    /////////////////////////////////////////////////////////////////////////////
    // Impl for a MultiCharEq wrapper
    /////////////////////////////////////////////////////////////////////////////

    #[doc(hidden)]
    trait MultiCharEq {
        fn matches(&mut self, c: char) -> bool;
    }

    impl<F> MultiCharEq for F
    where
        F: FnMut(char) -> bool,
    {
        #[inline]
        fn matches(&mut self, c: char) -> bool {
            (*self)(c)
        }
    }

    impl<const N: usize> MultiCharEq for [char; N] {
        #[inline]
        fn matches(&mut self, c: char) -> bool {
            self.contains(&c)
        }
    }

    impl<const N: usize> MultiCharEq for &[char; N] {
        #[inline]
        fn matches(&mut self, c: char) -> bool {
            self.contains(&c)
        }
    }

    impl MultiCharEq for &[char] {
        #[inline]
        fn matches(&mut self, c: char) -> bool {
            self.contains(&c)
        }
    }

    struct MultiCharEqPattern<C: MultiCharEq>(C);

    #[derive(Clone, Debug)]
    struct MultiCharEqSearcher<'a, C: MultiCharEq> {
        char_eq: C,
        haystack: &'a str,
        char_indices: str::CharIndices<'a>,
    }

    impl<C: MultiCharEq> Pattern for MultiCharEqPattern<C> {
        type Searcher<'a> = MultiCharEqSearcher<'a, C>;

        #[inline]
        fn into_searcher(self, haystack: &str) -> MultiCharEqSearcher<'_, C> {
            MultiCharEqSearcher { haystack, char_eq: self.0, char_indices: haystack.char_indices() }
        }
    }

    unsafe impl<'a, C: MultiCharEq> Searcher<'a> for MultiCharEqSearcher<'a, C> {
        #[inline]
        fn haystack(&self) -> &'a str {
            self.haystack
        }

        #[inline]
        fn next(&mut self) -> SearchStep {
            let s = &mut self.char_indices;
            let pre_len = s.iter.iter.len();
            if let Some((i, c)) = s.next() {
                let len = s.iter.iter.len();
                let char_len = pre_len - len;
                if self.char_eq.matches(c) {
                    return SearchStep::Match(i, i + char_len);
                } else {
                    return SearchStep::Reject(i, i + char_len);
                }
            }
            SearchStep::Done
        }
    }

    unsafe impl<'a, C: MultiCharEq> ReverseSearcher<'a> for MultiCharEqSearcher<'a, C> {
        #[inline]
        fn next_back(&mut self) -> SearchStep {
            let s = &mut self.char_indices;
            let pre_len = s.iter.iter.len();
            if let Some((i, c)) = s.next_back() {
                let len = s.iter.iter.len();
                let char_len = pre_len - len;
                if self.char_eq.matches(c) {
                    return SearchStep::Match(i, i + char_len);
                } else {
                    return SearchStep::Reject(i, i + char_len);
                }
            }
            SearchStep::Done
        }
    }

    impl<'a, C: MultiCharEq> DoubleEndedSearcher<'a> for MultiCharEqSearcher<'a, C> {}

    /// Associated type for `<[char; N] as Pattern>::Searcher<'a>`.
    #[derive(Clone, Debug)]
    pub struct CharArraySearcher<'a, const N: usize>(
        <MultiCharEqPattern<[char; N]> as Pattern>::Searcher<'a>,
    );

    /// Associated type for `<&[char; N] as Pattern>::Searcher<'a>`.
    #[derive(Clone, Debug)]
    pub struct CharArrayRefSearcher<'a, 'b, const N: usize>(
        <MultiCharEqPattern<&'b [char; N]> as Pattern>::Searcher<'a>,
    );

    /// Searches for chars that are equal to any of the [`char`]s in the array.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!("Hello world".find(['o', 'l']), Some(2));
    /// assert_eq!("Hello world".find(['h', 'w']), Some(6));
    /// ```
    impl<const N: usize> Pattern for [char; N] {
        pattern_methods!('a, CharArraySearcher<'a, N>, MultiCharEqPattern, CharArraySearcher);
    }

    unsafe impl<'a, const N: usize> Searcher<'a> for CharArraySearcher<'a, N> {
        searcher_methods!(forward);
    }

    unsafe impl<'a, const N: usize> ReverseSearcher<'a> for CharArraySearcher<'a, N> {
        searcher_methods!(reverse);
    }

    impl<'a, const N: usize> DoubleEndedSearcher<'a> for CharArraySearcher<'a, N> {}

    /// Searches for chars that are equal to any of the [`char`]s in the array.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!("Hello world".find(&['o', 'l']), Some(2));
    /// assert_eq!("Hello world".find(&['h', 'w']), Some(6));
    /// ```
    impl<'b, const N: usize> Pattern for &'b [char; N] {
        pattern_methods!('a, CharArrayRefSearcher<'a, 'b, N>, MultiCharEqPattern, CharArrayRefSearcher);
    }

    unsafe impl<'a, 'b, const N: usize> Searcher<'a> for CharArrayRefSearcher<'a, 'b, N> {
        searcher_methods!(forward);
    }

    unsafe impl<'a, 'b, const N: usize> ReverseSearcher<'a> for CharArrayRefSearcher<'a, 'b, N> {
        searcher_methods!(reverse);
    }

    impl<'a, 'b, const N: usize> DoubleEndedSearcher<'a> for CharArrayRefSearcher<'a, 'b, N> {}

    /////////////////////////////////////////////////////////////////////////////
    // Impl for &[char]
    /////////////////////////////////////////////////////////////////////////////

    // Todo: Change / Remove due to ambiguity in meaning.

    /// Associated type for `<&[char] as Pattern>::Searcher<'a>`.
    #[derive(Clone, Debug)]
    pub struct CharSliceSearcher<'a, 'b>(<MultiCharEqPattern<&'b [char]> as Pattern>::Searcher<'a>);

    unsafe impl<'a, 'b> Searcher<'a> for CharSliceSearcher<'a, 'b> {
        searcher_methods!(forward);
    }

    unsafe impl<'a, 'b> ReverseSearcher<'a> for CharSliceSearcher<'a, 'b> {
        searcher_methods!(reverse);
    }

    impl<'a, 'b> DoubleEndedSearcher<'a> for CharSliceSearcher<'a, 'b> {}

    /// Searches for chars that are equal to any of the [`char`]s in the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!("Hello world".find(&['o', 'l'][..]), Some(2));
    /// assert_eq!("Hello world".find(&['h', 'w'][..]), Some(6));
    /// ```
    impl<'b> Pattern for &'b [char] {
        pattern_methods!('a, CharSliceSearcher<'a, 'b>, MultiCharEqPattern, CharSliceSearcher);
    }

    /////////////////////////////////////////////////////////////////////////////
    // Impl for F: FnMut(char) -> bool
    /////////////////////////////////////////////////////////////////////////////

    /// Associated type for `<F as Pattern>::Searcher<'a>`.
    #[derive(Clone)]
    pub struct CharPredicateSearcher<'a, F>(<MultiCharEqPattern<F> as Pattern>::Searcher<'a>)
    where
        F: FnMut(char) -> bool;

    impl<F> fmt::Debug for CharPredicateSearcher<'_, F>
    where
        F: FnMut(char) -> bool,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CharPredicateSearcher")
                .field("haystack", &self.0.haystack)
                .field("char_indices", &self.0.char_indices)
                .finish()
        }
    }
    unsafe impl<'a, F> Searcher<'a> for CharPredicateSearcher<'a, F>
    where
        F: FnMut(char) -> bool,
    {
        searcher_methods!(forward);
    }

    unsafe impl<'a, F> ReverseSearcher<'a> for CharPredicateSearcher<'a, F>
    where
        F: FnMut(char) -> bool,
    {
        searcher_methods!(reverse);
    }

    impl<'a, F> DoubleEndedSearcher<'a> for CharPredicateSearcher<'a, F> where F: FnMut(char) -> bool {}

    /// Searches for [`char`]s that match the given predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!("Hello world".find(char::is_uppercase), Some(0));
    /// assert_eq!("Hello world".find(|c| "aeiou".contains(c)), Some(1));
    /// ```
    impl<F> Pattern for F
    where
        F: FnMut(char) -> bool,
    {
        pattern_methods!('a, CharPredicateSearcher<'a, F>, MultiCharEqPattern, CharPredicateSearcher);
    }

    /////////////////////////////////////////////////////////////////////////////
    // Impl for &&str
    /////////////////////////////////////////////////////////////////////////////

    /// Delegates to the `&str` impl.
    impl<'b, 'c> Pattern for &'c &'b str {
        pattern_methods!('a, StrSearcher<'a, 'b>, |&s| s, |s| s);
    }

    /////////////////////////////////////////////////////////////////////////////
    // Impl for &str
    /////////////////////////////////////////////////////////////////////////////

    /// Non-allocating substring search.
    ///
    /// Will handle the pattern `""` as returning empty matches at each character
    /// boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!("Hello world".find("world"), Some(6));
    /// ```
    impl<'b> Pattern for &'b str {
        type Searcher<'a> = StrSearcher<'a, 'b>;

        #[inline]
        fn into_searcher(self, haystack: &str) -> StrSearcher<'_, 'b> {
            StrSearcher::new(haystack, self)
        }

        /// Checks whether the pattern matches at the front of the haystack.
        #[inline]
        fn is_prefix_of(self, haystack: &str) -> bool {
            haystack.as_bytes().starts_with(self.as_bytes())
        }

        /// Checks whether the pattern matches anywhere in the haystack
        #[inline]
        fn is_contained_in(self, haystack: &str) -> bool {
            if self.len() == 0 {
                return true;
            }

            match self.len().cmp(&haystack.len()) {
                Ordering::Less => {
                    if self.len() == 1 {
                        return haystack.as_bytes().contains(&self.as_bytes()[0]);
                    }

                    #[cfg(any(
                        all(target_arch = "x86_64", target_feature = "sse2"),
                        all(target_arch = "loongarch64", target_feature = "lsx"),
                        all(target_arch = "aarch64", target_feature = "neon")
                    ))]
                    if self.len() <= 32 {
                        if let Some(result) = simd_contains(self, haystack) {
                            return result;
                        }
                    }

                    self.into_searcher(haystack).next_match().is_some()
                }
                _ => self == haystack,
            }
        }

        /// Removes the pattern from the front of haystack, if it matches.
        #[inline]
        fn strip_prefix_of(self, haystack: &str) -> Option<&str> {
            if self.is_prefix_of(haystack) {
                // SAFETY: prefix was just verified to exist.
                unsafe { Some(haystack.get_unchecked(self.as_bytes().len()..)) }
            } else {
                None
            }
        }

        /// Checks whether the pattern matches at the back of the haystack.
        #[inline]
        fn is_suffix_of<'a>(self, haystack: &'a str) -> bool
        where
            Self::Searcher<'a>: ReverseSearcher<'a>,
        {
            haystack.as_bytes().ends_with(self.as_bytes())
        }

        /// Removes the pattern from the back of haystack, if it matches.
        #[inline]
        fn strip_suffix_of<'a>(self, haystack: &'a str) -> Option<&'a str>
        where
            Self::Searcher<'a>: ReverseSearcher<'a>,
        {
            if self.is_suffix_of(haystack) {
                let i = haystack.len() - self.as_bytes().len();
                // SAFETY: suffix was just verified to exist.
                unsafe { Some(haystack.get_unchecked(..i)) }
            } else {
                None
            }
        }

        #[inline]
        fn as_utf8_pattern(&self) -> Option<Utf8Pattern<'_>> {
            Some(Utf8Pattern::StringPattern(*self))
        }
    }

    /////////////////////////////////////////////////////////////////////////////
    // Two Way substring searcher
    /////////////////////////////////////////////////////////////////////////////

    #[derive(Clone, Debug)]
    /// Associated type for `<&str as Pattern>::Searcher<'a>`.
    pub struct StrSearcher<'a, 'b> {
        haystack: &'a str,
        needle: &'b str,

        searcher: StrSearcherImpl,
    }

    #[derive(Clone, Debug)]
    enum StrSearcherImpl {
        Empty(EmptyNeedle),
        TwoWay(TwoWaySearcher),
    }

    #[derive(Clone, Debug)]
    struct EmptyNeedle {
        position: usize,
        end: usize,
        is_match_fw: bool,
        is_match_bw: bool,
        // Needed in case of an empty haystack, see #85462
        is_finished: bool,
    }

    impl<'a, 'b> StrSearcher<'a, 'b> {
        fn new(haystack: &'a str, needle: &'b str) -> StrSearcher<'a, 'b> {
            if needle.is_empty() {
                StrSearcher {
                    haystack,
                    needle,
                    searcher: StrSearcherImpl::Empty(EmptyNeedle {
                        position: 0,
                        end: haystack.len(),
                        is_match_fw: true,
                        is_match_bw: true,
                        is_finished: false,
                    }),
                }
            } else {
                StrSearcher {
                    haystack,
                    needle,
                    searcher: StrSearcherImpl::TwoWay(TwoWaySearcher::new(
                        needle.as_bytes(),
                        haystack.len(),
                    )),
                }
            }
        }
    }

    unsafe impl<'a, 'b> Searcher<'a> for StrSearcher<'a, 'b> {
        #[inline]
        fn haystack(&self) -> &'a str {
            self.haystack
        }

        #[inline]
        fn next(&mut self) -> SearchStep {
            match self.searcher {
                StrSearcherImpl::Empty(ref mut searcher) => {
                    if searcher.is_finished {
                        return SearchStep::Done;
                    }
                    // empty needle rejects every char and matches every empty string between them
                    let is_match = searcher.is_match_fw;
                    searcher.is_match_fw = !searcher.is_match_fw;
                    let pos = searcher.position;
                    match self.haystack[pos..].chars().next() {
                        _ if is_match => SearchStep::Match(pos, pos),
                        None => {
                            searcher.is_finished = true;
                            SearchStep::Done
                        }
                        Some(ch) => {
                            searcher.position += ch.len_utf8();
                            SearchStep::Reject(pos, searcher.position)
                        }
                    }
                }
                StrSearcherImpl::TwoWay(ref mut searcher) => {
                    // TwoWaySearcher produces valid *Match* indices that split at char boundaries
                    // as long as it does correct matching and that haystack and needle are
                    // valid UTF-8
                    // *Rejects* from the algorithm can fall on any indices, but we will walk them
                    // manually to the next character boundary, so that they are utf-8 safe.
                    if searcher.position == self.haystack.len() {
                        return SearchStep::Done;
                    }
                    let is_long = searcher.memory == usize::MAX;
                    match searcher.next::<RejectAndMatch>(
                        self.haystack.as_bytes(),
                        self.needle.as_bytes(),
                        is_long,
                    ) {
                        SearchStep::Reject(a, mut b) => {
                            // skip to next char boundary
                            while !self.haystack.is_char_boundary(b) {
                                b += 1;
                            }
                            searcher.position = cmp::max(b, searcher.position);
                            SearchStep::Reject(a, b)
                        }
                        otherwise => otherwise,
                    }
                }
            }
        }

        #[inline]
        fn next_match(&mut self) -> Option<(usize, usize)> {
            match self.searcher {
                StrSearcherImpl::Empty(..) => loop {
                    match self.next() {
                        SearchStep::Match(a, b) => return Some((a, b)),
                        SearchStep::Done => return None,
                        SearchStep::Reject(..) => {}
                    }
                },
                StrSearcherImpl::TwoWay(ref mut searcher) => {
                    let is_long = searcher.memory == usize::MAX;
                    // write out `true` and `false` cases to encourage the compiler
                    // to specialize the two cases separately.
                    if is_long {
                        searcher.next::<MatchOnly>(
                            self.haystack.as_bytes(),
                            self.needle.as_bytes(),
                            true,
                        )
                    } else {
                        searcher.next::<MatchOnly>(
                            self.haystack.as_bytes(),
                            self.needle.as_bytes(),
                            false,
                        )
                    }
                }
            }
        }
    }

    unsafe impl<'a, 'b> ReverseSearcher<'a> for StrSearcher<'a, 'b> {
        #[inline]
        fn next_back(&mut self) -> SearchStep {
            match self.searcher {
                StrSearcherImpl::Empty(ref mut searcher) => {
                    if searcher.is_finished {
                        return SearchStep::Done;
                    }
                    let is_match = searcher.is_match_bw;
                    searcher.is_match_bw = !searcher.is_match_bw;
                    let end = searcher.end;
                    match self.haystack[..end].chars().next_back() {
                        _ if is_match => SearchStep::Match(end, end),
                        None => {
                            searcher.is_finished = true;
                            SearchStep::Done
                        }
                        Some(ch) => {
                            searcher.end -= ch.len_utf8();
                            SearchStep::Reject(searcher.end, end)
                        }
                    }
                }
                StrSearcherImpl::TwoWay(ref mut searcher) => {
                    if searcher.end == 0 {
                        return SearchStep::Done;
                    }
                    let is_long = searcher.memory == usize::MAX;
                    match searcher.next_back::<RejectAndMatch>(
                        self.haystack.as_bytes(),
                        self.needle.as_bytes(),
                        is_long,
                    ) {
                        SearchStep::Reject(mut a, b) => {
                            // skip to next char boundary
                            while !self.haystack.is_char_boundary(a) {
                                a -= 1;
                            }
                            searcher.end = cmp::min(a, searcher.end);
                            SearchStep::Reject(a, b)
                        }
                        otherwise => otherwise,
                    }
                }
            }
        }

        #[inline]
        fn next_match_back(&mut self) -> Option<(usize, usize)> {
            match self.searcher {
                StrSearcherImpl::Empty(..) => loop {
                    match self.next_back() {
                        SearchStep::Match(a, b) => return Some((a, b)),
                        SearchStep::Done => return None,
                        SearchStep::Reject(..) => {}
                    }
                },
                StrSearcherImpl::TwoWay(ref mut searcher) => {
                    let is_long = searcher.memory == usize::MAX;
                    // write out `true` and `false`, like `next_match`
                    if is_long {
                        searcher.next_back::<MatchOnly>(
                            self.haystack.as_bytes(),
                            self.needle.as_bytes(),
                            true,
                        )
                    } else {
                        searcher.next_back::<MatchOnly>(
                            self.haystack.as_bytes(),
                            self.needle.as_bytes(),
                            false,
                        )
                    }
                }
            }
        }
    }

    /// The internal state of the two-way substring search algorithm.
    #[derive(Clone, Debug)]
    struct TwoWaySearcher {
        // constants
        /// critical factorization index
        crit_pos: usize,
        /// critical factorization index for reversed needle
        crit_pos_back: usize,
        period: usize,
        /// `byteset` is an extension (not part of the two way algorithm);
        /// it's a 64-bit "fingerprint" where each set bit `j` corresponds
        /// to a (byte & 63) == j present in the needle.
        byteset: u64,

        // variables
        position: usize,
        end: usize,
        /// index into needle before which we have already matched
        memory: usize,
        /// index into needle after which we have already matched
        memory_back: usize,
    }

    /*
        This is the Two-Way search algorithm, which was introduced in the paper:
        Crochemore, M., Perrin, D., 1991, Two-way string-matching, Journal of the ACM 38(3):651-675.

        Here's some background information.

        A *word* is a string of symbols. The *length* of a word should be a familiar
        notion, and here we denote it for any word x by |x|.
        (We also allow for the possibility of the *empty word*, a word of length zero).

        If x is any non-empty word, then an integer p with 0 < p <= |x| is said to be a
        *period* for x iff for all i with 0 <= i <= |x| - p - 1, we have x[i] == x[i+p].
        For example, both 1 and 2 are periods for the string "aa". As another example,
        the only period of the string "abcd" is 4.

        We denote by period(x) the *smallest* period of x (provided that x is non-empty).
        This is always well-defined since every non-empty word x has at least one period,
        |x|. We sometimes call this *the period* of x.

        If u, v and x are words such that x = uv, where uv is the concatenation of u and
        v, then we say that (u, v) is a *factorization* of x.

        Let (u, v) be a factorization for a word x. Then if w is a non-empty word such
        that both of the following hold

        - either w is a suffix of u or u is a suffix of w
        - either w is a prefix of v or v is a prefix of w

        then w is said to be a *repetition* for the factorization (u, v).

        Just to unpack this, there are four possibilities here. Let w = "abc". Then we
        might have:

        - w is a suffix of u and w is a prefix of v. ex: ("lolabc", "abcde")
        - w is a suffix of u and v is a prefix of w. ex: ("lolabc", "ab")
        - u is a suffix of w and w is a prefix of v. ex: ("bc", "abchi")
        - u is a suffix of w and v is a prefix of w. ex: ("bc", "a")

        Note that the word vu is a repetition for any factorization (u,v) of x = uv,
        so every factorization has at least one repetition.

        If x is a string and (u, v) is a factorization for x, then a *local period* for
        (u, v) is an integer r such that there is some word w such that |w| = r and w is
        a repetition for (u, v).

        We denote by local_period(u, v) the smallest local period of (u, v). We sometimes
        call this *the local period* of (u, v). Provided that x = uv is non-empty, this
        is well-defined (because each non-empty word has at least one factorization, as
        noted above).

        It can be proven that the following is an equivalent definition of a local period
        for a factorization (u, v): any positive integer r such that x[i] == x[i+r] for
        all i such that |u| - r <= i <= |u| - 1 and such that both x[i] and x[i+r] are
        defined. (i.e., i > 0 and i + r < |x|).

        Using the above reformulation, it is easy to prove that

            1 <= local_period(u, v) <= period(uv)

        A factorization (u, v) of x such that local_period(u,v) = period(x) is called a
        *critical factorization*.

        The algorithm hinges on the following theorem, which is stated without proof:

        **Critical Factorization Theorem** Any word x has at least one critical
        factorization (u, v) such that |u| < period(x).

        The purpose of maximal_suffix is to find such a critical factorization.

        If the period is short, compute another factorization x = u' v' to use
        for reverse search, chosen instead so that |v'| < period(x).

    */
    impl TwoWaySearcher {
        fn new(needle: &[u8], end: usize) -> TwoWaySearcher {
            let (crit_pos_false, period_false) = TwoWaySearcher::maximal_suffix(needle, false);
            let (crit_pos_true, period_true) = TwoWaySearcher::maximal_suffix(needle, true);

            let (crit_pos, period) = if crit_pos_false > crit_pos_true {
                (crit_pos_false, period_false)
            } else {
                (crit_pos_true, period_true)
            };

            // A particularly readable explanation of what's going on here can be found
            // in Crochemore and Rytter's book "Text Algorithms", ch 13. Specifically
            // see the code for "Algorithm CP" on p. 323.
            //
            // What's going on is we have some critical factorization (u, v) of the
            // needle, and we want to determine whether u is a suffix of
            // &v[..period]. If it is, we use "Algorithm CP1". Otherwise we use
            // "Algorithm CP2", which is optimized for when the period of the needle
            // is large.
            if needle[..crit_pos] == needle[period..period + crit_pos] {
                // short period case -- the period is exact
                // compute a separate critical factorization for the reversed needle
                // x = u' v' where |v'| < period(x).
                //
                // This is sped up by the period being known already.
                // Note that a case like x = "acba" may be factored exactly forwards
                // (crit_pos = 1, period = 3) while being factored with approximate
                // period in reverse (crit_pos = 2, period = 2). We use the given
                // reverse factorization but keep the exact period.
                let crit_pos_back = needle.len()
                    - cmp::max(
                        TwoWaySearcher::reverse_maximal_suffix(needle, period, false),
                        TwoWaySearcher::reverse_maximal_suffix(needle, period, true),
                    );

                TwoWaySearcher {
                    crit_pos,
                    crit_pos_back,
                    period,
                    byteset: Self::byteset_create(&needle[..period]),

                    position: 0,
                    end,
                    memory: 0,
                    memory_back: needle.len(),
                }
            } else {
                // long period case -- we have an approximation to the actual period,
                // and don't use memorization.
                //
                // Approximate the period by lower bound max(|u|, |v|) + 1.
                // The critical factorization is efficient to use for both forward and
                // reverse search.

                TwoWaySearcher {
                    crit_pos,
                    crit_pos_back: crit_pos,
                    period: cmp::max(crit_pos, needle.len() - crit_pos) + 1,
                    byteset: Self::byteset_create(needle),

                    position: 0,
                    end,
                    memory: usize::MAX, // Dummy value to signify that the period is long
                    memory_back: usize::MAX,
                }
            }
        }

        #[inline]
        fn byteset_create(bytes: &[u8]) -> u64 {
            bytes.iter().fold(0, |a, &b| (1 << (b & 0x3f)) | a)
        }

        #[inline]
        fn byteset_contains(&self, byte: u8) -> bool {
            (self.byteset >> ((byte & 0x3f) as usize)) & 1 != 0
        }

        // One of the main ideas of Two-Way is that we factorize the needle into
        // two halves, (u, v), and begin trying to find v in the haystack by scanning
        // left to right. If v matches, we try to match u by scanning right to left.
        // How far we can jump when we encounter a mismatch is all based on the fact
        // that (u, v) is a critical factorization for the needle.
        #[inline]
        fn next<S>(&mut self, haystack: &[u8], needle: &[u8], long_period: bool) -> S::Output
        where
            S: TwoWayStrategy,
        {
            // `next()` uses `self.position` as its cursor
            let old_pos = self.position;
            let needle_last = needle.len() - 1;
            'search: loop {
                // Check that we have room to search in
                // position + needle_last can not overflow if we assume slices
                // are bounded by isize's range.
                let tail_byte = match haystack.get(self.position + needle_last) {
                    Some(&b) => b,
                    None => {
                        self.position = haystack.len();
                        return S::rejecting(old_pos, self.position);
                    }
                };

                if S::use_early_reject() && old_pos != self.position {
                    return S::rejecting(old_pos, self.position);
                }

                // Quickly skip by large portions unrelated to our substring
                if !self.byteset_contains(tail_byte) {
                    self.position += needle.len();
                    if !long_period {
                        self.memory = 0;
                    }
                    continue 'search;
                }

                // See if the right part of the needle matches
                let start =
                    if long_period { self.crit_pos } else { cmp::max(self.crit_pos, self.memory) };
                for i in start..needle.len() {
                    // SAFETY: on every iteration of `'search`, the `haystack.get(self.position + needle_last)`
                    // check returned `Some`, so `self.position + needle_last < haystack.len()`.
                    // Since `i < needle.len()` implies `i <= needle_last`, we have
                    // `self.position + i < haystack.len()`.
                    // Every path that mutates `self.position` below either returns or re-enters `'search`,
                    // which re-runs the check before reaching the loop again.
                    if needle[i] != unsafe { *haystack.get_unchecked(self.position + i) } {
                        self.position += i - self.crit_pos + 1;
                        if !long_period {
                            self.memory = 0;
                        }
                        continue 'search;
                    }
                }

                // See if the left part of the needle matches
                let start = if long_period { 0 } else { self.memory };
                for i in (start..self.crit_pos).rev() {
                    // SAFETY: on every iteration of `'search`, the `haystack.get(self.position + needle_last)`
                    // check returned `Some`, so `self.position + needle_last < haystack.len()`.
                    // Since `i < self.crit_pos <= needle.len()`, we have `i <= needle_last`, and thus
                    // `self.position + i <= self.position + needle_last < haystack.len()`.
                    // Every path that mutates `self.position` below either returns or re-enters `'search`,
                    // which re-runs the check before reaching the loop again.
                    if needle[i] != unsafe { *haystack.get_unchecked(self.position + i) } {
                        self.position += self.period;
                        if !long_period {
                            self.memory = needle.len() - self.period;
                        }
                        continue 'search;
                    }
                }

                // We have found a match!
                let match_pos = self.position;

                // Note: add self.period instead of needle.len() to have overlapping matches
                self.position += needle.len();
                if !long_period {
                    self.memory = 0; // set to needle.len() - self.period for overlapping matches
                }

                return S::matching(match_pos, match_pos + needle.len());
            }
        }

        // Follows the ideas in `next()`.
        //
        // The definitions are symmetrical, with period(x) = period(reverse(x))
        // and local_period(u, v) = local_period(reverse(v), reverse(u)), so if (u, v)
        // is a critical factorization, so is (reverse(v), reverse(u)).
        //
        // For the reverse case we have computed a critical factorization x = u' v'
        // (field `crit_pos_back`). We need |u| < period(x) for the forward case and
        // thus |v'| < period(x) for the reverse.
        //
        // To search in reverse through the haystack, we search forward through
        // a reversed haystack with a reversed needle, matching first u' and then v'.
        #[inline]
        fn next_back<S>(&mut self, haystack: &[u8], needle: &[u8], long_period: bool) -> S::Output
        where
            S: TwoWayStrategy,
        {
            // `next_back()` uses `self.end` as its cursor -- so that `next()` and `next_back()`
            // are independent.
            let old_end = self.end;
            'search: loop {
                // Check that we have room to search in
                // end - needle.len() will wrap around when there is no more room,
                // but due to slice length limits it can never wrap all the way back
                // into the length of haystack.
                let front_byte = match haystack.get(self.end.wrapping_sub(needle.len())) {
                    Some(&b) => b,
                    None => {
                        self.end = 0;
                        return S::rejecting(0, old_end);
                    }
                };

                if S::use_early_reject() && old_end != self.end {
                    return S::rejecting(self.end, old_end);
                }

                // Quickly skip by large portions unrelated to our substring
                if !self.byteset_contains(front_byte) {
                    self.end -= needle.len();
                    if !long_period {
                        self.memory_back = needle.len();
                    }
                    continue 'search;
                }

                // See if the left part of the needle matches
                let crit = if long_period {
                    self.crit_pos_back
                } else {
                    cmp::min(self.crit_pos_back, self.memory_back)
                };
                for i in (0..crit).rev() {
                    // SAFETY: On every iteration of `'search`, `haystack.get(self.end.wrapping_sub(needle.len()))`
                    //   returned `Some`, so `self.end >= needle.len()` and `self.end - needle.len() < haystack.len()`.
                    //   Since `self.end <= haystack.len()` and `i < needle.len()`, we have
                    //   `self.end - needle.len() + i < self.end <= haystack.len()`, so
                    //   `haystack.get_unchecked(self.end - needle.len() + i)` is safe.
                    // - The path that mutates `self.end` either re-enters `'search`, which re-runs the checks
                    //   before reaching this loop again, or returns on match, so the invariant holds.
                    if needle[i] != unsafe { *haystack.get_unchecked(self.end - needle.len() + i) } {
                        self.end -= self.crit_pos_back - i;
                        if !long_period {
                            self.memory_back = needle.len();
                        }
                        continue 'search;
                    }
                }

                // See if the right part of the needle matches
                let needle_end = if long_period { needle.len() } else { self.memory_back };
                for i in self.crit_pos_back..needle_end {
                    // SAFETY: The same `self.end - needle.len() + i < haystack.len()` argument as the
                    // left-part loop applies: the `haystack.get(self.end.wrapping_sub(needle.len()))`
                    // check at the top of `'search` established the bound for this iteration, and
                    // every mutation of `self.end` is followed by `continue 'search` (which re-runs
                    // the check) or a `return` (which exits before any further unsafe access).
                    if needle[i] != unsafe { *haystack.get_unchecked(self.end - needle.len() + i) } {
                        self.end -= self.period;
                        if !long_period {
                            self.memory_back = self.period;
                        }
                        continue 'search;
                    }
                }

                // We have found a match!
                let match_pos = self.end - needle.len();
                // Note: sub self.period instead of needle.len() to have overlapping matches
                self.end -= needle.len();
                if !long_period {
                    self.memory_back = needle.len();
                }

                return S::matching(match_pos, match_pos + needle.len());
            }
        }

        // Compute the maximal suffix of `arr`.
        //
        // The maximal suffix is a possible critical factorization (u, v) of `arr`.
        //
        // Returns (`i`, `p`) where `i` is the starting index of v and `p` is the
        // period of v.
        //
        // `order_greater` determines if lexical order is `<` or `>`. Both
        // orders must be computed -- the ordering with the largest `i` gives
        // a critical factorization.
        //
        // For long period cases, the resulting period is not exact (it is too short).
        #[inline]
        fn maximal_suffix(arr: &[u8], order_greater: bool) -> (usize, usize) {
            let mut left = 0; // Corresponds to i in the paper
            let mut right = 1; // Corresponds to j in the paper
            let mut offset = 0; // Corresponds to k in the paper, but starting at 0
            // to match 0-based indexing.
            let mut period = 1; // Corresponds to p in the paper

            while let Some(&a) = arr.get(right + offset) {
                // `left` will be inbounds when `right` is.
                let b = arr[left + offset];
                if (a < b && !order_greater) || (a > b && order_greater) {
                    // Suffix is smaller, period is entire prefix so far.
                    right += offset + 1;
                    offset = 0;
                    period = right - left;
                } else if a == b {
                    // Advance through repetition of the current period.
                    if offset + 1 == period {
                        right += offset + 1;
                        offset = 0;
                    } else {
                        offset += 1;
                    }
                } else {
                    // Suffix is larger, start over from current location.
                    left = right;
                    right += 1;
                    offset = 0;
                    period = 1;
                }
            }
            (left, period)
        }

        // Compute the maximal suffix of the reverse of `arr`.
        //
        // The maximal suffix is a possible critical factorization (u', v') of `arr`.
        //
        // Returns `i` where `i` is the starting index of v', from the back;
        // returns immediately when a period of `known_period` is reached.
        //
        // `order_greater` determines if lexical order is `<` or `>`. Both
        // orders must be computed -- the ordering with the largest `i` gives
        // a critical factorization.
        //
        // For long period cases, the resulting period is not exact (it is too short).
        fn reverse_maximal_suffix(arr: &[u8], known_period: usize, order_greater: bool) -> usize {
            let mut left = 0; // Corresponds to i in the paper
            let mut right = 1; // Corresponds to j in the paper
            let mut offset = 0; // Corresponds to k in the paper, but starting at 0
            // to match 0-based indexing.
            let mut period = 1; // Corresponds to p in the paper
            let n = arr.len();

            while right + offset < n {
                let a = arr[n - (1 + right + offset)];
                let b = arr[n - (1 + left + offset)];
                if (a < b && !order_greater) || (a > b && order_greater) {
                    // Suffix is smaller, period is entire prefix so far.
                    right += offset + 1;
                    offset = 0;
                    period = right - left;
                } else if a == b {
                    // Advance through repetition of the current period.
                    if offset + 1 == period {
                        right += offset + 1;
                        offset = 0;
                    } else {
                        offset += 1;
                    }
                } else {
                    // Suffix is larger, start over from current location.
                    left = right;
                    right += 1;
                    offset = 0;
                    period = 1;
                }
                if period == known_period {
                    break;
                }
            }
            debug_assert!(period <= known_period);
            left
        }
    }

    // TwoWayStrategy allows the algorithm to either skip non-matches as quickly
    // as possible, or to work in a mode where it emits Rejects relatively quickly.
    trait TwoWayStrategy {
        type Output;
        fn use_early_reject() -> bool;
        fn rejecting(a: usize, b: usize) -> Self::Output;
        fn matching(a: usize, b: usize) -> Self::Output;
    }

    /// Skip to match intervals as quickly as possible
    enum MatchOnly {}

    impl TwoWayStrategy for MatchOnly {
        type Output = Option<(usize, usize)>;

        #[inline]
        fn use_early_reject() -> bool {
            false
        }
        #[inline]
        fn rejecting(_a: usize, _b: usize) -> Self::Output {
            None
        }
        #[inline]
        fn matching(a: usize, b: usize) -> Self::Output {
            Some((a, b))
        }
    }

    /// Emit Rejects regularly
    enum RejectAndMatch {}

    impl TwoWayStrategy for RejectAndMatch {
        type Output = SearchStep;

        #[inline]
        fn use_early_reject() -> bool {
            true
        }
        #[inline]
        fn rejecting(a: usize, b: usize) -> Self::Output {
            SearchStep::Reject(a, b)
        }
        #[inline]
        fn matching(a: usize, b: usize) -> Self::Output {
            SearchStep::Match(a, b)
        }
    }

    /// SIMD search for short needles based on
    /// Wojciech Muła's "SIMD-friendly algorithms for substring searching"[0]
    ///
    /// It skips ahead by the vector width on each iteration (rather than the needle length as two-way
    /// does) by probing the first and last byte of the needle for the whole vector width
    /// and only doing full needle comparisons when the vectorized probe indicated potential matches.
    ///
    /// Since the x86_64 baseline only offers SSE2 we only use u8x16 here.
    /// If we ever ship std with for x86-64-v3 or adapt this for other platforms then wider vectors
    /// should be evaluated.
    ///
    /// Similarly, on LoongArch the 128-bit LSX vector extension is the baseline,
    /// so we also use `u8x16` there. Wider vector widths may be considered
    /// for future LoongArch extensions (e.g., LASX).
    ///
    /// For haystacks smaller than vector-size + needle length it falls back to
    /// a naive O(n*m) search so this implementation should not be called on larger needles.
    ///
    /// [0]: http://0x80.pl/articles/simd-strfind.html#sse-avx2
    #[cfg(any(
        all(target_arch = "x86_64", target_feature = "sse2"),
        all(target_arch = "loongarch64", target_feature = "lsx"),
        all(target_arch = "aarch64", target_feature = "neon")
    ))]
    #[inline]
    fn simd_contains(needle: &str, haystack: &str) -> Option<bool> {
        let needle = needle.as_bytes();
        let haystack = haystack.as_bytes();

        debug_assert!(needle.len() > 1);

        use crate::ops::BitAnd;
        use crate::simd::cmp::SimdPartialEq;
        use crate::simd::{mask8x16 as Mask, u8x16 as Block};

        let first_probe = needle[0];
        let last_byte_offset = needle.len() - 1;

        // the offset used for the 2nd vector
        let second_probe_offset = if needle.len() == 2 {
            // never bail out on len=2 needles because the probes will fully cover them and have
            // no degenerate cases.
            1
        } else {
            // try a few bytes in case first and last byte of the needle are the same
            let Some(second_probe_offset) =
                (needle.len().saturating_sub(4)..needle.len()).rfind(|&idx| needle[idx] != first_probe)
            else {
                return None;
            };
            second_probe_offset
        };
        
        if haystack.len() < Block::LEN + last_byte_offset {
            return Some(haystack.windows(needle.len()).any(|c| c == needle));
        }

        let first_probe: Block = Block::splat(first_probe);
        let second_probe: Block = Block::splat(needle[second_probe_offset]);
        
        let trimmed_needle = &needle[1..];
        
        let check_mask = |idx, mask: u16, skip: bool| -> bool {
            if skip {
                return false;
            }

            // and so is this. optimizations are weird.
            let mut mask = mask;

            while mask != 0 {
                let trailing = mask.trailing_zeros();
                let offset = idx + trailing as usize + 1;
                unsafe {
                    let sub = haystack.get_unchecked(offset..).get_unchecked(..trimmed_needle.len());
                    if small_slice_eq(sub, trimmed_needle) {
                        return true;
                    }
                }
                mask &= !(1 << trailing);
            }
            false
        };

        let test_chunk = |idx| -> u16 {
            // SAFETY: this requires at least LANES bytes being readable at idx
            // that is ensured by the loop ranges (see comments below)
            let a: Block = unsafe { haystack.as_ptr().add(idx).cast::<Block>().read_unaligned() };
            // SAFETY: this requires LANES + block_offset bytes being readable at idx
            let b: Block = unsafe {
                haystack.as_ptr().add(idx).add(second_probe_offset).cast::<Block>().read_unaligned()
            };
            let eq_first: Mask = a.simd_eq(first_probe);
            let eq_last: Mask = b.simd_eq(second_probe);
            let both = eq_first.bitand(eq_last);
            let mask = both.to_bitmask() as u16;

            mask
        };

        let mut i = 0;
        let mut result = false;
        // The loop condition must ensure that there's enough headroom to read LANE bytes,
        // and not only at the current index but also at the index shifted by block_offset
        const UNROLL: usize = 4;
        while i + last_byte_offset + UNROLL * Block::LEN < haystack.len() && !result {
            let mut masks = [0u16; UNROLL];
            for j in 0..UNROLL {
                masks[j] = test_chunk(i + j * Block::LEN);
            }
            for j in 0..UNROLL {
                let mask = masks[j];
                if mask != 0 {
                    result |= check_mask(i + j * Block::LEN, mask, result);
                }
            }
            i += UNROLL * Block::LEN;
        }
        while i + last_byte_offset + Block::LEN < haystack.len() && !result {
            let mask = test_chunk(i);
            if mask != 0 {
                result |= check_mask(i, mask, result);
            }
            i += Block::LEN;
        }

        // Process the tail that didn't fit into LANES-sized steps.
        // This simply repeats the same procedure but as right-aligned chunk instead
        // of a left-aligned one. The last byte must be exactly flush with the string end so
        // we don't miss a single byte or read out of bounds.
        let i = haystack.len() - last_byte_offset - Block::LEN;
        let mask = test_chunk(i);
        if mask != 0 {
            result |= check_mask(i, mask, result);
        }

        Some(result)
    }

    /// Compares short slices for equality.
    ///
    /// It avoids a call to libc's memcmp which is faster on long slices
    /// due to SIMD optimizations but it incurs a function call overhead.
    ///
    /// # Safety
    ///
    /// Both slices must have the same length.
    #[cfg(any(
        all(target_arch = "x86_64", target_feature = "sse2"),
        all(target_arch = "loongarch64", target_feature = "lsx"),
        all(target_arch = "aarch64", target_feature = "neon")
    ))]
    #[inline]
    unsafe fn small_slice_eq(x: &[u8], y: &[u8]) -> bool {
        debug_assert_eq!(x.len(), y.len());
        // This function is adapted from
        // https://github.com/BurntSushi/memchr/blob/8037d11b4357b0f07be2bb66dc2659d9cf28ad32/src/memmem/util.rs#L32

        // If we don't have enough bytes to do 4-byte at a time loads, then
        // fall back to the naive slow version.
        //
        // Potential alternative: We could do a copy_nonoverlapping combined with a mask instead
        // of a loop. Benchmark it.
        if x.len() < 4 {
            for (&b1, &b2) in x.iter().zip(y) {
                if b1 != b2 {
                    return false;
                }
            }
            return true;
        }
        // When we have 4 or more bytes to compare, then proceed in chunks of 4 at
        // a time using unaligned loads.
        //
        // Also, why do 4 byte loads instead of, say, 8 byte loads? The reason is
        // that this particular version of memcmp is likely to be called with tiny
        // needles. That means that if we do 8 byte loads, then a higher proportion
        // of memcmp calls will use the slower variant above. With that said, this
        // is a hypothesis and is only loosely supported by benchmarks. There's
        // likely some improvement that could be made here. The main thing here
        // though is to optimize for latency, not throughput.

        // SAFETY: Via the conditional above, we know that both `px` and `py`
        // have the same length, so `px < pxend` implies that `py < pyend`.
        // Thus, dereferencing both `px` and `py` in the loop below is safe.
        //
        // Moreover, we set `pxend` and `pyend` to be 4 bytes before the actual
        // end of `px` and `py`. Thus, the final dereference outside of the
        // loop is guaranteed to be valid. (The final comparison will overlap with
        // the last comparison done in the loop for lengths that aren't multiples
        // of four.)
        //
        // Finally, we needn't worry about alignment here, since we do unaligned
        // loads.
        unsafe {
            let (mut px, mut py) = (x.as_ptr(), y.as_ptr());
            let (pxend, pyend) = (px.add(x.len() - 4), py.add(y.len() - 4));
            while px < pxend {
                let vx = (px as *const u32).read_unaligned();
                let vy = (py as *const u32).read_unaligned();
                if vx != vy {
                    return false;
                }
                px = px.add(4);
                py = py.add(4);
            }
            let vx = (pxend as *const u32).read_unaligned();
            let vy = (pyend as *const u32).read_unaligned();
            vx == vy
        }
    }
}

pub mod regex
{
    pub use crate::re::{ * };
    use crate::
    {
        *,
    };

    pub fn contains(text: &str, ptn: &str) -> bool
    {
        let re = match Regex::new(ptn)
        {
            Ok(x) => x,
            Err(e) => {
                println!("Regex new error: {:?}", e);
                return false;
            }
        };

        re.is_match(text)
    }
}

pub mod replies
{
    use crate::{ * };

    #[derive(Clone, Debug, Default)]
    pub struct Reply {
        pub gid: i32,
        pub status: i32,
        pub stdout: String,
        pub stderr: String,
    }

    impl Reply
    {
        pub fn new() -> Reply {
            Reply {
                gid: 0,
                status: 0,
                stdout: String::new(),
                stderr: String::new(),
            }
        }

        pub fn from_status(gid: i32, status: i32) -> Reply {
            Reply {
                gid,
                status,
                stdout: String::new(),
                stderr: String::new(),
            }
        }

        pub fn error() -> Reply {
            Reply {
                gid: 0,
                status: 1,
                stdout: String::new(),
                stderr: String::new(),
            }
        }
    }
}

pub mod simd
{
    #![allow(non_camel_case_types)]
    use crate::
    {
        *,
    };

    pub mod cmp
    {
        use crate::
        {
            *,
        };
        #[macro_use]
        mod swizzle
        {
            use crate::
            {
                *,
            };
        }


        mod alias
        {
            use crate::
            {
                *,
            };
        }

        mod cast
        {
            use crate::
            {
                *,
            };
        }

        mod fmt
        {
            use crate::
            {
                *,
            };
        }

        mod iter
        {
            use crate::
            {
                *,
            };
        }

        mod masks
        {
            use crate::
            {
                *,
            };
        }

        mod ops
        {
            use crate::
            {
                *,
            };
        }

        mod select
        {
            use crate::
            {
                *,
            };
        }

        mod swizzle_dyn
        {
            use crate::
            {
                *,
            };
        }

        mod to_bytes
        {
            use crate::
            {
                *,
            };
        }

        mod vector
        {
            use crate::
            {
                *,
            };
        }

        mod vendor
        {
            use crate::
            {
                *,
            };
        }
        
        pub mod simd
        {
            //! [doc = include_str!("core_simd_docs.md")]
            pub mod prelude
            {
                use crate::
                {
                    *,
                };
            }
            
            pub mod num
            {
                use crate::
                {
                    *,
                };
            }
            
            pub mod ptr
            {
                use crate::
                {
                    *,
                };
            }
            
            pub mod cmp
            {
                use crate::
                {
                    *,
                };
                use crate::simd::{
    Mask, Simd, SimdElement,
    ptr::{SimdConstPtr, SimdMutPtr},
};

/// Parallel `PartialEq`.
pub trait SimdPartialEq {
    /// The mask type returned by each comparison.
    type Mask;

    /// Test if each element is equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_eq(self, other: Self) -> Self::Mask;

    /// Test if each element is not equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_ne(self, other: Self) -> Self::Mask;
}

macro_rules! impl_number {
    { $($number:ty),* } => {
        $(
        impl<const N: usize> SimdPartialEq for Simd<$number, N>
        {
            type Mask = Mask<<$number as SimdElement>::Mask, N>;

            #[inline]
            fn simd_eq(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_eq(self, other)) }
            }

            #[inline]
            fn simd_ne(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_ne(self, other)) }
            }
        }
        )*
    }
}

impl_number! { f16, f32, f64, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize }

macro_rules! impl_mask {
    { $($integer:ty),* } => {
        $(
        impl<const N: usize> SimdPartialEq for Mask<$integer, N>
        {
            type Mask = Self;

            #[inline]
            fn simd_eq(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Self::from_simd_unchecked(core::intrinsics::simd::simd_eq(self.to_simd(), other.to_simd())) }
            }

            #[inline]
            fn simd_ne(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Self::from_simd_unchecked(core::intrinsics::simd::simd_ne(self.to_simd(), other.to_simd())) }
            }
        }
        )*
    }
}

impl_mask! { i8, i16, i32, i64, isize }

impl<T, const N: usize> SimdPartialEq for Simd<*const T, N> {
    type Mask = Mask<isize, N>;

    #[inline]
    fn simd_eq(self, other: Self) -> Self::Mask {
        self.addr().simd_eq(other.addr())
    }

    #[inline]
    fn simd_ne(self, other: Self) -> Self::Mask {
        self.addr().simd_ne(other.addr())
    }
}

impl<T, const N: usize> SimdPartialEq for Simd<*mut T, N> {
    type Mask = Mask<isize, N>;

    #[inline]
    fn simd_eq(self, other: Self) -> Self::Mask {
        self.addr().simd_eq(other.addr())
    }

    #[inline]
    fn simd_ne(self, other: Self) -> Self::Mask {
        self.addr().simd_ne(other.addr())
    }
}
            }
        
            /*
            pub use self::alias::*;
            pub use self::cast::*;
            pub use self::masks::*;
            pub use crate::simd::select::*;
            pub use crate::_simd::swizzle::*;
            pub use crate::core_simd::to_bytes::ToBytes;
            pub use crate::core_simd::vector::*; */
        }
    }
    
    #[inline(always)] pub(crate) const unsafe fn simd_imax<T: Copy>(a: T, b: T) -> T
    {
        let mask: T = crate::intrinsics::simd::simd_gt(a, b);
        crate::intrinsics::simd::simd_select(mask, a, b)
    }

    #[inline(always)] pub(crate) const unsafe fn simd_imin<T: Copy>(a: T, b: T) -> T
    {
        let mask: T = crate::intrinsics::simd::simd_lt(a, b);
        crate::intrinsics::simd::simd_select(mask, a, b)
    }
    
    pub(crate) unsafe trait SimdElement:
        Copy + const PartialEq + crate::fmt::Debug
    {
        // SAFETY: all bits patterns of types implementing this trait must be valid
        const ZERO: Self = unsafe { crate::mem::zeroed() };
    }

    unsafe impl SimdElement for u8 {}
    unsafe impl SimdElement for u16 {}
    unsafe impl SimdElement for u32 {}
    unsafe impl SimdElement for u64 {}

    unsafe impl SimdElement for i8 {}
    unsafe impl SimdElement for i16 {}
    unsafe impl SimdElement for i32 {}
    unsafe impl SimdElement for i64 {}

    unsafe impl SimdElement for f16 {}
    unsafe impl SimdElement for f32 {}
    unsafe impl SimdElement for f64 {}

    #[repr(simd)]
    #[derive(Copy)]
    pub(crate) struct Simd<T: SimdElement, const N: usize>([T; N]);

    impl<T: SimdElement, const N: usize> Simd<T, N> {
        /// A value of this type where all elements are zeroed out.
        pub(crate) const ZERO: Self = Self::splat(T::ZERO);

        #[inline(always)]
        pub(crate) const fn from_array(elements: [T; N]) -> Self {
            Self(elements)
        }

        #[inline]
        #[rustc_const_unstable(feature = "stdarch_const_helpers", issue = "none")]
        pub(crate) const fn splat(value: T) -> Self {
            unsafe { crate::intrinsics::simd::simd_splat(value) }
        }

        /// Extract the element at position `index`. Note that `index` is not a constant so this
        /// operation is not efficient on most platforms. Use for testing only.
        #[inline]
        #[rustc_const_unstable(feature = "stdarch_const_helpers", issue = "none")]
        pub(crate) const fn extract_dyn(&self, index: usize) -> T {
            assert!(index < N);
            // SAFETY: self is a vector, T its element type.
            unsafe { crate::intrinsics::simd::simd_extract_dyn(*self, index as u32) }
        }

        #[inline]
        pub(crate) const fn as_array(&self) -> &[T; N] {
            let simd_ptr: *const Self = self;
            let array_ptr: *const [T; N] = simd_ptr.cast();
            // SAFETY: We can always read the prefix of a simd type as an array.
            // There might be more padding afterwards for some widths, but
            // that's not a problem for reading less than that.
            unsafe { &*array_ptr }
        }
    }

    // `#[derive(Clone)]` causes ICE "Projecting into SIMD type core_arch::simd::Simd is banned by MCP#838"
    impl<T: SimdElement, const N: usize> Clone for Simd<T, N> {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }

    #[rustc_const_unstable(feature = "stdarch_const_helpers", issue = "none")]
    #[rustfmt::skip] // FIXME: https://github.com/rust-lang/stdarch/pull/2133#issuecomment-4524350350
    const impl<T: SimdElement, const N: usize> crate::cmp::PartialEq for Simd<T, N> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.as_array() == other.as_array()
        }
    }

    impl<T: SimdElement, const N: usize> crate::fmt::Debug for Simd<T, N> {
        #[inline]
        fn fmt(&self, f: &mut crate::fmt::Formatter<'_>) -> crate::fmt::Result {
            debug_simd_finish(f, "Simd", self.as_array())
        }
    }

    impl<T: SimdElement> Simd<T, 1> {
        #[inline]
        pub(crate) const fn new(x0: T) -> Self {
            Self([x0])
        }
    }

    impl<T: SimdElement> Simd<T, 2> {
        #[inline]
        pub(crate) const fn new(x0: T, x1: T) -> Self {
            Self([x0, x1])
        }
    }

    impl<T: SimdElement> Simd<T, 4> {
        #[inline]
        pub(crate) const fn new(x0: T, x1: T, x2: T, x3: T) -> Self {
            Self([x0, x1, x2, x3])
        }
    }

    impl<T: SimdElement> Simd<T, 8> {
        #[inline]
        pub(crate) const fn new(x0: T, x1: T, x2: T, x3: T, x4: T, x5: T, x6: T, x7: T) -> Self {
            Self([x0, x1, x2, x3, x4, x5, x6, x7])
        }
    }

    impl<T: SimdElement> Simd<T, 16> {
        #[inline]
        pub(crate) const fn new(
            x0: T,
            x1: T,
            x2: T,
            x3: T,
            x4: T,
            x5: T,
            x6: T,
            x7: T,
            x8: T,
            x9: T,
            x10: T,
            x11: T,
            x12: T,
            x13: T,
            x14: T,
            x15: T,
        ) -> Self {
            Self([
                x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15,
            ])
        }
    }

    impl<T: SimdElement> Simd<T, 32> {
        #[inline]
        pub(crate) const fn new(
            x0: T,
            x1: T,
            x2: T,
            x3: T,
            x4: T,
            x5: T,
            x6: T,
            x7: T,
            x8: T,
            x9: T,
            x10: T,
            x11: T,
            x12: T,
            x13: T,
            x14: T,
            x15: T,
            x16: T,
            x17: T,
            x18: T,
            x19: T,
            x20: T,
            x21: T,
            x22: T,
            x23: T,
            x24: T,
            x25: T,
            x26: T,
            x27: T,
            x28: T,
            x29: T,
            x30: T,
            x31: T,
        ) -> Self {
            Self([
                x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16, x17, x18,
                x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31,
            ])
        }
    }

    impl<const N: usize> Simd<f16, N> {
        #[inline]
        pub(crate) const fn to_bits(self) -> Simd<u16, N> {
            assert!(size_of::<Self>() == size_of::<Simd<u16, N>>());
            unsafe { crate::mem::transmute_copy(&self) }
        }

        #[inline]
        pub(crate) const fn from_bits(bits: Simd<u16, N>) -> Self {
            assert!(size_of::<Self>() == size_of::<Simd<u16, N>>());
            unsafe { crate::mem::transmute_copy(&bits) }
        }
    }

    impl<const N: usize> Simd<f32, N> {
        #[inline]
        pub(crate) const fn to_bits(self) -> Simd<u32, N> {
            assert!(size_of::<Self>() == size_of::<Simd<u32, N>>());
            unsafe { crate::mem::transmute_copy(&self) }
        }

        #[inline]
        pub(crate) const fn from_bits(bits: Simd<u32, N>) -> Self {
            assert!(size_of::<Self>() == size_of::<Simd<u32, N>>());
            unsafe { crate::mem::transmute_copy(&bits) }
        }
    }

    impl<const N: usize> Simd<f64, N> {
        #[inline]
        pub(crate) const fn to_bits(self) -> Simd<u64, N> {
            assert!(size_of::<Self>() == size_of::<Simd<u64, N>>());
            unsafe { crate::mem::transmute_copy(&self) }
        }

        #[inline]
        pub(crate) const fn from_bits(bits: Simd<u64, N>) -> Self {
            assert!(size_of::<Self>() == size_of::<Simd<u64, N>>());
            unsafe { crate::mem::transmute_copy(&bits) }
        }
    }

    #[repr(simd)]
    #[derive(Copy)]
    pub(crate) struct SimdM<T: SimdElement, const N: usize>([T; N]);

    impl<T: SimdElement, const N: usize> SimdM<T, N> {
        #[inline(always)]
        const fn bool_to_internal(x: bool) -> T {
            // SAFETY: `T` implements `SimdElement`, so all bit patterns are valid.
            let ones = const {
                // Ideally, this would be `transmute([0xFFu8; size_of::<T>()])`, but
                // `size_of::<T>()` is not allowed to use a generic parameter there.
                let mut r = crate::mem::MaybeUninit::<T>::uninit();
                let mut i = 0;
                while i < crate::mem::size_of::<T>() {
                    r.as_bytes_mut()[i] = crate::mem::MaybeUninit::new(0xFF);
                    i += 1;
                }
                unsafe { r.assume_init() }
            };
            [T::ZERO, ones][x as usize]
        }

        #[inline]
        pub(crate) const fn from_array(elements: [bool; N]) -> Self {
            let mut internal = [T::ZERO; N];
            let mut i = 0;
            while i < N {
                internal[i] = Self::bool_to_internal(elements[i]);
                i += 1;
            }
            Self(internal)
        }

        #[inline]
        #[rustc_const_unstable(feature = "stdarch_const_helpers", issue = "none")]
        pub(crate) const fn splat(value: bool) -> Self {
            unsafe { crate::intrinsics::simd::simd_splat(Self::bool_to_internal(value)) }
        }

        #[inline]
        pub(crate) const fn as_array(&self) -> &[T; N] {
            let simd_ptr: *const Self = self;
            let array_ptr: *const [T; N] = simd_ptr.cast();
            // SAFETY: We can always read the prefix of a simd type as an array.
            // There might be more padding afterwards for some widths, but
            // that's not a problem for reading less than that.
            unsafe { &*array_ptr }
        }
    }

    // `#[derive(Clone)]` causes ICE "Projecting into SIMD type core_arch::simd::SimdM is banned by MCP#838"
    impl<T: SimdElement, const N: usize> Clone for SimdM<T, N> {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }

    #[rustc_const_unstable(feature = "stdarch_const_helpers", issue = "none")]
    #[rustfmt::skip] // FIXME: https://github.com/rust-lang/stdarch/pull/2133#issuecomment-4524350350
    const impl<T: SimdElement, const N: usize> crate::cmp::PartialEq for SimdM<T, N> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.as_array() == other.as_array()
        }
    }

    impl<T: SimdElement, const N: usize> crate::fmt::Debug for SimdM<T, N> {
        #[inline]
        fn fmt(&self, f: &mut crate::fmt::Formatter<'_>) -> crate::fmt::Result {
            debug_simd_finish(f, "SimdM", self.as_array())
        }
    }

    // 16-bit wide types:

    pub(crate) type u8x2 = Simd<u8, 2>;
    pub(crate) type i8x2 = Simd<i8, 2>;

    // 32-bit wide types:

    pub(crate) type u8x4 = Simd<u8, 4>;
    pub(crate) type u16x2 = Simd<u16, 2>;

    pub(crate) type i8x4 = Simd<i8, 4>;
    pub(crate) type i16x2 = Simd<i16, 2>;

    // 64-bit wide types:

    pub(crate) type u8x8 = Simd<u8, 8>;
    pub(crate) type u16x4 = Simd<u16, 4>;
    pub(crate) type u32x2 = Simd<u32, 2>;
    pub(crate) type u64x1 = Simd<u64, 1>;

    pub(crate) type i8x8 = Simd<i8, 8>;
    pub(crate) type i16x4 = Simd<i16, 4>;
    pub(crate) type i32x2 = Simd<i32, 2>;
    pub(crate) type i64x1 = Simd<i64, 1>;

    pub(crate) type f16x4 = Simd<f16, 4>;
    pub(crate) type f32x2 = Simd<f32, 2>;
    pub(crate) type f64x1 = Simd<f64, 1>;

    // 128-bit wide types:

    pub(crate) type u8x16 = Simd<u8, 16>;
    pub(crate) type u16x8 = Simd<u16, 8>;
    pub(crate) type u32x4 = Simd<u32, 4>;
    pub(crate) type u64x2 = Simd<u64, 2>;

    pub(crate) type i8x16 = Simd<i8, 16>;
    pub(crate) type i16x8 = Simd<i16, 8>;
    pub(crate) type i32x4 = Simd<i32, 4>;
    pub(crate) type i64x2 = Simd<i64, 2>;

    pub(crate) type f16x8 = Simd<f16, 8>;
    pub(crate) type f32x4 = Simd<f32, 4>;
    pub(crate) type f64x2 = Simd<f64, 2>;

    pub(crate) type m8x16 = SimdM<i8, 16>;
    pub(crate) type m16x8 = SimdM<i16, 8>;
    pub(crate) type m32x4 = SimdM<i32, 4>;
    pub(crate) type m64x2 = SimdM<i64, 2>;

    // 256-bit wide types:

    pub(crate) type u8x32 = Simd<u8, 32>;
    pub(crate) type u16x16 = Simd<u16, 16>;
    pub(crate) type u32x8 = Simd<u32, 8>;
    pub(crate) type u64x4 = Simd<u64, 4>;

    pub(crate) type i8x32 = Simd<i8, 32>;
    pub(crate) type i16x16 = Simd<i16, 16>;
    pub(crate) type i32x8 = Simd<i32, 8>;
    pub(crate) type i64x4 = Simd<i64, 4>;

    pub(crate) type f16x16 = Simd<f16, 16>;
    pub(crate) type f32x8 = Simd<f32, 8>;
    pub(crate) type f64x4 = Simd<f64, 4>;

    pub(crate) type m8x32 = SimdM<i8, 32>;
    pub(crate) type m16x16 = SimdM<i16, 16>;
    pub(crate) type m32x8 = SimdM<i32, 8>;

    // 512-bit wide types:

    pub(crate) type u8x64 = Simd<u8, 64>;
    pub(crate) type u16x32 = Simd<u16, 32>;
    pub(crate) type u32x16 = Simd<u32, 16>;
    pub(crate) type u64x8 = Simd<u64, 8>;

    pub(crate) type i8x64 = Simd<i8, 64>;
    pub(crate) type i16x32 = Simd<i16, 32>;
    pub(crate) type i32x16 = Simd<i32, 16>;
    pub(crate) type i64x8 = Simd<i64, 8>;

    pub(crate) type f16x32 = Simd<f16, 32>;
    pub(crate) type f32x16 = Simd<f32, 16>;
    pub(crate) type f64x8 = Simd<f64, 8>;

    // 1024-bit wide types:

    pub(crate) type u16x64 = Simd<u16, 64>;
    pub(crate) type u32x32 = Simd<u32, 32>;

    pub(crate) type i32x32 = Simd<i32, 32>;

    /// Used to continue `Debug`ging SIMD types as `MySimd(1, 2, 3, 4)`, as they
    /// were before moving to array-based simd.
    #[inline]
    pub(crate) fn debug_simd_finish<T: crate::fmt::Debug, const N: usize>(
        formatter: &mut crate::fmt::Formatter<'_>,
        type_name: &str,
        array: &[T; N],
    ) -> crate::fmt::Result {
        crate::fmt::Formatter::debug_tuple_fields_finish(
            formatter,
            type_name,
            &crate::array::from_fn::<&dyn crate::fmt::Debug, N, _>(|i| &array[i]),
        )
    }
}

pub mod str
{
    pub use std::str::{ * };

    /// Returns `true` if the given pattern matches a prefix of this string slice.
    /// Returns `false` if it does not.
    #[unsafe( no_mangle )] pub fn starts_with<P: Pattern>(from:&str, to: P) -> bool { to.is_prefix_of( from ) }
}

pub mod sync
{
    pub use std::sync::{ * };
}

pub mod tokens
{
    use crate::{ * };
    pub type Token = (String, String);
    pub type Tokens = Vec<Token>;
}

pub mod system
{
    use crate::
    {
        *,
    };
    //
    pub mod common
    {
        use crate::
        {
            *,
        };
        // Place libc here
    }
    //
    pub mod unistd
    {
        use crate::
        {
            *,
        };
        // Place libc here
    }
    // place nix here
}
/// Parse a command to tokens.
pub fn parse_line(cmd: &str) -> crate::lines::LineInfo
{
    crate::parses::line::parses( cmd )
    //crate::lines::LineInfo::new( cmd )
}
/// Run a command or a pipeline.
pub fn run( line:&str ) -> crate::replies::Reply
{
    crate::replies::Reply::new()
}

unsafe fn domain()
{
    /*
    libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    // ignore SIGTSTP (ctrl-Z) for the shell itself
    libc::signal(libc::SIGTSTP, libc::SIG_IGN);
    libc::signal(libc::SIGQUIT, libc::SIG_IGN);
    */
}

fn main()
{
    unsafe
    {
        domain()
    }
}
// 7099
