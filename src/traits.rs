pub use core::sync::atomic::Ordering;

/// Trait describing types that are atomically sized, i.e. it's possible to perform
/// atomic operations on them.
pub trait AtomicallySized: Sized {
    type AtomicVersion: Atomic<Self>;
}

/// Trait describing an atomic version of an atomically-sized type.
pub trait Atomic<T>: Send + Sync + Sized
where
    T: Sized,
{
    /// Generate a reference to the atomic version of this type using its normal
    /// version.
    unsafe fn from_raw(val: &T) -> &Self {
        &*(val as *const T as *const Self)
    }

    fn new(val: T) -> Self;

    fn store(&self, val: T, order: Ordering);

    fn load(&self, order: Ordering) -> T;

    fn swap(&self, val: T, order: Ordering) -> T;

    fn get_mut(&mut self) -> &mut T;

    fn into_inner(self) -> T;

    fn compare_and_swap(&self, current: T, new: T, order: Ordering) -> T;

    fn compare_exchange(
        &self,
        current: T,
        new: T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<T, T>;

    fn compare_exchange_weak(
        &self,
        current: T,
        new: T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<T, T>;
}

/// Trait describing structs that implement atomic integer operations.
///
/// Makes generic containers a little easier.
pub trait AtomicNumeric<T>: Atomic<T> {
    fn fetch_add(&self, val: T, order: Ordering) -> T;
    fn fetch_sub(&self, val: T, order: Ordering) -> T;
}

/// Trait describing structs that implement atomic bitwise operations.
///
/// Makes generic containers a little easier.
pub trait AtomicBitwise<T>: Atomic<T> {
    fn fetch_and(&self, val: T, order: Ordering) -> T;
    fn fetch_or(&self, val: T, order: Ordering) -> T;
    fn fetch_xor(&self, val: T, order: Ordering) -> T;
    fn fetch_nand(&self, val: T, order: Ordering) -> T;
}
