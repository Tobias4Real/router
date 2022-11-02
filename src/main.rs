pub mod args;
pub mod coords;
pub mod edge;
pub mod graph;
pub mod grid;
pub mod node;
pub mod router;

use owo_colors::OwoColorize;
use std::process::exit;
use std::sync::Arc;
use std::time::Instant;
use std::{env, io};

use crate::coords::Coords;
use crate::graph::Graph;
use crate::grid::NodeTree;
use crate::node::NodeIndex;
use args::Args;

fn main() {
	println!(
		"{}",
		"Router. The ultimate route finder. (C) 2022 Thorben Bernhardt & Tobias Schaberl.".cyan()
	);
	#[cfg(debug_assertions)]
	println!("{}", "WARNING! The debug build is very slow. Using the release build is highly recommended for large maps.".red());

	env::set_var("RUST_LOG", "info");
	let args = Args::parse()
		.map_err(|err| {
			println!("{}", err);
			exit(-1);
		})
		.unwrap();

	if let Some(file) = args.graph_file {
		let now = Instant::now();
		println!("{}", "Loading graph...".yellow());
		let graph = Graph::from_file(file);
		println!(
			"Loading the graph took {}{}.",
			now.elapsed().as_millis(),
			"ms".green()
		);

		let now = Instant::now();
		println!("{}", "Building nearest data structure... ".yellow());
		let tree = NodeTree::build(graph.nodes());
		println!(
			"Building the data structure took {}{}",
			now.elapsed().as_millis(),
			"ms".green()
		);

		if let (Some(lat), Some(lon)) = (args.lat, args.lon) {
			let coordinates = Coords::deg(lat, lon);

			if (args.flags & args::flag::SHOW_NAIVE_NODE) != 0 {
				print!("Finding nearest node (naïve)... ");
				let now = Instant::now();
				let nearest = Graph::nearest_node_naive(graph.nodes(), coordinates);
				println!("   {}{}", now.elapsed().as_millis(), "ms".green());
				println!(
					"Naïve nearest node to {}, {}:       [{}] {}.",
					lat,
					lon,
					nearest,
					graph.node(nearest).unwrap()
				);
			}

			print!("Finding nearest node (QuadTree)... ");
			let now = Instant::now();
			let nearest = tree.nearest_node(graph.nodes(), coordinates);
			println!("{}{}", now.elapsed().as_micros(), "µs".red());
			println!(
				"Nearest node to {}, {}:             [{}] {}.",
				lat,
				lon,
				nearest,
				graph.node(nearest).unwrap()
			);
		}

		let arc = Arc::new(graph);

		if let Some(query) = args.query_file {
			let now = Instant::now();
			router::solve_file(arc.clone(), query).unwrap();
			println!("\n");
			println!(
				"Calculating the distances took {}{}",
				now.elapsed().as_millis(),
				"ms".green()
			);
		}

		if let Some(source) = args.source_node {
			let now = Instant::now();
			println!("Running one-to-all dijkstra... ");
			let paths = router::shortest_paths(arc.as_ref(), source as usize);
			println!(
				"One-to-all dijkstra took {}{}",
				now.elapsed().as_millis(),
				"ms".green()
			);

			let target = match args.target_node {
				Some(target) => target as usize,
				None => {
					println!("Enter target node:");
					let mut line = String::new();
					io::stdin().read_line(&mut line).unwrap();
					line[0..line.len() - 1]
						.parse::<usize>()
						.expect("Target node id must be a positive number.")
				}
			};

			let mut dist = *paths.get(target).expect("Invalid node id");

			if dist == NodeIndex::MAX {
				dist = -1;
			}

			println!("Distance to target[{}]: {}", target, dist);
		}
	} else {
		println!("Nothing to do!");
	}
}
