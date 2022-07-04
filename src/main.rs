use std::env;

use algo::Algo;
use getopts::Options;
use heuristic::Heuristic;
use solver::Puzzle;

mod algo;
mod heuristic;
mod node;
mod solver;

fn print_usage(program: &str, opts: Options) {
	let brief = format!("Usage: ./{} [options]", program);
	print! {"{}", opts.usage(&brief)};
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut file_path = None;
	let mut size = 3;
	let mut silent = false;
	let mut random = false;
	let mut algo = Box::new(algo::Astar::solve) as Algo;
	let mut heuristic = Box::new(heuristic::Manhattan::heuristic) as Heuristic;

	let mut opts = Options::new();
	opts.optopt("f", "file", "Path to a puzzle file", "<PATH>");
	opts.optflag("r", "random", "Generate a random puzzle");
	opts.optflag("m", "manhattan", "Select the Manhattan heuristic");
	opts.optflag("e", "euclidian", "Select the Euclidian heuristic");
	opts.optflag("H", "Hamming", "Select the Hamming heuristic");
	opts.optopt(
		"s",
		"size",
		"Size of the randomly generated puzzle, default: 3",
		"<SIZE>",
	);
	opts.optflag("i", "ida", "Use IDA* algorithm");
	opts.optflag("S", "silent", "Don't print the solution");
	opts.optflag("h", "help", "Print help information");
	let matches = match opts.parse(&args[1..]) {
		Ok(m) => m,
		Err(f) => {
			eprintln!("{}", f);
			print_usage(&program, opts);
			return;
		}
	};
	if matches.opt_present("S") {
		silent = true;
	}
	if matches.opt_present("e") {
		heuristic = Box::new(heuristic::Euclidian::heuristic) as Heuristic;
	}
	if matches.opt_present("H") {
		heuristic = Box::new(heuristic::Hamming::heuristic) as Heuristic;
	}
	if matches.opt_present("i") {
		algo = Box::new(algo::IDAstar::solve) as Algo;
	}
	if matches.opt_present("s") {
		size = matches.opt_str("s").unwrap().parse::<u32>().unwrap();
	}
	if matches.opt_present("r") {
		random = true;
	}
	if matches.opt_present("f") {
		file_path = Some(matches.opt_str("f").unwrap());
	}

	if matches.opt_present("h") {
		print_usage(&program, opts);
		return;
	}
	if random && file_path.is_some() {
		println!("Cannot use both random and file, using the file by default");
		random = false;
	} else if !random && file_path.is_none() {
		eprintln!("Must use either random or file, using random by default");
		random = true;
	}

	let mut puzzle = Puzzle::new(file_path, random, size, silent, algo, heuristic);
	puzzle.solve();
}
