use crate::traits::*;
use core::sync::atomic::*;

impl AtomicallySized for usize {
    type AtomicVersion = AtomicUsize;
}

impl Atomic<usize> for AtomicUsize {
    fn new(val: usize) -> Self {
        Self::new(val)
    }
    fn store(&self, val: usize, order: Ordering) {
        self.store(val, order)
    }
    fn load(&self, order: Ordering) -> usize {
        self.load(order)
    }
    fn swap(&self, val: usize, order: Ordering) -> usize {
        self.swap(val, order)
    }
    fn get_mut(&mut self) -> &mut usize {
        self.get_mut()
    }
    fn into_inner(self) -> usize {
        self.into_inner()
    }
    fn compare_and_swap(&self, current: usize, new: usize, order: Ordering) -> usize {
        self.compare_and_swap(current, new, order)
    }
    fn compare_exchange(
        &self,
        current: usize,
        new: usize,
        success: Ordering,
        failure: Ordering,
    ) -> Result<usize, usize> {
        self.compare_exchange(current, new, success, failure)
    }
    fn compare_exchange_weak(
        &self,
        current: usize,
        new: usize,
        success: Ordering,
        failure: Ordering,
    ) -> Result<usize, usize> {
        self.compare_exchange_weak(current, new, success, failure)
    }
}

impl AtomicNumeric<usize> for AtomicUsize {
    fn fetch_add(&self, val: usize, order: Ordering) -> usize {
        self.fetch_add(val, order)
    }
    fn fetch_sub(&self, val: usize, order: Ordering) -> usize {
        self.fetch_sub(val, order)
    }
}

impl AtomicBitwise<usize> for AtomicUsize {
    fn fetch_and(&self, val: usize, order: Ordering) -> usize {
        self.fetch_and(val, order)
    }
    fn fetch_or(&self, val: usize, order: Ordering) -> usize {
        self.fetch_or(val, order)
    }
    fn fetch_xor(&self, val: usize, order: Ordering) -> usize {
        self.fetch_xor(val, order)
    }
    fn fetch_nand(&self, val: usize, order: Ordering) -> usize {
        self.fetch_nand(val, order)
    }
}

impl AtomicallySized for isize {
    type AtomicVersion = AtomicIsize;
}

impl Atomic<isize> for AtomicIsize {
    fn new(val: isize) -> Self {
        Self::new(val)
    }
    fn store(&self, val: isize, order: Ordering) {
        self.store(val, order)
    }
    fn load(&self, order: Ordering) -> isize {
        self.load(order)
    }
    fn swap(&self, val: isize, order: Ordering) -> isize {
        self.swap(val, order)
    }
    fn get_mut(&mut self) -> &mut isize {
        self.get_mut()
    }
    fn into_inner(self) -> isize {
        self.into_inner()
    }
    fn compare_and_swap(&self, current: isize, new: isize, order: Ordering) -> isize {
        self.compare_and_swap(current, new, order)
    }
    fn compare_exchange(
        &self,
        current: isize,
        new: isize,
        success: Ordering,
        failure: Ordering,
    ) -> Result<isize, isize> {
        self.compare_exchange(current, new, success, failure)
    }
    fn compare_exchange_weak(
        &self,
        current: isize,
        new: isize,
        success: Ordering,
        failure: Ordering,
    ) -> Result<isize, isize> {
        self.compare_exchange_weak(current, new, success, failure)
    }
}

impl AtomicNumeric<isize> for AtomicIsize {
    fn fetch_add(&self, val: isize, order: Ordering) -> isize {
        self.fetch_add(val, order)
    }
    fn fetch_sub(&self, val: isize, order: Ordering) -> isize {
        self.fetch_sub(val, order)
    }
}

