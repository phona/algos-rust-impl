use std::iter::Iterator;

pub mod lists;

pub trait Collection<E, Iter: Iterator> {
	fn size(&self) -> usize;

	fn contains(&self, e: &E) -> bool;

	fn add(&self, e: E) -> bool;

	fn remove(&self, e: &E) -> bool;

	fn iter(&self) -> Iter;

	fn iter_mut(&mut self) -> Iter;

	fn is_empty(&self) -> bool {
		self.size() == 0
	}
}
