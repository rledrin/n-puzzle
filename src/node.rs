use std::{
	hash::{Hash, Hasher},
	rc::Rc,
};

use crate::solver::Vec2;

#[derive(Debug, Clone)]
pub struct Node {
	pub state: Vec<u32>,
	pub father: Option<Rc<Node>>,

	pub blank: Vec2,

	pub g: u32, // cost from start to this node
	pub h: u32, // cost from this node to goal
	pub f: u32, // g + h
}

impl Hash for Node {
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		self.state.hash(hasher);
	}
}

impl Eq for Node {}
impl PartialEq for Node {
	fn eq(&self, other: &Self) -> bool {
		self.state.iter().eq(other.state.iter())
	}
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		let sf = f32::from_bits(self.f);
		let of = f32::from_bits(other.f);
		of.partial_cmp(&sf).unwrap()
	}
}

impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}
