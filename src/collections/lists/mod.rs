use super::Collection;

pub mod linked_lists;

pub trait List<'a, E, V> {
	fn pop(&mut self) -> Option<V>;

	fn get(&'a self, index: usize) -> Option<&'a V>;

	fn get_mut(&'a mut self, index: usize) -> Option<&'a mut V>;

	fn set(&mut self, index: usize, e: E) -> bool;

	fn index_of(&self, v: &V) -> i32;

	fn contains(&self, v: &V) -> bool;
}
