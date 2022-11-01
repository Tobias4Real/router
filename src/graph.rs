use std::fs::File;
use std::io::{BufRead, BufReader};
use pbr::ProgressBar;
use crate::Coords;
use crate::edge::{Edge, EdgeCost};
use crate::node::{Node, NodeIndex};


pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

const NODE_INDEX_MAX_USIZE: usize = NodeIndex::MAX as usize;

impl Graph {
    fn new(node_count: usize, edge_count: usize) -> Self {
        Self {
            nodes: Vec::<Node>::with_capacity(node_count),
            edges: Vec::<Edge>::with_capacity(edge_count)
        }
    }

    pub fn is_empty(&self) -> bool { self.nodes.is_empty() }
    pub fn node(&self, index: usize) -> Option<&Node> { self.nodes.get(index) }
    pub fn edge(&self, index: usize) -> Option<&Edge> { self.edges.get(index) }
    pub fn edges(&self) -> &Vec<Edge> { &self.edges }
    pub fn nodes(&self) -> &Vec<Node> { &self.nodes }
    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn edge_count(&self) -> usize { self.edges.len() }

    pub fn outgoing_edges(&self, index: usize) -> &[Edge] {
        //Calculate upper the limit of the outgoing edges
        let end = if index == self.node_count() - 1 {
            self.edge_count()
        } else {
            self.node(index + 1).unwrap().offset as usize
        };

        let start = self.node(index).unwrap().offset as usize;

        if start == NODE_INDEX_MAX_USIZE {
            return &[];
        }

        &self.edges[start..end]
    }

    pub fn nearest_node_naive_indices(nodes: &Vec<Node>, indices: &Vec<usize>, coords: Coords) -> usize {
        if nodes.is_empty() {
            panic!("The graph loaded is empty!")
        }

        let mut lowest_dist: f64 = f64::MAX;
        let mut lowest_index = usize::MAX;

        for i in indices {
            let i = *i;
            if lowest_index == usize::MAX {
                lowest_index = i;
            } else {
                let dist = coords.distance_to(&nodes[i].coords);
                if dist < lowest_dist {
                    lowest_index = i;
                    lowest_dist = dist;
                }
            }
        }
        lowest_index
    }

    pub fn nearest_node_naive(nodes: &Vec<Node>, coords: Coords) -> usize {
        if nodes.is_empty() {
            panic!("The graph loaded is empty!")
        }

        let mut lowest_dist: f64 = f64::MAX;
        let mut lowest_index = usize::MAX;

        for (i, node) in nodes.iter().enumerate() {
            if lowest_index == usize::MAX {
                lowest_index = i;
            } else {
                let dist = coords.distance_to(&node.coords);
                if dist < lowest_dist {
                    lowest_index = i;
                    lowest_dist = dist;
                }
            }
        }
        lowest_index
    }

    pub fn from_file(path: String) -> Graph {
        let file = File::open(path).expect("Couldn't open the graph file. Please check if the path is valid!");
        let mut reader = BufReader::new(file);

        let mut node_count: usize = 0;
        let mut edge_count: usize = 0;

        let mut line_buf = String::with_capacity(128);

        let mut is_node_count = true;
        while reader.read_line(&mut line_buf).unwrap() != 0 {
            let buf: &str = &line_buf[0..line_buf.len()-1];
            if !buf.is_empty() && buf.as_bytes().get(0) != Some(&b'#') {
                if is_node_count {
                    node_count = buf.parse::<usize>().unwrap();
                    is_node_count = false;
                } else {
                    edge_count = buf.parse::<usize>().unwrap();
                    break;
                }
            } else {
            }
            line_buf.clear();
        }

        if edge_count <= 0 || node_count <= 0 {
            panic!("Graph file is incorrect.")
        }

        let mut graph = Self::new(node_count, edge_count);

        let mut i: usize = 0;
        let border = ((edge_count + node_count) / 100) as usize;
        let mut pb = ProgressBar::new(100);
        pb.show_speed = false;

        let mut last_edge_src = 0;
        let mut last_edge_cnt: NodeIndex = 0;

        line_buf.clear();
        while reader.read_line(&mut line_buf).unwrap() != 0 {
            i += 1;
            if i % border == 0 {
                pb.inc();
            }

            //Remove the newline
            let buf: &str = &line_buf[0..line_buf.len() - 1];

            if graph.nodes.len() < node_count {
                let mut node = Node::default();
                let mut it = buf.split(char::is_whitespace);
                it.next();
                //node.offset = it.next().unwrap().parse::<NodeIndex>().unwrap();
                it.next();


                let mut coords = Coords::default();
                coords.set_lat_deg(it.next().unwrap().parse::<f64>().unwrap());
                coords.set_lon_deg(it.next().unwrap().parse::<f64>().unwrap());
                node.coords = coords;
                graph.nodes.push(node);
            } else {
                let mut edge = Edge::default();
                let mut it = buf.split(char::is_whitespace);
                edge.src = it.next().unwrap().parse::<NodeIndex>().unwrap();

                if last_edge_src != edge.src {
                    graph.nodes[last_edge_src as usize].offset = last_edge_cnt;
                    last_edge_cnt = graph.edges.len() as NodeIndex;

                    //Set offsets for nodes which have no outgoing edges
                    (last_edge_src+1..edge.src).map(|i| i as usize).for_each(|i| graph.nodes[i].offset = last_edge_cnt);

                    last_edge_src = edge.src;
                }

                edge.trg = it.next().unwrap().parse::<NodeIndex>().unwrap();
                edge.cost = it.next().unwrap().parse::<EdgeCost>().unwrap();

                graph.edges.push(edge);
            }
            line_buf.clear();
        }

        graph.nodes[last_edge_src as usize].offset = last_edge_cnt;

        #[cfg(debug_assertions)]
        graph.nodes.iter().for_each(|node| assert_ne!(node.offset, NodeIndex::MAX));

        println!("\nProcessed {} lines, {} / {} edges, {} / {} nodes", i, graph.edges.len(), edge_count, graph.nodes.len(), node_count);

        graph
    }
}