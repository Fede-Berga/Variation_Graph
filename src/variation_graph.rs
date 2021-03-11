use crate::maf_paser::{
    Alignment,
};
use handlegraph::{
    handle::{Edge, Handle},
    hashgraph::{
        HashGraph,
    },
    mutablehandlegraph::*,
    pathhandlegraph::*,
};

const INDEL : u8 = '-' as u8;

#[derive(Debug)]
pub struct VariationGraph {
    pub graph : HashGraph,
}

impl VariationGraph {
    pub fn new(alignment : &Alignment) -> VariationGraph {
        let vg = VariationGraph::build_vg(&alignment);
        VariationGraph {graph : vg}
    }

    pub fn print_path(&self, path_name : &[u8]) -> Result<(), String> {
        match self.graph.path_id.get(path_name) {
            Some(path) => {
                println!("Path name : {}", std::str::from_utf8(path_name).unwrap());
                self.graph.print_path(path);
                Ok(())
            },
            _ => Err(format!("Path \'{}\' does not exist", std::str::from_utf8(path_name).unwrap())),
        }
    }

    fn build_vg(alignment : &Alignment) -> HashGraph {
        let (mut vg, path, mut prev_handle) = VariationGraph::init(&alignment);
    
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
        
        let last_handle = vg.append_handle(b"Last_node");

        for (i, handle) in prev_handle.into_iter().enumerate() {
            vg.create_edge(Edge(handle, last_handle));
            vg.path_append_step(path[i], last_handle);
        }
        
        vg
    }

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