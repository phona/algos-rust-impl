use super::super::Collection;
use super::List;
use std::borrow::{Borrow, BorrowMut};
use std::iter::Iterator;

pub struct Node<E> {
	next: Option<Box<Node<E>>>,
	value: E,
}

impl<E> Node<E> {
	pub fn new(value: E) -> Node<E> {
		Node { next: None, value }
	}
}

pub struct Iter<'a, E> {
	node: Option<&'a Node<E>>,
}

impl<'a, E> Iterator for Iter<'a, E> {
	type Item = &'a Node<E>;

	fn next(&mut self) -> Option<Self::Item> {
		let node = self.node;
		if let Some(node) = node {
			if let Some(next) = &node.next {
				self.node = Some(&next);
			} else {
				self.node = None;
			}
			Some(node)
		} else {
			None
		}
	}
}

pub struct IterMut<'a, E> {
	node: Option<&'a mut Node<E>>,
}

impl<'a, E> Iterator for IterMut<'a, E> {
	type Item = &'a mut Node<E>;

	fn next(&mut self) -> Option<Self::Item> {
		let node = self.node.take();
		if let Some(current_node) = node {
			if let Some(next) = &mut current_node.next {
				self.node = Some(next.borrow_mut());
			}
			Some(current_node)
		} else {
			None
		}
	}
}

pub struct LinkedList<E> {
	size: usize,
	head: Option<Box<Node<E>>>,
}

impl<E> LinkedList<E> {
	pub fn new() -> LinkedList<E> {
		LinkedList {
			size: 0,
			head: None,
		}
	}

	pub fn iter(&self) -> Iter<E> {
		let head = if let Some(head) = &self.head {
			Some(head.borrow())
		} else {
			None
		};

		Iter { node: head }
	}
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

impl<E> Collection<E> for LinkedList<E> {
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
}
