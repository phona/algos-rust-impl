pub trait Sortable: PartialOrd + Copy + std::fmt::Debug {}
impl<T: PartialOrd + Copy + std::fmt::Debug> Sortable for T {}

pub fn bubble_sort<T: Sortable>(arr: &mut [T]) {
	if arr.len() == 0 {
		return;
	}

	let mut counter = 0;
	loop {
		for i in 0..arr.len() - 1 {
			if arr[i] > arr[i + 1] {
				arr.swap(i + 1, i);
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
				arr.swap(i, j);
			}
		}
	}
}

pub fn select_sort<T: Sortable>(arr: &mut [T]) {
	for i in 0..arr.len() {
		for j in i..arr.len() {
			if arr[i] > arr[j] {
				arr.swap(i, j);
			}
		}
	}
}

fn _merge_sort<T: Sortable>(arr: &mut [T], tmp_arr: &mut [T]) {
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

pub fn merge_sort<T: Sortable>(arr: &mut [T]) {
	// create temporary array for store partial sorted result
	let mut tmp_arr = Vec::with_capacity(arr.len());
	for i in 0..arr.len() {
		tmp_arr.push(arr[i]);
	}
	_merge_sort(arr, &mut tmp_arr);
}

fn partition<T: Sortable>(arr: &mut [T]) -> usize {
	let lo = 0;
	let hi = arr.len() - 1;
	let pivot = arr[hi];
	let mut low_item_index = None;

	// rearrage array
	for i in lo..hi {
		if arr[i] < pivot {
			low_item_index = low_item_index
				.and_then(|x| Some(x + 1))
				.or_else(|| Some(lo))
				.map(|x| {
					arr.swap(x, i);
					x
				});
		}
	}

	return low_item_index
		.and_then(|x| Some(x + 1))
		.or_else(|| Some(lo))
		.map(|x| {
			arr.swap(x, hi);
			x
		})
		.unwrap();
}

pub fn quick_sort<T: Sortable>(arr: &mut [T]) {
	let len = arr.len();
	if len > 1 {
		let pivot = partition(arr);
		quick_sort(&mut arr[0..pivot]);
		quick_sort(&mut arr[pivot + 1..len]);
	}
}
