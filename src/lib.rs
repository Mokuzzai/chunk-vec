mod chunk;

pub use chunk::Chunk;

pub struct ChunkVec<T, const N: usize> {
	chunks: Vec<Chunk<T, N>>,
	cached_len: usize,
}

impl<T, const N: usize> ChunkVec<T, N> {
	pub fn new() -> Self {
		Self {
			chunks: Vec::new(),
			cached_len: 0,
		}
	}
	pub fn as_slice_of_slices(&self) -> &[&[T]] {
		chunk::cast_slice(&self.chunks)
	}
	pub fn as_slice_of_slices_mut(&mut self) -> &mut [&mut [T]] {
		chunk::cast_slice_mut(&mut self.chunks)
	}
	pub fn len(&self) -> usize {
		self.cached_len
	}
	pub fn push(&mut self, item: T) {
		let item = if let Some(last) = self.chunks.last_mut() {
			match last.push(item) {
				Ok(_) => return,
				Err(item) => item,
			}
		} else {
			item
		};

		let mut chunk = Chunk::<T, N>::new();

		let _ = chunk.push(item).unwrap_or_else(|_| panic!("`N` must not be zero"));

		self.chunks.push(chunk);
	}
	pub fn insert(&mut self, index: usize, item: T) {
		todo!()
	}
}
