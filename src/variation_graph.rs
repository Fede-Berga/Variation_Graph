use crate::maf_paser::{
    Alignment,
};
use handlegraph::{
    handle::{Direction, Edge, Handle},
    handlegraph::*,
    hashgraph::{
        HashGraph,
    },
    mutablehandlegraph::*,
    pathhandlegraph::*,
};
use std::cmp;

const INDEL : u8 = '-' as u8;

///Represents a variation graph
#[derive(Debug)]
pub struct VariationGraph {
    pub graph : HashGraph,
}

#[derive(Clone, Debug)]
struct Cell {
    payload : i32,
    prev : usize,
}

impl VariationGraph {
    ///Builds a variation graph given an Alignment
    pub fn new(alignment : &Alignment, threshold : usize) -> VariationGraph {
        let vg = VariationGraph::build_vg(&alignment, threshold);
        VariationGraph {graph : vg}
    }

    ///Prints the path corrisponding to 'path_name'
    pub fn print_path(&self, path_name : &str) -> Result<(), String> {
        match self.graph.path_id.get(path_name.as_bytes()) {
            Some(path) => {
                println!("Path name : {}", path_name);
                self.graph.print_path(path);
                Ok(())
            },
            _ => Err(format!("Path \'{}\' does not exist", path_name)),
        }
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

    fn build_vg(alignment : &Alignment, threshold : usize) -> HashGraph {
        //init
        let (mut vg, path, mut prev_handle, mut partition) = VariationGraph::init(alignment, threshold);
    
        /*debug
        for path_elem in path.iter() {
            vg.print_path(path_elem);
        }
    
        println!("prev_handle : {:?}", prev_handle);
        println!("partition : {:?}", partition);
        */

        //build
        let mut i = 0;
    
        while i < alignment.0[0].seq.len() {
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
        }
        
        //Debug
        /*for path_elem in path.iter() {
            vg.print_path(path_elem);
        }*/

        //Epilogue
        let last_handle = vg.append_handle(b"Last_node");

        for (i, handle) in prev_handle.into_iter().enumerate() {
            vg.create_edge(Edge(handle, last_handle));
            vg.path_append_step(path[i], last_handle);
        }

        vg
    }

    fn init(alignment : &Alignment, threshold : usize) -> (HashGraph, Vec<PathId> , Vec<Handle>, Vec<usize>) {
        let mut vg = HashGraph::new();
        let mut path = Vec::new();
        let partition = VariationGraph::get_partitioning(alignment, threshold);
        let first_handle = vg.append_handle(b"First_node");
        let prev_handle = vec![first_handle; alignment.0.len()];
    
        for seq in alignment.0.iter() {
            let p = vg.create_path(seq.name.as_bytes(), false).unwrap();
            vg.path_append_step(p, first_handle);
            path.push(p);
        }
    
        (vg, path, prev_handle, partition)
    }

    pub fn get_partitioning(alignment : &Alignment, threshold : usize) -> Vec<usize> {
        let mut dyn_prog : Vec<Cell> =vec![Cell{payload : i32::MIN, prev : 0}; threshold - 1];
        let n = alignment.0[0].seq.len();
    
        //Base Case
        println!("Base case : ");
        let base_case = VariationGraph::segment_cardinality(alignment, 0, threshold);
        dyn_prog.push(Cell{payload : base_case, prev : 0});
    
        //Recursion
        for j in threshold..n {
            println!("dyn_prog : {:#?}", dyn_prog);
            let mut min = i32::MAX;
            let mut prev = 0;
            for h in 0..=(j - threshold) {
                let mut begin = h;
                let seg_card;
                if h < threshold - 1 {
                    begin = 0;
                    println!("[{}, {}]", begin, j);
                    seg_card = VariationGraph::segment_cardinality(alignment, begin, j + 1);
                } else {
                    println!("[{}, {}]", begin + 1, j);
                    seg_card = VariationGraph::segment_cardinality(alignment, begin + 1, j + 1);
                }
                println!("M(h) = {}", dyn_prog[begin].payload);
                println!("C[h + 1, j] = {}", seg_card);
                let max = cmp::max(seg_card, dyn_prog[begin].payload);
                if min > max {
                    min = max;
                    prev = begin;
                }
            }
            println!("min : {}\n\n", min);
            dyn_prog.push(Cell{payload : min, prev : prev});
        }
    
        println!("dyn_prog : {:#?}", dyn_prog);
    
        let tb = VariationGraph::trace_back(dyn_prog, threshold);
    
        println!("bounds : {:?}", tb);

        tb
    }
    
    fn segment_cardinality(alignment : &Alignment, begin : usize, end : usize) -> i32 {
        println!("begin = {}, end = {}", begin, end - 1);
        let mut subsequences : Vec<String> = Vec::new();
        for i in 0..alignment.0.len() {
            let sub_as_string = String::from_utf8(alignment.0[i].seq[begin..end].to_vec()).unwrap();
            if !subsequences.contains(&sub_as_string) {
                subsequences.push(sub_as_string);
            }
        }
        println!("{:?}", subsequences);
        subsequences.len() as i32
    }
    
    fn trace_back(dyn_prog : Vec<Cell>, threshold : usize) -> Vec<usize> {
        let mut current = &dyn_prog[dyn_prog.len() - 1];
        let mut res = Vec::new();
    
        while current.prev != 0 {
            res.push(current.prev);
            current = &dyn_prog[current.prev];
        }
    
        res
    }

    /*
    ///Build an [handlegraph::hashgraph::HashGraph] given an ['maf_parser::Alignment']
    fn build_vg(alignment : &Alignment) -> HashGraph {
        //init
        let (mut vg, path, mut prev_handle) = VariationGraph::init(&alignment);
        
        //Building
        for i in 0..alignment.0[0].seq.len() {
            let mut current_nucleotide = Vec::new();
    
            for sequence in alignment.0.iter() {
                if sequence.seq[i] != INDEL && !current_nucleotide.contains(&sequence.seq[i]) {
                    current_nucleotide.push(sequence.seq[i]);
                }
            }
    
            for nucleotide in current_nucleotide {
                let handle = vg.append_handle(&vec![nucleotide]);
                for (j, sequence) in alignment.0.iter().enumerate() {
                    if sequence.seq[i] == nucleotide {
                        vg.create_edge(Edge(prev_handle[j], handle));
                        prev_handle[j] = handle;
                        vg.path_append_step(path[j], handle);
                    }
                }
            }
        }
        
        //Epilogue
        let last_handle = vg.append_handle(b"Last_node");

        for (i, handle) in prev_handle.into_iter().enumerate() {
            vg.create_edge(Edge(handle, last_handle));
            vg.path_append_step(path[i], last_handle);
        }
        
        vg
    }

    /// Initializes data structure for ['variation_graph::build_vg']
    fn init(alignment : &Alignment) -> (HashGraph, Vec<PathId> , Vec<Handle>) {
        let mut vg = HashGraph::new();
        let mut path = Vec::new();
        let first_handle = vg.append_handle(b"First_node");
        let prev_handle = vec![first_handle; alignment.0.len()];
    
        for seq in alignment.0.iter() {
            let p = vg.create_path(seq.name.as_bytes(), false).unwrap();
            vg.path_append_step(p, first_handle);
            path.push(p);
        }
    
        (vg, path, prev_handle)
    }*/
}