impl AtomicBitwise<isize> for AtomicIsize {
    fn fetch_and(&self, val: isize, order: Ordering) -> isize {
        self.fetch_and(val, order)
    }
    fn fetch_or(&self, val: isize, order: Ordering) -> isize {
        self.fetch_or(val, order)
    }
    fn fetch_xor(&self, val: isize, order: Ordering) -> isize {
        self.fetch_xor(val, order)
    }
    fn fetch_nand(&self, val: isize, order: Ordering) -> isize {
        self.fetch_nand(val, order)
    }
}

impl AtomicallySized for u64 {
    type AtomicVersion = AtomicU64;
}

impl Atomic<u64> for AtomicU64 {
    fn new(val: u64) -> Self {
        Self::new(val)
    }
    fn store(&self, val: u64, order: Ordering) {
        self.store(val, order)
    }
    fn load(&self, order: Ordering) -> u64 {
        self.load(order)
    }
    fn swap(&self, val: u64, order: Ordering) -> u64 {
        self.swap(val, order)
    }
    fn get_mut(&mut self) -> &mut u64 {
        self.get_mut()
    }
    fn into_inner(self) -> u64 {
        self.into_inner()
    }
    fn compare_and_swap(&self, current: u64, new: u64, order: Ordering) -> u64 {
        self.compare_and_swap(current, new, order)
    }
    fn compare_exchange(
        &self,
        current: u64,
        new: u64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<u64, u64> {
        self.compare_exchange(current, new, success, failure)
    }
    fn compare_exchange_weak(
        &self,
        current: u64,
        new: u64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<u64, u64> {
        self.compare_exchange_weak(current, new, success, failure)
    }
}

impl AtomicNumeric<u64> for AtomicU64 {
    fn fetch_add(&self, val: u64, order: Ordering) -> u64 {
        self.fetch_add(val, order)
    }
    fn fetch_sub(&self, val: u64, order: Ordering) -> u64 {
        self.fetch_sub(val, order)
    }
}

impl AtomicBitwise<u64> for AtomicU64 {
    fn fetch_and(&self, val: u64, order: Ordering) -> u64 {
        self.fetch_and(val, order)
    }
    fn fetch_or(&self, val: u64, order: Ordering) -> u64 {
        self.fetch_or(val, order)
    }
    fn fetch_xor(&self, val: u64, order: Ordering) -> u64 {
        self.fetch_xor(val, order)
    }
    fn fetch_nand(&self, val: u64, order: Ordering) -> u64 {
        self.fetch_nand(val, order)
    }
}

impl AtomicallySized for i64 {
    type AtomicVersion = AtomicI64;
}

impl Atomic<i64> for AtomicI64 {
    fn new(val: i64) -> Self {
        Self::new(val)
    }
    fn store(&self, val: i64, order: Ordering) {
        self.store(val, order)
    }
    fn load(&self, order: Ordering) -> i64 {
        self.load(order)
    }
    fn swap(&self, val: i64, order: Ordering) -> i64 {
        self.swap(val, order)
    }
    fn get_mut(&mut self) -> &mut i64 {
        self.get_mut()
    }
    fn into_inner(self) -> i64 {
        self.into_inner()
    }
    fn compare_and_swap(&self, current: i64, new: i64, order: Ordering) -> i64 {
        self.compare_and_swap(current, new, order)
    }
    fn compare_exchange(
        &self,
        current: i64,
        new: i64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<i64, i64> {
        self.compare_exchange(current, new, success, failure)
    }
    fn compare_exchange_weak(
        &self,
        current: i64,
        new: i64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<i64, i64> {
        self.compare_exchange_weak(current, new, success, failure)
    }
}

impl AtomicNumeric<i64> for AtomicI64 {
    fn fetch_add(&self, val: i64, order: Ordering) -> i64 {
        self.fetch_add(val, order)
    }
    fn fetch_sub(&self, val: i64, order: Ordering) -> i64 {
        self.fetch_sub(val, order)
    }
}

impl AtomicBitwise<i64> for AtomicI64 {
    fn fetch_and(&self, val: i64, order: Ordering) -> i64 {
        self.fetch_and(val, order)
    }
    fn fetch_or(&self, val: i64, order: Ordering) -> i64 {
        self.fetch_or(val, order)
    }
    fn fetch_xor(&self, val: i64, order: Ordering) -> i64 {
        self.fetch_xor(val, order)
    }
    fn fetch_nand(&self, val: i64, order: Ordering) -> i64 {
        self.fetch_nand(val, order)
    }
}

impl AtomicallySized for u32 {
    type AtomicVersion = AtomicU32;
}

impl Atomic<u32> for AtomicU32 {
    fn new(val: u32) -> Self {
        Self::new(val)
    }
    fn store(&self, val: u32, order: Ordering) {
        self.store(val, order)
    }
    fn load(&self, order: Ordering) -> u32 {
        self.load(order)
    }
    fn swap(&self, val: u32, order: Ordering) -> u32 {
        self.swap(val, order)
    }
    fn get_mut(&mut self) -> &mut u32 {
        self.get_mut()
    }
    fn into_inner(self) -> u32 {
        self.into_inner()
    }
    fn compare_and_swap(&self, current: u32, new: u32, order: Ordering) -> u32 {
        self.compare_and_swap(current, new, order)
    }
    fn compare_exchange(
        &self,
        current: u32,
        new: u32,
        success: Ordering,
        failure: Ordering,
    ) -> Result<u32, u32> {
        self.compare_exchange(current, new, success, failure)
    }
    fn compare_exchange_weak(
        &self,
        current: u32,
        new: u32,
        success: Ordering,
        failure: Ordering,
    ) -> Result<u32, u32> {
        self.compare_exchange_weak(current, new, success, failure)
    }
}

impl AtomicNumeric<u32> for AtomicU32 {
    fn fetch_add(&self, val: u32, order: Ordering) -> u32 {
        self.fetch_add(val, order)
    }
    fn fetch_sub(&self, val: u32, order: Ordering) -> u32 {
        self.fetch_sub(val, order)
    }
}

impl AtomicBitwise<u32> for AtomicU32 {
    fn fetch_and(&self, val: u32, order: Ordering) -> u32 {
        self.fetch_and(val, order)
    }
    fn fetch_or(&self, val: u32, order: Ordering) -> u32 {
        self.fetch_or(val, order)
    }
    fn fetch_xor(&self, val: u32, order: Ordering) -> u32 {
        self.fetch_xor(val, order)
    }
    fn fetch_nand(&self, val: u32, order: Ordering) -> u32 {
        self.fetch_nand(val, order)
    }
}

impl AtomicallySized for i32 {
    type AtomicVersion = AtomicI32;
}

impl Atomic<i32> for AtomicI32 {
    fn new(val: i32) -> Self {
        Self::new(val)
    }
    fn store(&self, val: i32, order: Ordering) {
        self.store(val, order)
    }
    fn load(&self, order: Ordering) -> i32 {
        self.load(order)
    }
    fn swap(&self, val: i32, order: Ordering) -> i32 {
        self.swap(val, order)
    }
    fn get_mut(&mut self) -> &mut i32 {
        self.get_mut()
    }
    fn into_inner(self) -> i32 {
        self.into_inner()
    }
    fn compare_and_swap(&self, current: i32, new: i32, order: Ordering) -> i32 {
        self.compare_and_swap(current, new, order)
    }
    fn compare_exchange(
        &self,
        current: i32,
        new: i32,
        success: Ordering,
        failure: Ordering,
    ) -> Result<i32, i32> {
        self.compare_exchange(current, new, success, failure)
    }
    fn compare_exchange_weak(
        &self,
        current: i32,
        new: i32,
        success: Ordering,
        failure: Ordering,
    ) -> Result<i32, i32> {
        self.compare_exchange_weak(current, new, success, failure)
    }
}

impl AtomicNumeric<i32> for AtomicI32 {
    fn fetch_add(&self, val: i32, order: Ordering) -> i32 {
        self.fetch_add(val, order)
    }
    fn fetch_sub(&self, val: i32, order: Ordering) -> i32 {
        self.fetch_sub(val, order)
    }
}

impl AtomicBitwise<i32> for AtomicI32 {
    fn fetch_and(&self, val: i32, order: Ordering) -> i32 {
        self.fetch_and(val, order)
    }
    fn fetch_or(&self, val: i32, order: Ordering) -> i32 {
        self.fetch_or(val, order)
    }
    fn fetch_xor(&self, val: i32, order: Ordering) -> i32 {
        self.fetch_xor(val, order)
    }
    fn fetch_nand(&self, val: i32, order: Ordering) -> i32 {
        self.fetch_nand(val, order)
    }
}

impl AtomicallySized for u16 {
    type AtomicVersion = AtomicU16;
}

impl Atomic<u16> for AtomicU16 {
    fn new(val: u16) -> Self {
        Self::new(val)
    }
    fn store(&self, val: u16, order: Ordering) {
        self.store(val, order)
    }
    fn load(&self, order: Ordering) -> u16 {
        self.load(order)
    }
    fn swap(&self, val: u16, order: Ordering) -> u16 {
        self.swap(val, order)
    }
    fn get_mut(&mut self) -> &mut u16 {
        self.get_mut()
    }
    fn into_inner(self) -> u16 {
        self.into_inner()
    }
    fn compare_and_swap(&self, current: u16, new: u16, order: Ordering) -> u16 {
        self.compare_and_swap(current, new, order)
    }
    fn compare_exchange(
        &self,
        current: u16,
        new: u16,
        success: Ordering,
        failure: Ordering,
    ) -> Result<u16, u16> {
        self.compare_exchange(current, new, success, failure)
    }
    fn compare_exchange_weak(
        &self,
        current: u16,
        new: u16,
        success: Ordering,
        failure: Ordering,
    ) -> Result<u16, u16> {
        self.compare_exchange_weak(current, new, success, failure)
    }
}

impl AtomicNumeric<u16> for AtomicU16 {
    fn fetch_add(&self, val: u16, order: Ordering) -> u16 {
        self.fetch_add(val, order)
    }
    fn fetch_sub(&self, val: u16, order: Ordering) -> u16 {
        self.fetch_sub(val, order)
    }
}

impl AtomicBitwise<u16> for AtomicU16 {
    fn fetch_and(&self, val: u16, order: Ordering) -> u16 {
        self.fetch_and(val, order)
    }
    fn fetch_or(&self, val: u16, order: Ordering) -> u16 {
        self.fetch_or(val, order)
    }
    fn fetch_xor(&self, val: u16, order: Ordering) -> u16 {
        self.fetch_xor(val, order)
    }
    fn fetch_nand(&self, val: u16, order: Ordering) -> u16 {
        self.fetch_nand(val, order)
    }
}

impl AtomicallySized for i16 {
    type AtomicVersion = AtomicI16;
}

impl Atomic<i16> for AtomicI16 {
    fn new(val: i16) -> Self {
        Self::new(val)
    }
    fn store(&self, val: i16, order: Ordering) {
        self.store(val, order)
    }
    fn load(&self, order: Ordering) -> i16 {
        self.load(order)
    }
    fn swap(&self, val: i16, order: Ordering) -> i16 {
        self.swap(val, order)
    }
    fn get_mut(&mut self) -> &mut i16 {
        self.get_mut()
    }
    fn into_inner(self) -> i16 {
        self.into_inner()
    }
    fn compare_and_swap(&self, current: i16, new: i16, order: Ordering) -> i16 {
        self.compare_and_swap(current, new, order)
    }
    fn compare_exchange(
        &self,
        current: i16,
        new: i16,
        success: Ordering,
        failure: Ordering,
    ) -> Result<i16, i16> {
        self.compare_exchange(current, new, success, failure)
    }
    fn compare_exchange_weak(
        &self,
        current: i16,
        new: i16,
        success: Ordering,
        failure: Ordering,
    ) -> Result<i16, i16> {
        self.compare_exchange_weak(current, new, success, failure)
    }
}

impl AtomicNumeric<i16> for AtomicI16 {
    fn fetch_add(&self, val: i16, order: Ordering) -> i16 {
        self.fetch_add(val, order)
    }
    fn fetch_sub(&self, val: i16, order: Ordering) -> i16 {
        self.fetch_sub(val, order)
    }
}

impl AtomicBitwise<i16> for AtomicI16 {
    fn fetch_and(&self, val: i16, order: Ordering) -> i16 {
        self.fetch_and(val, order)
    }
    fn fetch_or(&self, val: i16, order: Ordering) -> i16 {
        self.fetch_or(val, order)
    }
    fn fetch_xor(&self, val: i16, order: Ordering) -> i16 {
        self.fetch_xor(val, order)
    }
    fn fetch_nand(&self, val: i16, order: Ordering) -> i16 {
        self.fetch_nand(val, order)
    }
}

impl AtomicallySized for u8 {
    type AtomicVersion = AtomicU8;
}

impl Atomic<u8> for AtomicU8 {
    fn new(val: u8) -> Self {
        Self::new(val)
    }
    fn store(&self, val: u8, order: Ordering) {
        self.store(val, order)
    }
    fn load(&self, order: Ordering) -> u8 {
        self.load(order)
    }
    fn swap(&self, val: u8, order: Ordering) -> u8 {
        self.swap(val, order)
    }
    fn get_mut(&mut self) -> &mut u8 {
        self.get_mut()
    }
    fn into_inner(self) -> u8 {
        self.into_inner()
    }
    fn compare_and_swap(&self, current: u8, new: u8, order: Ordering) -> u8 {
        self.compare_and_swap(current, new, order)
    }
    fn compare_exchange(
        &self,
        current: u8,
        new: u8,
        success: Ordering,
        failure: Ordering,
    ) -> Result<u8, u8> {
        self.compare_exchange(current, new, success, failure)
    }
    fn compare_exchange_weak(
        &self,
        current: u8,
        new: u8,
        success: Ordering,
        failure: Ordering,
    ) -> Result<u8, u8> {
        self.compare_exchange_weak(current, new, success, failure)
    }
}

impl AtomicNumeric<u8> for AtomicU8 {
    fn fetch_add(&self, val: u8, order: Ordering) -> u8 {
        self.fetch_add(val, order)
    }
    fn fetch_sub(&self, val: u8, order: Ordering) -> u8 {
        self.fetch_sub(val, order)
    }
}

impl AtomicBitwise<u8> for AtomicU8 {
    fn fetch_and(&self, val: u8, order: Ordering) -> u8 {
        self.fetch_and(val, order)
    }
    fn fetch_or(&self, val: u8, order: Ordering) -> u8 {
        self.fetch_or(val, order)
    }
    fn fetch_xor(&self, val: u8, order: Ordering) -> u8 {
        self.fetch_xor(val, order)
    }
    fn fetch_nand(&self, val: u8, order: Ordering) -> u8 {
        self.fetch_nand(val, order)
    }
}

impl AtomicallySized for i8 {
    type AtomicVersion = AtomicI8;
}

impl Atomic<i8> for AtomicI8 {
    fn new(val: i8) -> Self {
        Self::new(val)
    }
    fn store(&self, val: i8, order: Ordering) {
        self.store(val, order)
    }
    fn load(&self, order: Ordering) -> i8 {
        self.load(order)
    }
    fn swap(&self, val: i8, order: Ordering) -> i8 {
        self.swap(val, order)
    }
    fn get_mut(&mut self) -> &mut i8 {
        self.get_mut()
    }
    fn into_inner(self) -> i8 {
        self.into_inner()
    }
    fn compare_and_swap(&self, current: i8, new: i8, order: Ordering) -> i8 {
        self.compare_and_swap(current, new, order)
    }
    fn compare_exchange(
        &self,
        current: i8,
        new: i8,
        success: Ordering,
        failure: Ordering,
    ) -> Result<i8, i8> {
        self.compare_exchange(current, new, success, failure)
    }
    fn compare_exchange_weak(
        &self,
        current: i8,
        new: i8,
        success: Ordering,
        failure: Ordering,
    ) -> Result<i8, i8> {
        self.compare_exchange_weak(current, new, success, failure)
    }
}

impl AtomicNumeric<i8> for AtomicI8 {
    fn fetch_add(&self, val: i8, order: Ordering) -> i8 {
        self.fetch_add(val, order)
    }
    fn fetch_sub(&self, val: i8, order: Ordering) -> i8 {
        self.fetch_sub(val, order)
    }
}

impl AtomicBitwise<i8> for AtomicI8 {
    fn fetch_and(&self, val: i8, order: Ordering) -> i8 {
        self.fetch_and(val, order)
    }
    fn fetch_or(&self, val: i8, order: Ordering) -> i8 {
        self.fetch_or(val, order)
    }
    fn fetch_xor(&self, val: i8, order: Ordering) -> i8 {
        self.fetch_xor(val, order)
    }
    fn fetch_nand(&self, val: i8, order: Ordering) -> i8 {
        self.fetch_nand(val, order)
    }
}

impl AtomicallySized for bool {
    type AtomicVersion = AtomicBool;
}

impl Atomic<bool> for AtomicBool {
    fn new(val: bool) -> Self {
        Self::new(val)
    }
    fn store(&self, val: bool, order: Ordering) {
        self.store(val, order)
    }
    fn load(&self, order: Ordering) -> bool {
        self.load(order)
    }
    fn swap(&self, val: bool, order: Ordering) -> bool {
        self.swap(val, order)
    }
    fn get_mut(&mut self) -> &mut bool {
        self.get_mut()
    }
    fn into_inner(self) -> bool {
        self.into_inner()
    }
    fn compare_and_swap(&self, current: bool, new: bool, order: Ordering) -> bool {
        self.compare_and_swap(current, new, order)
    }
    fn compare_exchange(
        &self,
        current: bool,
        new: bool,
        success: Ordering,
        failure: Ordering,
    ) -> Result<bool, bool> {
        self.compare_exchange(current, new, success, failure)
    }
    fn compare_exchange_weak(
        &self,
        current: bool,
        new: bool,
        success: Ordering,
        failure: Ordering,
    ) -> Result<bool, bool> {
        self.compare_exchange_weak(current, new, success, failure)
    }
}

impl AtomicBitwise<bool> for AtomicBool {
    fn fetch_and(&self, val: bool, order: Ordering) -> bool {
        self.fetch_and(val, order)
    }
    fn fetch_or(&self, val: bool, order: Ordering) -> bool {
        self.fetch_or(val, order)
    }
    fn fetch_xor(&self, val: bool, order: Ordering) -> bool {
        self.fetch_xor(val, order)
    }
    fn fetch_nand(&self, val: bool, order: Ordering) -> bool {
        self.fetch_nand(val, order)
    }
}

impl<T> AtomicallySized for *mut T {
    type AtomicVersion = AtomicPtr<T>;
}

impl<T> Atomic<*mut T> for AtomicPtr<T> {
    fn new(val: *mut T) -> Self {
        Self::new(val)
    }
    fn store(&self, val: *mut T, order: Ordering) {
        self.store(val, order)
    }
    fn load(&self, order: Ordering) -> *mut T {
        self.load(order)
    }
    fn swap(&self, val: *mut T, order: Ordering) -> *mut T {
        self.swap(val, order)
    }
    fn get_mut(&mut self) -> &mut *mut T {
        self.get_mut()
    }
    fn into_inner(self) -> *mut T {
        self.into_inner()
    }
    fn compare_and_swap(&self, current: *mut T, new: *mut T, order: Ordering) -> *mut T {
        self.compare_and_swap(current, new, order)
    }
    fn compare_exchange(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        self.compare_exchange(current, new, success, failure)
    }
    fn compare_exchange_weak(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        self.compare_exchange_weak(current, new, success, failure)
    }
}
