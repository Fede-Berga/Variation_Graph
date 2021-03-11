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

const INDEL : u8 = '-' as u8;

///Represents a variation graph
#[derive(Debug)]
pub struct VariationGraph {
    pub graph : HashGraph,
}

impl VariationGraph {
    ///Builds a variation graph given an Alignment
    pub fn new(alignment : &Alignment) -> VariationGraph {
        let vg = VariationGraph::build_vg(&alignment);
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
    }
}