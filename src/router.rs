use std::cmp::{Ordering, min};
use std::collections::BinaryHeap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use owo_colors::OwoColorize;
use pbr::ProgressBar;

use crate::edge::{EdgeCost};
use crate::Graph;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: EdgeCost,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve_file(graph: Arc<Graph>, path: String) -> Result<(), Box<dyn Error>> {
    let file = File::open(path).expect("Couldn't open query file. Please check if the path is correct!");
    let reader = BufReader::new(file);
    let mut handles = Vec::new();
    let lines = reader.lines().into_iter().map(|x| x.unwrap()).collect::<Vec<String>>();
    let line_count = lines.len();
    let cpus = min(4, num_cpus::get());

    println!("{}", format!("Calculating distances multi-threaded with {} threads.", cpus).yellow());

    let distances = (0..line_count).map(|_| -1).collect::<Vec<i64>>();
    let distances = Arc::new(Mutex::new(distances));
    let lines_iter = Arc::new(Mutex::new((lines.into_iter(), 0)));

    for _ in 0..cpus {
        let graph = graph.clone();
        let distances = distances.clone();
        let lines_iter = lines_iter.clone();
        let handle = thread::spawn(move || {
            loop {
                let mut guard = lines_iter.lock().unwrap();
                let index = guard.1;
                guard.1 += 1;
                if let Some(line) = guard.0.next() {
                    drop(guard);
                    let mut split = line.split(char::is_whitespace);
                    let distance: i64 = shortest_path(&graph, split.next().unwrap().parse::<usize>().unwrap(), split.next().unwrap().parse::<usize>().unwrap());
                    distances.lock().unwrap()[index] = distance;
                } else {
                    break;
                }
            }
        });

        handles.push(handle);
    }
    let mut pb = ProgressBar::new(line_count as u64);
    pb.show_speed = false;


    let mut progress = 0;
    while progress < line_count {
        progress = lines_iter.as_ref().lock().unwrap().1;

        pb.set(progress as u64);
        thread::sleep(Duration::from_millis(100));
    }

    handles.into_iter().for_each(|x| {
        x.join().unwrap()
    });

    println!("\n\n");

    distances.lock().unwrap().iter().for_each(|dist| {
        println!("{}", *dist);
    });

    Ok(())
}

pub fn shortest_paths(graph: &Graph, start: usize) -> Vec<EdgeCost> {
    dijkstra(graph, start, usize::MAX).1
}

pub fn shortest_path(graph: &Graph, start: usize, goal: usize) -> EdgeCost {
    dijkstra(graph, start, goal).0
}

fn dijkstra(graph: &Graph, start: usize, goal: usize,) -> (EdgeCost, Vec<EdgeCost>) {
    let mut heap = BinaryHeap::with_capacity(graph.edge_count());
    let mut dist = (0..graph.node_count()).map(|_| EdgeCost::MAX).collect::<Vec<EdgeCost>>();
    let mut prev = (0..graph.node_count()).map(|_| usize::MAX).collect::<Vec<usize>>();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            let mut path = Vec::new();
            let mut previous = position;
            while previous != start {
                path.push(previous);
                previous = prev[previous];
            }

            return (cost, dist);
        }

        if cost > dist[position] {
            continue;
        }

        for edge in graph.outgoing_edges(position) {
            let edge_cost = edge.cost;
            let next = State { cost: cost + edge_cost, position: edge.trg as usize };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
                prev[next.position] = position;
            }
        }
    }

    (-1, dist)
}