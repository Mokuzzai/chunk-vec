
use std::ptr::NonNull;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ops::DerefMut;

/// Box<[T]> that can grow in capacity
#[repr(transparent)]
pub struct Chunk<T, const N: usize> {
	slice: NonNull<[T]>,
}

impl<T, const N: usize> Chunk<T, N> {
	pub unsafe fn from_raw_parts(ptr: *mut T, len: usize) -> Self {
		debug_assert!(len <= N);
		assert!(N < isize::MAX as usize);

		Self {
			slice: NonNull::slice_from_raw_parts(NonNull::new_unchecked(ptr), len),
		}
	}
	pub fn cap(&self) -> usize {
		N
	}
	pub fn as_slice(&self) -> &[T] {
		unsafe { self.slice.as_ref() }
	}
	pub fn as_mut_slice(&mut self) -> &mut [T] {
		unsafe { self.slice.as_mut() }
	}
	pub fn new() -> Self {
		unsafe { Self::from_raw_parts(Box::into_raw(Box::new(MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init())).cast(), 0) }
	}
	/// NOTE: this `ptr` has provanence to the whole allocated buffer
	pub fn as_mut_ptr(&mut self) -> *mut T {
		self.slice.as_ptr().cast()
	}
	/// NOTE: this `ptr` has provanence to the whole allocated buffer
	pub fn as_ptr(&self) -> *const T {
		self.slice.as_ptr().cast()
	}
	pub fn is_full(&self) -> bool {
		self.len() == self.cap()
	}
	fn parse_non_full(&self, item: T) -> Result<T, T> {
		if self.is_full() {
			Err(item)
		} else {
			Ok(item)
		}
	}
	pub unsafe fn set_len(&mut self, new_len: usize) {
		let ptr = self.as_mut_ptr();

		unsafe { self.slice = NonNull::slice_from_raw_parts(NonNull::new_unchecked(ptr), new_len) }
	}
	pub fn push(&mut self, item: T) -> Result<&mut T, T> {
		let item = self.parse_non_full(item)?;

		unsafe {
			let ptr = self.as_mut_ptr().add(self.len());

			ptr.write(item);

			Ok(&mut *ptr)
		}
	}
	pub fn insert(&mut self, index: usize, item: T) -> (&mut T, Option<T>) {
		if !(index < N) {
			panic!(
				"insert overflows allocation (index is {} but length is {})",
				index, N
			);
		}

		let mut ret = None;

		unsafe {
			if self.is_full() {
				ret = Some(self.as_mut_ptr().add(self.len() - 1).read())
			} else {
				self.set_len(self.len() + 1)
			}
		}

		let ptr = unsafe { self.as_mut_ptr().add(index) };

		unsafe {
			ptr.copy_to(ptr.add(1), self.len() - index - 1);
			ptr.write(item);

			(&mut *ptr, ret)
		}
	}
}

impl<T, const N: usize> Deref for Chunk<T, N> {
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

impl<T, const N: usize> DerefMut for Chunk<T, N> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_slice()
	}
}

pub fn cast_slice<T, const N: usize>(slice: &[Chunk<T, N>]) -> &[&[T]] {
	let ptr = slice.as_ptr();
	let len = slice.len();

	unsafe { std::slice::from_raw_parts(ptr.cast(), len) }
}

pub fn cast_slice_mut<T, const N: usize>(slice: &mut [Chunk<T, N>]) -> &mut [&mut [T]] {
	let ptr = slice.as_mut_ptr();
	let len = slice.len();

	unsafe { std::slice::from_raw_parts_mut(ptr.cast(), len) }
}
