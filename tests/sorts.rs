extern crate sorts;

use sorts::sorts::*;

fn sort_test<F: Fn(&mut [T]), T: Sortable + std::fmt::Debug>(f: F, arr: &mut [T], expect: &[T]) {
	f(arr);
	assert_eq!(arr, expect);
}

#[cfg(test)]
mod bubble_sort_test_cases {
	use super::{bubble_sort as sort_func, sort_test};
	#[test]
	fn positive_numbers() {
		sort_test(
			sort_func,
			&mut vec![5, 5, 7, 8, 2, 4, 1],
			&vec![1, 2, 4, 5, 5, 7, 8],
		)
	}

	#[test]
	fn negative_numbers_only() {
		sort_test(
			sort_func,
			&mut vec![-1, -3, -5, -7, -9, -5],
			&vec![-9, -7, -5, -5, -3, -1],
		);
	}

	#[test]
	fn negative_and_positive_numbers() {
		sort_test(
			sort_func,
			&mut vec![-6, -5, -4, 0, 5, 5, 7, 8, 2, 4, 1],
			&vec![-6, -5, -4, 0, 1, 2, 4, 5, 5, 7, 8],
		);
	}

	#[test]
	fn same_numbers() {
		sort_test(sort_func, &mut vec![1, 1, 1, 1], &vec![1, 1, 1, 1]);
	}

	#[test]
	fn empty_list() {
		let mut arr1: Vec<i32> = Vec::new();
		let arr2: Vec<i32> = Vec::new();
		sort_test(sort_func, &mut arr1, &arr2);
	}
}

#[cfg(test)]
mod insert_sort_test_cases {
	use super::{insert_sort as sort_func, sort_test};
	#[test]
	fn positive_numbers() {
		sort_test(
			sort_func,
			&mut vec![5, 5, 7, 8, 2, 4, 1],
			&vec![1, 2, 4, 5, 5, 7, 8],
		)
	}

	#[test]
	fn negative_numbers_only() {
		sort_test(
			sort_func,
			&mut vec![-1, -3, -5, -7, -9, -5],
			&vec![-9, -7, -5, -5, -3, -1],
		);
	}

	#[test]
	fn negative_and_positive_numbers() {
		sort_test(
			sort_func,
			&mut vec![-6, -5, -4, 0, 5, 5, 7, 8, 2, 4, 1],
			&vec![-6, -5, -4, 0, 1, 2, 4, 5, 5, 7, 8],
		);
	}

	#[test]
	fn same_numbers() {
		sort_test(sort_func, &mut vec![1, 1, 1, 1], &vec![1, 1, 1, 1]);
	}

	#[test]
	fn empty_list() {
		let mut arr1: Vec<i32> = Vec::new();
		let arr2: Vec<i32> = Vec::new();
		sort_test(sort_func, &mut arr1, &arr2);
	}
}

#[cfg(test)]
mod select_sort_test_cases {
	use super::{select_sort as sort_func, sort_test};
	#[test]
	fn positive_numbers() {
		sort_test(
			sort_func,
			&mut vec![5, 5, 7, 8, 2, 4, 1],
			&vec![1, 2, 4, 5, 5, 7, 8],
		)
	}

	#[test]
	fn negative_numbers_only() {
		sort_test(
			sort_func,
			&mut vec![-1, -3, -5, -7, -9, -5],
			&vec![-9, -7, -5, -5, -3, -1],
		);
	}

	#[test]
	fn negative_and_positive_numbers() {
		sort_test(
			sort_func,
			&mut vec![-6, -5, -4, 0, 5, 5, 7, 8, 2, 4, 1],
			&vec![-6, -5, -4, 0, 1, 2, 4, 5, 5, 7, 8],
		);
	}

	#[test]
	fn same_numbers() {
		sort_test(sort_func, &mut vec![1, 1, 1, 1], &vec![1, 1, 1, 1]);
	}

	#[test]
	fn empty_list() {
		let mut arr1: Vec<i32> = Vec::new();
		let arr2: Vec<i32> = Vec::new();
		sort_test(sort_func, &mut arr1, &arr2);
	}
}

#[cfg(test)]
mod merge_sort_test_cases {
	use super::{merge_sort as sort_func, sort_test};
	#[test]
	fn positive_numbers() {
		sort_test(
			sort_func,
			&mut vec![5, 5, 7, 8, 2, 4, 1],
			&vec![1, 2, 4, 5, 5, 7, 8],
		)
	}

	#[test]
	fn negative_numbers_only() {
		sort_test(
			sort_func,
			&mut vec![-1, -3, -5, -7, -9, -5],
			&vec![-9, -7, -5, -5, -3, -1],
		);
	}

	#[test]
	fn negative_and_positive_numbers() {
		sort_test(
			sort_func,
			&mut vec![-6, -5, -4, 0, 5, 5, 7, 8, 2, 4, 1],
			&vec![-6, -5, -4, 0, 1, 2, 4, 5, 5, 7, 8],
		);
	}

	#[test]
	fn same_numbers() {
		sort_test(sort_func, &mut vec![1, 1, 1, 1], &vec![1, 1, 1, 1]);
	}

	#[test]
	fn empty_list() {
		let mut arr1: Vec<i32> = Vec::new();
		let arr2: Vec<i32> = Vec::new();
		sort_test(sort_func, &mut arr1, &arr2);
	}
}

#[cfg(test)]
mod quick_sort_test_cases {
	use super::{quick_sort as sort_func, sort_test};
	#[test]
	fn positive_numbers() {
		sort_test(
			sort_func,
			&mut vec![5, 5, 7, 8, 2, 4, 1],
			&vec![1, 2, 4, 5, 5, 7, 8],
		)
	}

	#[test]
	fn negative_numbers_only() {
		sort_test(
			sort_func,
			&mut vec![-1, -3, -5, -7, -9, -5],
			&vec![-9, -7, -5, -5, -3, -1],
		);
	}

	#[test]
	fn negative_and_positive_numbers() {
		sort_test(
			sort_func,
			&mut vec![-6, -5, -4, 0, 5, 5, 7, 8, 2, 4, 1],
			&vec![-6, -5, -4, 0, 1, 2, 4, 5, 5, 7, 8],
		);
	}

	#[test]
	fn same_numbers() {
		sort_test(sort_func, &mut vec![1, 1, 1, 1], &vec![1, 1, 1, 1]);
	}

	#[test]
	fn empty_list() {
		let mut arr1: Vec<i32> = Vec::new();
		let arr2: Vec<i32> = Vec::new();
		sort_test(sort_func, &mut arr1, &arr2);
	}
}
