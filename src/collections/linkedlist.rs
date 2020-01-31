use std::mem;

#[derive(Debug, PartialEq)]
pub struct Node<V> {
	next: Option<Box<Node<V>>>,
	value: V,
}

impl<V> Node<V> {
	pub fn new(value: V) -> Node<V> {
		Node { next: None, value }
	}
}

pub struct Iter<'a, V> {
	node: Option<&'a Node<V>>,
}

impl<'a, V> Iterator for Iter<'a, V> {
	type Item = &'a V;

	fn next(&mut self) -> Option<Self::Item> {
		let node = self.node;
		if let Some(node) = node {
			if let Some(next) = &node.next {
				self.node = Some(next);
			} else {
				self.node = None;
			}
			Some(&node.value)
		} else {
			None
		}
	}
}

pub struct IterMut<'a, V> {
	node: Option<&'a mut Node<V>>,
}

impl<'a, V> Iterator for IterMut<'a, V> {
	type Item = &'a mut V;

	fn next(&mut self) -> Option<Self::Item> {
		let node = self.node.take();
		if let Some(current_node) = node {
			if let Some(next) = &mut current_node.next {
				self.node = Some(next);
			}
			Some(&mut current_node.value)
		} else {
			None
		}
	}
}

#[derive(Debug, PartialEq)]
pub struct LinkedList<V> {
	size: usize,
	head: Option<Box<Node<V>>>,
}

impl<V> LinkedList<V> {
	pub fn new() -> LinkedList<V> {
		LinkedList {
			size: 0,
			head: None,
		}
	}

	pub fn iter(&self) -> Iter<V> {
		let head = if let Some(head) = &self.head {
			Some(head.as_ref())
		} else {
			None
		};

		Iter { node: head }
	}

	pub fn iter_mut(&mut self) -> IterMut<V> {
		let head = if let Some(head) = &mut self.head {
			Some(head.as_mut())
		} else {
			None
		};

		IterMut{ node: head }
	}

	pub fn add(&mut self, value: V) {
		let node = Node {
			value,
			next: self.head.take(),
		};
		self.head = Some(Box::new(node));
		self.size += 1;
	}

	pub fn pop(&mut self) -> Option<V> {
		if let Some(head) = self.head.take() {
			self.head = head.next;
			Some(head.value)
		} else {
			None
		}
	}

	pub fn len(&self) -> usize {
		self.size
	}
	
	pub fn is_empty(&self) -> bool {
		self.size == 0
	}
}

impl<V: PartialEq + Copy> LinkedList<V> {
	pub fn replace_all(&mut self, target: &V, value: V) {
		for i in self.iter_mut() {
			if *i == *target {
				mem::replace(i, value);
			}
		}
	}
}

impl<V: PartialEq> LinkedList<V> {
	pub fn replace(&mut self, target: &V, value: V) {
		for i in self.iter_mut() {
			if *i == *target {
				mem::replace(i, value);
				return
			}
		}
	}

	pub fn contains(&self, v: &V) -> bool {
		for i in self.iter() {
			if *v == *i {
				return true
			}
		}
		false
	}

	pub fn remove(&mut self, v: &V) -> bool {
		let mut cursor = self.head.as_mut();
		let mut result = false;

		while let Some(c) = cursor {
			if let Some(next) = c.next.as_mut() {
				if next.value == *v {
					c.next = next.next.take();
					result = true;
					cursor = Some(c);
					continue;
				}
			}
			cursor = c.next.as_mut();
		}

		result
	}
}

impl<V: std::fmt::Display> LinkedList<V> {
	pub fn display(&self) {
		for i in self.iter() {
			print!("{} -> ", &i)
		}
		print!("_\n")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add() {
		let mut list = LinkedList::new();
		list.add(1);
		list.add(2);
		list.add(3);

		assert_eq!(list.len(), 3);
	}

	#[test]
	fn test_modify() {
		let mut list = LinkedList::new();
		list.add(1);
		list.add(2);
		list.add(3);

		for i in list.iter_mut() {
			if *i == 1 {
				*i = 10;
			}
		}

		let mut other = LinkedList::new();
		other.add(10);
		other.add(2);
		other.add(3);

		assert_eq!(list, other);
	}

	#[test]
	fn test_replace() {
		let mut list = LinkedList::new();
		list.add(1);
		list.add(2);
		list.add(3);
		list.replace(&1, 10);

		let mut other = LinkedList::new();
		other.add(10);
		other.add(2);
		other.add(3);

		assert_eq!(list, other);
	}

	#[test]
	fn test_replace_all() {
		let mut list = LinkedList::new();
		for _ in 0..10 {
			list.add(1);
		}
		list.replace_all(&1, 10);

		let mut other = LinkedList::new();
		for _ in 0..10 {
			other.add(10);
		}

		assert_eq!(list, other);
	}
}
