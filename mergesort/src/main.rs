extern crate rand;
use rand::{Rng, SeedableRng, StdRng};
use std::time::Instant;

mod mergesort;

const RANDOM_SEED: usize = 42;
const LARGE_ARRAY_SHIFT: usize = 20;
const NUM_REPS: usize = 10;

fn print_vector(vec: &Vec<f32>, length: usize) {
	print!("{{");
	for i in 0..(length - 1) {
		print!("{}, ", vec[i]);
	}
	print!("{}", vec[length - 1]);
	print!("}}\n");
}

fn correctness_tests(arg: String) {
	let mut arr0: Vec<f32> = vec![1.0];
	let mut arr1: Vec<f32> = vec![4.0, 1.0, 2.0, 5.0, 3.0];
	let mut arr2: Vec<f32> = vec![9.0, 4.0, 5.0, 2.0, 8.0, 1.0, 7.0, 6.0, 3.0];

	let arr0_sorted: Vec<f32> = vec![1.0];
	let arr1_sorted: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
	let arr2_sorted: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

	if arg == "singlethreaded" {
		println!("  Single-threaded version:");
		mergesort::mergesort_st(&mut arr0, 0, 1);
		mergesort::mergesort_st(&mut arr1, 0, 5);
		mergesort::mergesort_st(&mut arr2, 0, 9);
	} else if arg == "multithreaded" {
		println!("  Multithreaded version:");
		arr0 = mergesort::mergesort_mt(arr0, 0, 1);
		arr1 = mergesort::mergesort_mt(arr1, 0, 5);
		arr2 = mergesort::mergesort_mt(arr2, 0, 9);
	} else {
		println!("Incorrect argument!");
	}
	
	print!("  arr0: "); print_vector(&arr0, 1);
	assert_eq!(arr0, arr0_sorted);
	print!("  arr1: "); print_vector(&arr1, 5);
	assert_eq!(arr1, arr1_sorted);
	print!("  arr2: "); print_vector(&arr2, 9);
	assert_eq!(arr2, arr2_sorted);
}

fn stress_test(arg: String) {
	let seed: &[_] = &[RANDOM_SEED];
	let mut rng: StdRng = SeedableRng::from_seed(seed);
	match arg.as_ref() {
		"singlethreaded" => println!("  Single-threaded version:"),
		"multithreaded" => println!("  Multithreaded version:"),
		"builtin" => println!("  Builtin vector sort:"),
		_ => println!("Incorrect argument!")
	};
	for i in 0..NUM_REPS {
		let array_size: usize = 1 << (LARGE_ARRAY_SHIFT + i);
		let mut arr: Vec<f32> = Vec::with_capacity(array_size);
		for _ in 0..array_size { arr.push(0.0); }
		for j in 0..array_size {
			arr[j] = rng.gen::<f32>();
		}
		let time = Instant::now();
		if arg == "singlethreaded" {
			mergesort::mergesort_st(&mut arr, 0, array_size);
		} else if arg == "multithreaded" {
			arr = mergesort::mergesort_mt(arr, 0, array_size);
		} else if arg == "builtin" {
			arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
		}
		println!("    Round {}:", i);
		println!("    Time elapsed: {}", time.elapsed().as_secs());
	}
}

fn main() {
	println!("Correctness tests:");
	correctness_tests("singlethreaded".into());
	correctness_tests("multithreaded".into());
	println!("Stress tests:");
	stress_test("singlethreaded".into());
	stress_test("multithreaded".into());
	stress_test("builtin".into());
  println!("All done!");
}
