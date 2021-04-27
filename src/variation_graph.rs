#[allow(unused_imports)]
use crate::parser::{
    Alignment,
};
#[allow(unused_imports)]
use crate::partitioner::{
    GreedyPartitioner,
    Partitioner,
    Interval,
    Partition
};
#[allow(unused_imports)]
use handlegraph::{
    handle::{Direction, Edge, Handle, NodeId},
    handlegraph::*,
    hashgraph::{
        HashGraph,
    },
    mutablehandlegraph::*,
    pathhandlegraph::*,
};

use crate::INDEL;

//const INDEL : u8 = '-' as u8;
const FIST_NODE_LABEL : &[u8] = b"first_node";
const LAST_NODE_LABEL : &[u8] = b"last_node";

///Represents a variation graph
#[derive(Debug)]
pub struct VariationGraph {
    pub graph : HashGraph,
    first_node : Handle,
    last_node : Handle,
}

#[derive(Debug)]
pub enum VariationGraphError {
    PathNotFound
}

impl VariationGraph {
    ///Builds a variation graph given an Alignment
    pub fn new(alignment : &Alignment, partition : &Partition) -> VariationGraph {
        let (vg, first_node, last_node) = VariationGraph::build_vg(alignment, partition);
        VariationGraph {graph : vg, first_node : first_node, last_node : last_node}
    }

    ///Prints the path corrisponding to 'path_name'
    pub fn print_path(&self, path_name : &str) -> Result<(), VariationGraphError> {
        match self.graph.path_id.get(path_name.as_bytes()) {
            Some(path) => {
                println!("Path name : {}", path_name);
                self.graph.print_path(path);
                Ok(())
            },
            _ => Err(VariationGraphError::PathNotFound),
        }
    }

    pub fn get_possible_paths(&self) -> usize {
        let mut occ : Vec<usize> = vec![0; self.graph.handles().count() + 1];
        self.get_possible_paths_helper(self.first_node, &mut occ);
        occ[1]
    }

    fn get_possible_paths_helper(&self, handle : Handle, occ : &mut Vec<usize>) {
        let outgoing_edges = self.graph.neighbors(handle, Direction::Right).count();
    
        if outgoing_edges == 0 {
            occ[u64::from(handle.id()) as usize] = 1;
            return;
        }

        let mut count = 0;
        for node in self.graph.neighbors(handle, Direction::Right) {
            if occ[u64::from(node.id()) as usize] == 0 {
                self.get_possible_paths_helper(node, occ);
            }
            count += occ[u64::from(node.id()) as usize];
        }

        occ[u64::from(handle.id()) as usize] = count;
    }

    pub fn label_len_sum(&self) -> usize {
        self.graph.handles().map(|handle| self.graph.get_node_unchecked(&handle.id()).sequence.len()).sum::<usize>() - FIST_NODE_LABEL.len() - LAST_NODE_LABEL.len()
    }

    ///Prints the graph's topology
    pub fn print_graph(&self) {
        for handle in self.graph.handles() {
            println!("ID : {}", handle.id());
            println!("Value : {}", self.graph.get_node_unchecked(&handle.id()).sequence.as_slice().iter().map(|&x| x as char).collect::<String>());
            let left : Vec<_> = self.graph.neighbors(handle, Direction::Left).map(|h| h.id()).collect();
            let right : Vec<_> = self.graph.neighbors(handle, Direction::Right).map(|h| h.id()).collect();
            println!("Incoming edges : {:?}", left);
            println!("outcoming Edges : {:?} \n", right);
        }
    }

    fn build_vg(alignment : &Alignment, partition : &Partition) -> (HashGraph, Handle, Handle) {
        //init
        let (mut vg, path, mut prev_handle, first_node) = VariationGraph::init(alignment);

        for interval in partition.intervals() {
            let mut segment = Vec::new();

            for sequence in alignment.sequences() {
                let subsequence : Vec<u8> = sequence.seq[interval.begin..=interval.end].iter().filter(|&&ch| ch != INDEL).map(|&ch| ch).collect();
                if !subsequence.is_empty() && !segment.contains(&subsequence) {
                    segment.push(subsequence);
                }
            }

            for subsequence in segment {
                let mut label : Vec<u8> = subsequence.clone();
                label.append(&mut format!("  [{} - {}]", interval.begin, interval.end).as_bytes().to_vec());
                let handle = vg.append_handle(&label[..]);
                //let handle = vg.append_handle(&subsequence[..]);
                for (i, sequence) in alignment.sequences().enumerate() {
                    let clean_sequence : Vec<u8> = sequence.seq[interval.begin..=interval.end].iter().filter(|&&ch| ch != INDEL).map(|&ch| ch).collect();
                    if clean_sequence == subsequence {
                        vg.create_edge(Edge(prev_handle[i], handle));
                        prev_handle[i] = handle;
                        vg.path_append_step(path[i], handle);
                    }
                }
            }
        }

        //Epilogue
        let last_node = vg.append_handle(LAST_NODE_LABEL);

        for (i, handle) in prev_handle.into_iter().enumerate() {
            vg.create_edge(Edge(handle, last_node));
            vg.path_append_step(path[i], last_node);
        }

        (vg, first_node, last_node)
    }

    fn init(alignment : &Alignment) -> (HashGraph, Vec<PathId> , Vec<Handle>, Handle) {
        let mut vg = HashGraph::new();
        let mut path = Vec::new();
        let first_handle = vg.append_handle(FIST_NODE_LABEL);
        let prev_handle = vec![first_handle; alignment.sequences().len()];
    
        for seq in alignment.sequences() {
            let p = vg.create_path(seq.name.as_bytes(), false).unwrap();
            vg.path_append_step(p, first_handle);
            path.push(p);
        }
    
        (vg, path, prev_handle, first_handle)
    }  
}