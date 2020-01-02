use super::Collection;
use super::List;
use std::iter::Iterator;

pub struct Node<E> {
	next: Option<Box<Node<E>>>,
	value: Option<E>,
}

impl<E> Iterator for Node<E> {
	type Item = Box<Node<E>>;

	fn next(&mut self) -> Option<Self::Item> {
		self.next
	}
}

pub struct LinkedList<E> {
	size: usize,
	head: Option<Box<Node<E>>>,
}

impl<'a, E, V> List<'a, E, V> for LinkedList<E> {
	fn pop(&mut self) -> Option<V> {
		None
	}

	fn get(&'a self, index: usize) -> Option<&'a V> {
		None
	}

	fn get_mut(&'a mut self, index: usize) -> Option<&'a mut V> {
		None
	}

	fn set(&mut self, index: usize, e: E) -> bool {
		false
	}

	fn index_of(&self, v: &V) -> i32 {
		-1
	}

	fn contains(&self, v: &V) -> bool {
		false
	}
}

impl<E, Iter> Collection<E, Iter> for LinkedList<E>
where
	Iter: Iterator,
{
	fn size(&self) -> usize {
		self.size
	}

	fn contains(&self, e: &E) -> bool {
		false
	}

	fn add(&self, e: E) -> bool {
		false
	}

	fn remove(&self, e: &E) -> bool {
		false
	}

	fn iter(&mut self) -> Iter {
	}

	fn iter_mut(&mut self) -> &mut Iter {}
}
