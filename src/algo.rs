use std::{
	cmp::Ordering,
	collections::{BinaryHeap, HashSet},
	ops::Deref,
	rc::Rc,
};

use crate::{node::Node, solver::Puzzle};

pub type Algo = Box<dyn Fn(&mut Puzzle)>;

pub struct Astar {}

pub struct IDAstar {}

enum SearchState {
	Found,
	N(f32),
}

impl Astar {
	pub fn solve(puzzle: &mut Puzzle) {
		let mut open = BinaryHeap::new();
		let mut close = HashSet::new();
		let mut node = Rc::new(Node {
			father: None,
			state: puzzle.initial_state.clone(),
			blank: puzzle.initial_blank,
			g: 0.0f32.to_bits(),
			h: puzzle.calculate_heuristic(&puzzle.initial_state),
			f: 0.0f32.to_bits(),
		});

		open.push(node.clone());

		puzzle.start_time = std::time::Instant::now();
		loop {
			if open.is_empty() {
				println!("No solution found 👎");
				break;
			}
			node = open.pop().unwrap();
			puzzle.current_open -= 1;

			if is_goal(node.clone()) {
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
						puzzle.print_state(&n.state);
						println!();
					}
				}
				return;
			}
			Astar::expand(puzzle, node, &mut open, &mut close);
		}
	}

	fn expand(
		puzzle: &mut Puzzle,
		node: Rc<Node>,
		open: &mut BinaryHeap<Rc<Node>>,
		close: &mut HashSet<Rc<Node>>,
	) {
		let new_nodes = neighbors(puzzle, node.clone());
		Astar::add_to_open(puzzle, new_nodes, open, close);
		close.insert(node);
	}

	fn add_to_open(
		puzzle: &mut Puzzle,
		new_nodes: Vec<Rc<Node>>,
		open: &mut BinaryHeap<Rc<Node>>,
		close: &mut HashSet<Rc<Node>>,
	) {
		for node in new_nodes {
			if close.get(&node).is_none() {
				open.push(node.clone());
				puzzle.total_open += 1;
				puzzle.current_open += 1;
			}
		}
	}
}

impl IDAstar {
	pub fn solve(puzzle: &mut Puzzle) {
		let node = Rc::new(Node {
			father: None,
			state: puzzle.initial_state.clone(),
			blank: puzzle.initial_blank,
			g: 0.0f32.to_bits(),
			h: puzzle.calculate_heuristic(&puzzle.initial_state),
			f: 0.0f32.to_bits(),
		});
		let mut bound = f32::from_bits(node.h);
		let mut path = vec![node];
		loop {
			let t = IDAstar::search(puzzle, &mut path, 0.0, bound);
			match t {
				SearchState::Found => {
					println!("Solved! 👍");
					println!(
						"Solution found in {} steps",
						f32::from_bits(path.last().unwrap().g)
					);
					println!("{} ms", puzzle.start_time.elapsed().as_millis());
					println!("Complexity in time(total open): {}", puzzle.total_open);
					println!(
						"Complexity in space(open at the same time) {}",
						puzzle.current_open
					);
					if !puzzle.silent {
						println!("Solution: ");

						for n in path.iter() {
							puzzle.print_state(&n.state);
							println!();
						}
					}
					return;
				}
				SearchState::N(n) => {
					if n.partial_cmp(&f32::MAX).unwrap() == Ordering::Equal {
						println!("No solution found 👎");
						return;
					} else {
						bound = n;
					}
				}
			}
		}
	}

	fn search(puzzle: &mut Puzzle, path: &mut Vec<Rc<Node>>, g: f32, bound: f32) -> SearchState {
		let node = path.last().unwrap().clone();
		let f = g + f32::from_bits(node.h);
		if f > bound {
			return SearchState::N(f);
		}
		if is_goal(node.clone()) {
			return SearchState::Found;
		}
		let mut min = f32::MAX;
		for succ in neighbors(puzzle, node) {
			if !path.contains(&succ) {
				path.push(succ.clone());
				puzzle.total_open += 1;
				puzzle.current_open += 1;
				let t = IDAstar::search(puzzle, path, g + 1.0f32, bound);
				match t {
					SearchState::Found => return SearchState::Found,
					SearchState::N(n) => {
						if n < min {
							min = n;
						}
					}
				}
				path.pop();
				puzzle.current_open -= 1;
			}
		}
		SearchState::N(min)
	}
}

fn neighbors(puzzle: &mut Puzzle, node: Rc<Node>) -> Vec<Rc<Node>> {
	let c = vec![(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];

	let mut new_nodes = Vec::new();
	for (x, y) in c {
		if (node.blank.x as i32) + x >= 0
			&& (node.blank.x as i32) + x < puzzle.size as i32
			&& (node.blank.y as i32) + y >= 0
			&& (node.blank.y as i32) + y < puzzle.size as i32
		{
			let mut n = node.deref().clone();
			n.blank.x = ((node.blank.x as i32) + x) as u32;
			n.blank.y = ((node.blank.y as i32) + y) as u32;
			n.state.swap(
				(n.blank.y * puzzle.size + n.blank.x) as usize,
				(node.blank.y * puzzle.size + node.blank.x) as usize,
			);
			n.father = Some(node.clone());

			n.g = (f32::from_bits(node.g) + 1.0f32).to_bits();
			n.h = puzzle.calculate_heuristic(&n.state);
			n.f = (f32::from_bits(n.g) + f32::from_bits(n.h)).to_bits();

			new_nodes.push(Rc::new(n));
		}
	}
	new_nodes
}
fn is_goal(node: Rc<Node>) -> bool {
	f32::from_bits(node.h).partial_cmp(&0.0).unwrap() == std::cmp::Ordering::Equal
}
