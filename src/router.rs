use std::cmp::Ordering;
use std::collections::BinaryHeap;
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

pub fn solve_file(graph: Arc<Graph>, thread_count: u32, path: String) {
    let file = File::open(path).expect("Couldn't open query file. Please check if the path is correct!");
    let reader = BufReader::new(file);
    let mut handles = Vec::new();
    let lines = reader.lines().into_iter().map(|x| x.unwrap()).collect::<Vec<String>>();
    let line_count = lines.len();
    // Fill the distance array with values
    let distances = (0..line_count).map(|_| -1).collect::<Vec<i64>>();
    let distances = Arc::new(Mutex::new(distances));
    let lines_iter = Arc::new(Mutex::new((lines.into_iter(), 0)));
    println!("{}", format!("Calculating distances multi-threaded with {} threads...", thread_count).yellow());

    (0..thread_count).for_each(|_| {
        let graph = graph.clone();
        let distances = distances.clone();
        let lines_iter = lines_iter.clone();    
        let handle = thread::spawn(move || {
            loop {
                let mut guard = lines_iter.lock().unwrap();
                let index = guard.1;
                guard.1 += 1;
                if let Some(line) = guard.0.next() {
                    // Unlock the mutex
                    drop(guard);
                    let mut split = line.split(char::is_whitespace);
                    let start = split.next().unwrap().parse::<usize>().unwrap();
                    let goal = split.next().unwrap().parse::<usize>().unwrap();
                    let distance: i64 = shortest_path(&graph, start, goal);
                    distances.lock().unwrap()[index] = distance;
                } else {
                    break;
                }
            }
        });

        handles.push(handle);
    });
    
    let mut pb = ProgressBar::new(line_count as u64);
    pb.show_speed = false;
    let mut progress = 0;
    while progress < line_count {
        progress = lines_iter.as_ref().lock().unwrap().1;
        
        // Needed because the thread(s) increase the progress counter before they start working on it
        if progress < thread_count as usize {
            progress = 0;
        } else {
            progress -= thread_count as usize;
        }

        pb.set(progress as u64);
        // Important to not use the lines_iter mutex too often
        thread::sleep(Duration::from_millis(100));
    }

    // Joining remaining thread(s), should not be necessary
    handles.into_iter().for_each(|x| {
        x.join().unwrap()
    });
    
    // Write out the distances
    println!("\n\n");
    distances.lock().unwrap().iter().for_each(|dist| {
        println!("{}", *dist);
    });
}

pub fn shortest_paths(graph: &Graph, start: usize) -> Vec<EdgeCost> {
    dijkstra(graph, start, usize::MAX).1
}

pub fn shortest_path(graph: &Graph, start: usize, goal: usize) -> EdgeCost {
    dijkstra(graph, start, goal).0
}

fn dijkstra(graph: &Graph, start: usize, goal: usize) -> (EdgeCost, Vec<EdgeCost>) {
    let mut heap = BinaryHeap::with_capacity(graph.edge_count());
    let mut dist = (0..graph.node_count()).map(|_| EdgeCost::MAX).collect::<Vec<EdgeCost>>();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
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
            }
        }
    }

    (-1, dist)
}