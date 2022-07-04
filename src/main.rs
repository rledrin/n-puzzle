use std::env;

use getopts::Options;
use solver::Puzzle;

mod algo;
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
	let mut algo;

	let mut opts = Options::new();
	opts.optopt("f", "file", "Path to a puzzle file", "<PATH>");
	opts.optflag("r", "random", "Generate a random puzzle");
	opts.optopt(
		"s",
		"size",
		"Size of the randomly generated puzzle, default: 3",
		"<SIZE>",
	);
	opts.optflag("ida", "", "Use IDA* algorithm");
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
	if matches.opt_present("ida") {
		algo = algo::Algo::idastar_solve;
	} else {
		algo = algo::Algo::astar_solve;
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
	// let _ = if !matches.free.is_empty() {
	// 	matches.free[0].clone()
	// } else {
	// 	print_usage(&program, opts);
	// 	return;
	// };

	let mut puzzle = Puzzle::new(file_path, random, size, silent, algo);
	puzzle.solve();
}
