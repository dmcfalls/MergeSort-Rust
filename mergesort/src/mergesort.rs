use std::thread;
use std::sync::mpsc;
use std::cell::RefCell;

const MAX_THREADS: usize = 4;

fn merge(arr: &mut Vec<f32>, left: usize, mid: usize, right: usize, scratch: &mut Vec<f32>) {
	for i in left..mid {
		scratch[i] = arr[i];
	}
	for j in mid..right {
		scratch[j] = arr[j];
	}

	let mut left_ptr = left;
	let mut right_ptr = mid;
	let mut merge_ptr = left;

	while left_ptr < mid && right_ptr < right {
		if scratch[left_ptr] < scratch[right_ptr] {
			arr[merge_ptr] = scratch[left_ptr];
			left_ptr += 1;
		} else {
			arr[merge_ptr] = scratch[right_ptr];
			right_ptr += 1;
		}
		merge_ptr += 1;
	}

	while left_ptr < mid {
		arr[merge_ptr] = scratch[left_ptr];
		left_ptr += 1;
		merge_ptr += 1;
	}
	while right_ptr < right {
		arr[merge_ptr] = scratch[right_ptr];
		right_ptr += 1;
		merge_ptr += 1;
	}
}

fn mergesort_st_helper(arr: &mut Vec<f32>, left: usize, right: usize, scratch: &mut Vec<f32>) {
	if right - left > 1 {
		let mid = (left + right) / 2;

		mergesort_st_helper(arr, left, mid, scratch);
		mergesort_st_helper(arr, mid, right, scratch);

		merge(arr, left, mid, right, scratch);
	}
}

pub fn mergesort_st(arr: &mut Vec<f32>, left: usize, right: usize) {
	let size = right - left;
	let mut scratch: Vec<f32> = vec![0.0; size];
	mergesort_st_helper(arr, left, right, &mut scratch);
}

fn merge_mt(arr_left: Vec<f32>, arr_right: Vec<f32>) -> Vec<f32> {
	let mut result = Vec::new();
	let mut left_ptr = 0;
	let mut right_ptr = 0;

	while left_ptr < arr_left.len() && right_ptr < arr_right.len() {
		if arr_left[left_ptr] < arr_right[right_ptr] {
			result.push(arr_left[left_ptr]);
			left_ptr += 1;
		} else {
			result.push(arr_right[right_ptr]);
			right_ptr += 1;
		}
	}

	while left_ptr < arr_left.len() {
		result.push(arr_left[left_ptr]);
		left_ptr += 1;
	}

	while right_ptr < arr_right.len() {
		result.push(arr_right[right_ptr]);
		right_ptr += 1;
	}

	return result;
}

fn mergesort_mt_helper(arr: Vec<f32>, left: usize, right: usize, depth: usize) -> Vec<f32> {
	if right - left > 1 {
		let mid = (left + right) / 2;
		let new_depth = depth + 1;

		let (mut arr_left, mut arr_right) = match arr.split_at(mid) {
			(l, r) => (l.to_vec(), r.to_vec())
		};
		let arr_left_len = arr_left.len();
		let arr_right_len = arr_right.len();

		if new_depth < MAX_THREADS {
			let (sender, receiver) = mpsc::channel();
			let left_ptr = RefCell::new(arr_left);
			let _ = thread::spawn(move || {
				let left_sorted = mergesort_mt_helper(left_ptr.into_inner(), 0, arr_left_len, new_depth);
				sender.send(left_sorted).unwrap();
			});
			arr_right = mergesort_mt_helper(arr_right, 0, arr_right_len, new_depth);
			arr_left = receiver.recv().unwrap();
		} else {
			arr_left = mergesort_mt_helper(arr_left, 0, arr_left_len, new_depth);
			arr_right = mergesort_mt_helper(arr_right, 0, arr_right_len, new_depth);
		}

		return merge_mt(arr_left, arr_right);
	}
	return arr;
}

pub fn mergesort_mt(arr: Vec<f32>, left: usize, right: usize) -> Vec<f32> {
	return mergesort_mt_helper(arr, left, right, 0);
}