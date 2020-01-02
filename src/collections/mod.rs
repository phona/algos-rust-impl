use std::iter::Iterator;

pub mod lists;

trait Collection<E> {
	fn size(&self) -> usize;

	fn contains(&self, e: &E) -> bool;

	fn add(&self, e: E) -> bool;

	fn remove(&self, e: &E) -> bool;

	fn is_empty(&self) -> bool {
		self.size() == 0
	}
}
