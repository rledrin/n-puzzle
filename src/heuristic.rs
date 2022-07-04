use crate::solver::Puzzle;

pub type Heuristic = Box<dyn Fn(&Puzzle, &[u32]) -> f32>;

pub struct Manhattan {}

pub struct Euclidian {}

pub struct Hamming {}

impl Manhattan {
	pub fn heuristic(puzzle: &Puzzle, state: &[u32]) -> f32 {
		let mut distance = 0.0;
		for i in 0..puzzle.size {
			for j in 0..puzzle.size {
				if state[(i * puzzle.size + j) as usize] != 0 {
					let coord = puzzle
						.coords
						.get(&state[(i * puzzle.size + j) as usize])
						.unwrap();
					let x = coord.x as f32;
					let y = coord.y as f32;
					distance += (x - j as f32).abs() + (y - i as f32).abs();
				}
			}
		}
		distance
	}
}

impl Euclidian {
	pub fn heuristic(puzzle: &Puzzle, state: &[u32]) -> f32 {
		let mut distance = 0.0;
		for i in 0..puzzle.size {
			for j in 0..puzzle.size {
				if state[(i * puzzle.size + j) as usize] != 0 {
					let coord = puzzle
						.coords
						.get(&state[(i * puzzle.size + j) as usize])
						.unwrap();
					let x = coord.x as f32;
					let y = coord.y as f32;
					distance += (x - j as f32).powi(2) + (y - i as f32).powi(2);
				}
			}
		}
		distance.sqrt()
	}
}

impl Hamming {
	pub fn heuristic(puzzle: &Puzzle, state: &[u32]) -> f32 {
		let mut distance = 0.0;
		for i in 0..puzzle.size {
			for j in 0..puzzle.size {
				if state[(i * puzzle.size + j) as usize] != 0 {
					let coord = puzzle
						.coords
						.get(&state[(i * puzzle.size + j) as usize])
						.unwrap();
					let x = coord.x as f32;
					let y = coord.y as f32;
					if x != j as f32 || y != i as f32 {
						distance += 1.0;
					}
				}
			}
		}
		distance
	}
}
