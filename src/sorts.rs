pub trait Sortable: PartialOrd + Copy {}
impl<T: PartialOrd + Copy> Sortable for T {}

fn exchange<T: Copy>(arr: &mut [T], i: usize, j: usize) {
	let tmp = arr[i];
	arr[i] = arr[j];
	arr[j] = tmp;
}

pub fn bubble_sort<T: Sortable>(arr: &mut [T]) {
	if arr.len() == 0 {
		return;
	}

	let mut counter = 0;
	loop {
		for i in 0..arr.len() - 1 {
			if arr[i] > arr[i + 1] {
				exchange(arr, i, i + 1);
				counter += 1;
			}
		}

		if counter == 0 {
			break;
		}
		counter = 0;
	}
}

pub fn insert_sort<T: Sortable>(arr: &mut [T]) {
	for i in 0..arr.len() {
		for j in 0..i {
			if arr[j] > arr[i] {
				exchange(arr, i, j);
			}
		}
	}
}

pub fn select_sort<T: Sortable>(arr: &mut [T]) {
	for i in 0..arr.len() {
		for j in i..arr.len() {
			if arr[i] > arr[j] {
				exchange(arr, i, j);
			}
		}
	}
}

fn _merge_sort<T: Sortable + std::fmt::Debug>(arr: &mut [T], tmp_arr: &mut[T]) {
	let size = arr.len();
	if size > 1 {
		let mid = size / 2;
		_merge_sort(&mut arr[0..mid], tmp_arr);
		_merge_sort(&mut arr[mid..size], tmp_arr);

		let mut left_index = 0;
		let mut right_index = mid;
		for i in 0..size {
			if left_index < mid && right_index < size {
				if arr[left_index] > arr[right_index] {
					tmp_arr[i] = arr[right_index];
					right_index += 1;
				} else {
					tmp_arr[i] = arr[left_index];
					left_index += 1;
				}
			} else {
				if left_index < mid {
					tmp_arr[i] = arr[left_index];
					left_index += 1;
				}

				if right_index < size {
					tmp_arr[i] = arr[right_index];
					right_index += 1;
				}
			}
		}

		for i in 0..size {
			arr[i] = tmp_arr[i];
		}
	}
}

pub fn merge_sort<T: Sortable + std::fmt::Debug>(arr: &mut [T]) {
	// create temporary array for store partial sorted result
	let mut tmp_arr = Vec::with_capacity(arr.len());
	for i in 0..arr.len() {
		tmp_arr.push(arr[i]);
	}
	_merge_sort(arr, &mut tmp_arr);
}
