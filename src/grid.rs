use core::panic;
use std::cmp::max;

use colored::Colorize;
use pbr::ProgressBar;

use crate::{node::Node, coords::Coords, graph::Graph};

static TREE_DEPTH: i32 = 17;


#[derive(Clone)]
pub enum NodeTree {
    Leaf {
        center: Coords,
        size: f64,
        nodes: Option<Vec<usize>>
    },
    Node {
        center: Coords,
        next: Box<[NodeTree; 4]>,
    }
}

impl NodeTree {
    /// Subdivide a quadrat into for smaller quadrats
    ///
    /// Bremen
    /// 53° 5′ N , 8° 48′ O
    /// 
    /// München
    /// 48° 8‘ 13.92 N 11° 34‘ 31.8 E
    ///  
    pub fn subdivide(&mut self, graph_nodes: &Vec<Node>) -> Self {
        match self {
            Self::Leaf {center, size, nodes} => {
                    let half_size = *size / 2.0;
                    let quarter_size = half_size / 2.0;
                    let coords = [
                        Coords::deg(center.lat + quarter_size, center.lon - quarter_size), //nw
                        Coords::deg(center.lat + quarter_size, center.lon + quarter_size), //ne
                        Coords::deg(center.lat - quarter_size, center.lon - quarter_size), //sw
                        Coords::deg(center.lat - quarter_size, center.lon + quarter_size), //se
                    ];

                    let mut split_nodes: [Option<Vec<usize>>; 4] = [None, None, None, None]; 

                    if let Some(node_indices) = nodes {
                        for index in node_indices.to_owned() {
                            let position = Self::relative_position(graph_nodes[index].coords, *center);

                            //TODO: Not optimal to check always for Some / None
                            match &mut split_nodes[position] {
                                Some(nodes) => {
                                    nodes.push(index);
                                },
                                None => {
                                    *nodes = Some(vec![index]);
                                },
                            }
                        }
                    }

                    let mut iter = split_nodes.into_iter();

                    let next: [NodeTree; 4] = [
                        Self::Leaf { center: coords[0], size: half_size, nodes: iter.next().unwrap() },
                        Self::Leaf { center: coords[1], size: half_size, nodes: iter.next().unwrap() },
                        Self::Leaf { center: coords[2], size: half_size, nodes: iter.next().unwrap() },
                        Self::Leaf { center: coords[3], size: half_size, nodes: iter.next().unwrap() },
                    ];
                    
                    Self::Node {
                        next: Box::new(next),
                        center: *center
                    }
            }
            _ => {
                panic!("Invalid!");
            }
        }
    }


    pub fn root_leaf() -> Self {
        Self::Leaf {center: Coords { lat: 0.0, lon: 0.0 }, size: 180.0, nodes: None }
    }

    pub fn relative_position(coord1: Coords, coord2: Coords) -> usize {
        let lat = coord1.lat < coord2.lat;
        let lon = coord1.lon > coord2.lon;

        return lat as usize * 2 + lon as usize;
    }

    pub fn nearest_node(&self, graph_nodes: &Vec<Node>, coords: Coords) -> usize {
        let mut prev_next: Option<&Box<[NodeTree; 4]>> = None;
        let mut node = self;
        let mut last_pos = usize::MAX;
        let mut i = 0;
        let mut tried: [bool; 4] = [false, false, false, false];

        loop {        
            if let NodeTree::Node { center, next } = node {
                last_pos = Self::relative_position(coords, *center);
                node = &next[last_pos];
                i += 1;    
                prev_next = Some(next);
                tried = [false, false, false, false];
            } else {
                if i == TREE_DEPTH {
                    break;
                } else {
                    let mut lowest_dist = f64::MAX;
                    let mut lowest_index: usize = 0;

                    let prev_next = prev_next.expect("Nothing was inserted into the tree...");

                    for i in 0..4 {
                        if i != last_pos && !tried[i] {
                            if let NodeTree::Node { center, next: _ } = &prev_next[i] {
                                let dist = center.distance_to(&coords);

                                if dist < lowest_dist {
                                    lowest_dist = dist;
                                    lowest_index = i;
                                }
                            }
                        }
                    }

                    if lowest_dist == f64::MAX {
                        panic!("Tree is inconsistent...");
                    }
                    
                    tried[lowest_index] = true;
                    node = &prev_next[lowest_index];
                }
            }
        }

        match node {
            NodeTree::Leaf {center: _, size: _, nodes } => {

                if let Some(nodes) = nodes {
                    return Graph::nearest_node_naive_indices(graph_nodes, nodes, coords);
                }
            },
            _ => {
                // Can not happen in theory
                panic!("Error finding nearest node (Not a leaf)");
            }
        }

        for i in 0..4 {
            if i == last_pos {
                continue;
            }
            
            node = &prev_next.unwrap()[i];

            match node {
                NodeTree::Leaf {center: _, size: _, nodes } => {

                    if let Some(nodes) = nodes {
                        return Graph::nearest_node_naive_indices(graph_nodes, nodes, coords);
                    }
                },
                _ => {
                    // Can not happen in theory
                    panic!("Error finding nearest node (Not a leaf)");
                }
            }
        }

        panic!("Error finding nearest node (No nodes found in leaf)");
    }

    pub fn build(graph_nodes: &Vec<Node>) -> Self {
        let mut element_count = 0;
        let mut tree = Self::root_leaf();
        tree.subdivide(graph_nodes);     

        let mut leaf : &mut NodeTree = &mut tree;
        let mut max_depth = 0;
        let mut max_leaf_size: usize = 0;

        let mut subdivisions = 0;

        let border = (graph_nodes.len() / 100) as usize;
        let mut pb = ProgressBar::new(100);
        pb.show_speed = false;

        for (i, node) in graph_nodes.iter().enumerate() {
            if i % border == 0 {
                pb.inc();
            }

            let mut iy = 0;

            loop {
                max_depth = max(max_depth, iy);
                match leaf {
                    NodeTree::Node { next, center } => {
                        leaf = &mut next[Self::relative_position(node.coords, *center)];
                        iy += 1;      
                        //print!("{} ({:.2}, {:.2} ; {:.2}, {:.2}) ", Self::relative_position(node.coords, *center), node.coords.lat, node.coords.lon, center.lat, center.lon);
                    },
                    NodeTree::Leaf { center: _, size: _, nodes }  => {
                        if iy == TREE_DEPTH {
                            match nodes {
                                Some(vector) => {
                                    vector.push(i);
                                    if vector.len() > max_leaf_size {
                                        max_leaf_size = vector.len();
                                    }
                                },
                                None => {
                                    *nodes = Some(vec![i]);
                                },
                            }
                        
                            element_count += 1;
                            break;
                        }

                        *leaf = leaf.subdivide(graph_nodes);
                        subdivisions += 1;
                    }
                }        
            }

            leaf = &mut tree;
        }

        println!("{}", format!("(d: {}, ec: {}, ml: {}, s: {})", max_depth, element_count, max_leaf_size, subdivisions).magenta());

        /*for i in 0..100 {
            leaf.subdivide();


            match leaf {
                NodeTree::Node { next } => {
                    leaf = &mut next[0];
                },
                _  => {
                    panic!("Should be a node")
                }
            }

        }*/

        tree
    }
}

#[cfg(test)]
mod tests {
    use crate::{node::Node, coords::Coords};
    use super::NodeTree;

    #[test]
    fn position() {
        let node = Node::new(Coords::deg(53.5, 8.48), 0);

        assert_eq!(NodeTree::relative_position(node.coords, Coords::deg(48.81392, 11.34318)), 3);

    }
}    