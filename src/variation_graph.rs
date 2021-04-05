#[allow(unused_imports)]
use crate::maf_paser::{
    Alignment,
};
#[allow(unused_imports)]
use crate::partitioner::{
    GreedyPartitioner,
    Partitioner,
    Interval
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
//fix getpossiblepath

const INDEL : u8 = '-' as u8;

///Represents a variation graph
#[derive(Debug)]
pub struct VariationGraph {
    pub graph : HashGraph,
    first_node : Handle,
    last_node : Handle,
}

#[derive(Debug)]
pub enum VariationGraphError {
    InvalidThreshold(String),
    PathNotFound(String),
}

impl VariationGraph {
    ///Builds a variation graph given an Alignment
    pub fn new(alignment : &Alignment, threshold : usize) -> Result<VariationGraph, VariationGraphError> {
        let upper_bound = alignment.0[0].seq.len();
        if threshold > 0 && threshold < upper_bound{
            let (vg, first_node, last_node) = VariationGraph::build_vg(&alignment, threshold);
            Ok(VariationGraph {graph : vg, first_node : first_node, last_node : last_node})
        } else {
            Err(
                VariationGraphError::InvalidThreshold(
                    format!("threshold : {} not valid, must be [1, {}]", threshold, upper_bound)
                )
            )
        }
    }

    ///Prints the path corrisponding to 'path_name'
    pub fn print_path(&self, path_name : &str) -> Result<(), VariationGraphError> {
        match self.graph.path_id.get(path_name.as_bytes()) {
            Some(path) => {
                println!("Path name : {}", path_name);
                self.graph.print_path(path);
                Ok(())
            },
            _ => Err(VariationGraphError::PathNotFound(format!("Path \"{}\" does not exist", path_name))),
        }
    }

    pub fn get_possible_paths(&self) -> usize {
        //let outgoing_edges = self.graph.neighbors(self.last_node, Direction::Right).count();
        //println!("out : {}", outgoing_edges);
        let mut reps = Vec::new();
        self.get_possible_paths_helper(self.first_node, &mut reps)
    }

    fn get_possible_paths_helper(&self, handle : Handle, reps : &mut Vec<NodeId>) -> usize {
        //println!("handle : {}", handle.id());
        //println!("first_handle : {}", self.first_node.id());
        //println!("last_handle : {}", self.last_node.id());
        if reps.contains(&handle.id()) {
            println!("loop : {}", handle.id());
        } else {
            reps.push(handle.id());
        }

        let outgoing_edges = self.graph.neighbors(handle, Direction::Right).count();

        if outgoing_edges == 0 {
            //println!("handle : {}", handle.id());
            //println!("first_handle : {}", self.first_node.id());
            //println!("last_handle : {}", self.last_node.id());
            //println!("reps : {:?}", reps);
            return 1 as usize;
        }

        let mut count = 0;
        for node in self.graph.neighbors(handle, Direction::Right) {
            let mut vec = reps.clone();
            count += self.get_possible_paths_helper(node, &mut vec);
        }

        //println!("count : {}", count);

        return count;
    }

    pub fn lable_len_sum(&self) -> usize {
        self.graph.handles().map(|handle| self.graph.get_node_unchecked(&handle.id()).sequence.len()).sum()
    }

    ///Prints the graph's topology
    pub fn print_graph(&self) {
        for handle in self.graph.handles() {
            println!("ID : {}", handle.id());
            println!("Value : {}", self.graph.get_node_unchecked(&handle.id()).sequence.as_slice().iter().map(|&x| x as char).collect::<String>());
            let left : Vec<_> = self.graph.neighbors(handle, Direction::Left).map(|h| h.id()).collect();
            let right : Vec<_> = self.graph.neighbors(handle, Direction::Right).map(|h| h.id()).collect();
            println!("Outgoing edges : {:?}", left);
            println!("Incoming Edges : {:?} \n", right);
        }
    }

    fn build_vg(alignment : &Alignment, threshold : usize) -> (HashGraph, Handle, Handle) {
        //init
        let (mut vg, path, mut prev_handle, partition, first_node) = VariationGraph::init(alignment, threshold);

        for interval in partition.iter() {
            let mut segment = Vec::new();

            for sequence in alignment.sequences() {
                let subsequence : Vec<u8> = sequence.seq[interval.begin..=interval.end].iter().filter(|&&ch| ch != INDEL).map(|&ch| ch).collect();
                if !subsequence.is_empty() && !segment.contains(&subsequence) {
                    segment.push(subsequence);
                }
            }

            for subsequence in segment {
                let handle = vg.append_handle(&subsequence[..]);
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

        /*let mut i = 0;
        let n = slignment.sequences().next().unwrap().len();
    
        while i < n {
            let mut segment = Vec::new();
            let upper_bound = match partition.pop() {
                Some(ub) => ub,
                None => alignment.0[0].seq.len() - 1,
            };
    
            for sequence in alignment.0.iter() {
                let subsequence : Vec<u8> = sequence.seq[i..=upper_bound].iter().filter(|&&ch| ch != INDEL).map(|&ch| ch).collect();
                if !segment.contains(&subsequence) {
                    segment.push(subsequence);
                }
            }
    
            //println!("segment : {:?}", segment.iter().map(|sub| sub.iter().map(|&ch| ch as char).collect::<String>()).collect::<Vec<_>>());
    
            for subsequence in segment {
                if ! subsequence.is_empty() {
                    let handle = vg.append_handle(&subsequence[..]);
                    for (j, sequence) in alignment.0.iter().enumerate() {
                        let clean_sequence : Vec<u8> = sequence.seq[i..=upper_bound].iter().filter(|&&ch| ch != INDEL).map(|&ch| ch).collect();
                        //println!("{:?} - {:?}", clean_sequence, subsequence);
                        if clean_sequence == subsequence {
                            vg.create_edge(Edge(prev_handle[j], handle));
                            prev_handle[j] = handle;
                            vg.path_append_step(path[j], handle);
                        }
                    }
                }
            }
    
            i = upper_bound + 1;
        }*/

        //Epilogue
        let last_node = vg.append_handle(b"Last_node");

        for (i, handle) in prev_handle.into_iter().enumerate() {
            vg.create_edge(Edge(handle, last_node));
            vg.path_append_step(path[i], last_node);
        }

        (vg, first_node, last_node)
    }

    fn init(alignment : &Alignment, threshold : usize) -> (HashGraph, Vec<PathId> , Vec<Handle>, Vec<Interval>, Handle) {
        let mut vg = HashGraph::new();
        let mut path = Vec::new();
        let partition = GreedyPartitioner::new(alignment, threshold);
        let first_handle = vg.append_handle(b"First_node");
        let prev_handle = vec![first_handle; alignment.0.len()];
    
        for seq in alignment.0.iter() {
            let p = vg.create_path(seq.name.as_bytes(), false).unwrap();
            vg.path_append_step(p, first_handle);
            path.push(p);
        }

        //println!("partition : {:?}", partition);
    
        (vg, path, prev_handle, partition, first_handle)
    }  
}