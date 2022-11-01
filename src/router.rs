use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
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

pub fn solve_file(graph: &Graph, path: String) -> Result<(), Box<dyn Error>> {
    let file = File::open(path).expect("Couldn't open query file. Please check if the path is correct!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let mut split = line.split(char::is_whitespace);
        let distance = shortest_path(graph, split.next().unwrap().parse::<usize>()?, split.next().unwrap().parse::<usize>()?);
        println!("{}", distance);
    }

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
            //return Some(path);
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