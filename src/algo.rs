use std::rc::Rc;

use crate::solver::{Node, Puzzle};

pub trait Algo {
	fn solve(puzzle: &mut Puzzle);
}

pub struct Astar {}
pub struct IDAstar {}

impl Algo for Astar {
	fn solve(puzzle: &mut Puzzle) {
		let mut node = Rc::new(Node {
			father: None,
			state: puzzle.initial_state.clone(),
			blank: puzzle.initial_blank,
			g: 0.0f32.to_bits(),
			h: puzzle.calculate_heuristic(&puzzle.initial_state),
			f: 0.0f32.to_bits(),
		});

		let print = |v: &Vec<u32>, s: u32| {
			for i in 0..s {
				for j in 0..s {
					if v[(i * s + j) as usize] < 10 {
						print!("{}  ", v[(i * s + j) as usize]);
					} else {
						print!("{} ", v[(i * s + j) as usize]);
					}
				}
				println!();
			}
		};

		println!("Initial heuristic: {}", f32::from_bits(node.h));
		println!("Initial state: ");
		print(&puzzle.initial_state, puzzle.size);
		println!();
		puzzle.open.push(node.clone());

		puzzle.start_time = std::time::Instant::now();
		loop {
			if puzzle.open.is_empty() {
				println!("No solution found 👎");
				break;
			}
			node = puzzle.open.pop().unwrap();
			puzzle.current_open -= 1;

			if puzzle.is_goal(node.clone()) {
				println!("Solved! 👍");
				println!("Solution found in {} steps", f32::from_bits(node.g));
				println!("{} ms", puzzle.start_time.elapsed().as_millis());
				println!("Complexity in time(total open): {}", puzzle.total_open);
				println!(
					"Complexity in space(open at the same time) {}",
					puzzle.current_open
				);
				if !puzzle.silent {
					println!("Solution: ");
					let mut sol = Vec::with_capacity(f32::from_bits(node.g) as usize);
					sol.push(&node);
					let mut n = node.father.as_ref().unwrap();
					for _ in 0..(f32::from_bits(node.g) as usize) {
						sol.push(n);
						if n.father.is_none() {
							break;
						}
						n = n.father.as_ref().unwrap();
					}
					for n in sol.iter().rev() {
						print(&n.state, puzzle.size);
						println!();
					}
				}
				return;
			}
			puzzle.expand(node);
		}
	}
}

impl Algo for IDAstar {
	fn solve(_puzzle: &mut Puzzle) {}
}
