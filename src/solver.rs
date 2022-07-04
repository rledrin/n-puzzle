use std::{collections::HashMap, fs, hash::Hash};

use rand::prelude::SliceRandom;

use crate::algo::Algo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
	pub x: u32,
	pub y: u32,
}

pub struct Puzzle {
	pub start_time: std::time::Instant,
	pub total_open: u64,
	pub current_open: u64,
	pub coords: HashMap<u32, Vec2>,
	pub size: u32,
	pub initial_blank: Vec2,
	pub initial_state: Vec<u32>,
	pub silent: bool,
	pub algo: Algo,
}

impl Puzzle {
	pub fn new(
		file_path: Option<String>,
		random: bool,
		size: u32,
		silent: bool,
		algo: Algo,
	) -> Puzzle {
		let mut initial_blank = Vec2 { x: 0, y: 0 };

		let mut initial_size = (3u32, false);

		let mut initial_state = Vec::with_capacity((initial_size.0 * initial_size.0) as usize);
		if let Some(path) = file_path {
			//TODO: improve parsing
			let input = fs::read_to_string(path).expect("Unable to read the puzzle file");
			let lines = input.lines();

			for l in lines {
				for number in l.split_whitespace() {
					println!("number: {}", number);
					println!("number.contains('#'): {}\n", number.contains('#'));
					if number.contains('#') {
						break;
					}

					if !initial_size.1 {
						initial_size.0 = number.parse::<u32>().unwrap();
						initial_size.1 = true;
						continue;
					}
					let number = number.parse::<u32>().unwrap();
					if number == 0 {
						initial_blank = Vec2 {
							x: initial_state.len() as u32 % initial_size.0,
							y: initial_state.len() as u32 / initial_size.0,
						};
					}
					initial_state.push(number);
				}
			}
			if !Puzzle::sovable(&initial_state, initial_size.0, initial_blank) {
				println!("This is not solvable");
				std::process::exit(0);
			}
		} else if random {
			Puzzle::generate_random(&mut initial_state, size, &mut initial_blank);
			initial_size.0 = size;
		}
		Puzzle {
			start_time: std::time::Instant::now(),
			total_open: 1,
			current_open: 1,
			coords: Puzzle::init_coordinates(initial_size.0),
			size: initial_size.0,
			initial_blank,
			initial_state,
			silent,
			algo,
		}
	}

	pub fn solve(&mut self) {
		println!(
			"Initial heuristic: {}",
			f32::from_bits(self.calculate_heuristic(&self.initial_state))
		);
		println!("Initial state: ");
		self.print_state(&self.initial_state);
		println!();
		let algo: Algo = unsafe { std::mem::transmute_copy(&self.algo) };
		algo(self);
		drop(algo);
	}

	pub fn print_state(&self, v: &[u32]) {
		for i in 0..self.size {
			for j in 0..self.size {
				if v[(i * self.size + j) as usize] < 10 {
					print!("{}  ", v[(i * self.size + j) as usize]);
				} else {
					print!("{} ", v[(i * self.size + j) as usize]);
				}
			}
			println!();
		}
	}

	pub fn calculate_heuristic(&self, state: &[u32]) -> u32 {
		// TODO: add other heuristics functions

		self.manhatan_distance(state).to_bits()
	}

	fn sovable(state: &[u32], size: u32, blank: Vec2) -> bool {
		let b = Puzzle::get_coordinates(0, size);
		let blank_dist =
			((b.x as f32 - blank.x as f32).abs() + (b.y as f32 - blank.y as f32).abs()) as u32;
		let mut swap = 0u32;
		let mut res = state.to_owned();
		let mut sorted = false;
		while !sorted {
			sorted = true;
			for i in 0..size {
				for j in 0..size {
					let v = Puzzle::get_coordinates(res[(i * size + j) as usize], size);
					if v.y != i || v.x != j {
						swap += 1;
						res.swap((v.y * size + v.x) as usize, (i * size + j) as usize);
						sorted = false;
					}
				}
			}
		}
		let swap_mod = if swap % 2 == 2 { 0 } else { swap % 2 };

		let blank_mod = if blank_dist % 2 == 2 {
			0
		} else {
			blank_dist % 2
		};

		swap_mod == blank_mod
	}

	fn manhatan_distance(&self, state: &[u32]) -> f32 {
		let mut distance = 0.0;
		for i in 0..self.size {
			for j in 0..self.size {
				if state[(i * self.size + j) as usize] != 0 {
					let coord = self
						.coords
						.get(&state[(i * self.size + j) as usize])
						.unwrap();
					let x = coord.x as f32;
					let y = coord.y as f32;
					distance += (x - j as f32).abs() + (y - i as f32).abs();
				}
			}
		}
		distance
	}

	fn _euclidian_distance(&self, state: &[u32]) -> f32 {
		let mut distance = 0.0;
		for i in 0..self.size {
			for j in 0..self.size {
				if state[(i * self.size + j) as usize] != 0 {
					let coord = self
						.coords
						.get(&state[(i * self.size + j) as usize])
						.unwrap();
					let x = coord.x as f32;
					let y = coord.y as f32;
					distance += (x - j as f32).powi(2) + (y - i as f32).powi(2);
				}
			}
		}
		distance.sqrt()
	}

	// fn misplaced_tiles(&self, state: &[u32]) -> f32 {}

	fn init_coordinates(size: u32) -> HashMap<u32, Vec2> {
		let mut coords = HashMap::new();
		let mut count = 0;
		for _ in 0..size {
			for _ in 0..size {
				coords.insert(count, Puzzle::get_coordinates(count, size));
				count += 1;
			}
		}
		coords
	}

	fn generate_random(state: &mut Vec<u32>, size: u32, blank: &mut Vec2) {
		loop {
			(0..(size * size)).into_iter().for_each(|n| {
				state.push(n);
			});
			state.shuffle(&mut rand::thread_rng());
			for (i, x) in state.iter().enumerate() {
				if *x == 0 {
					blank.x = i as u32 % size;
					blank.y = i as u32 / size;
					break;
				}
			}
			if Puzzle::sovable(state, size, *blank) {
				return;
			} else {
				state.clear();
			}
		}
	}

	fn get_coordinates(mut value: u32, size: u32) -> Vec2 {
		if value == 0 {
			value = size * size;
		}
		let mut r = 0;
		let mut span = size;
		while value > span {
			value -= span;
			r += 1;
			span -= r % 2;
		}
		let d = r / 4;
		let m = r % 4;
		let c = size - 1 - d;

		match m {
			0 => Vec2 {
				x: d + value - 1,
				y: d,
			},
			1 => Vec2 { x: c, y: d + value },
			2 => Vec2 { x: c - value, y: c },
			3 => Vec2 { x: d, y: c - value },
			_ => Vec2 { x: 0, y: 0 },
		}
	}
}
