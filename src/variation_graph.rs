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
    graph : HashGraph,
}

impl VariationGraph {
    pub fn new(file_name : &str) -> Result<VariationGraph, &'static str> {
        match Alignment::new(file_name) {
            Ok(alignment) => Ok(VariationGraph::build_graph(&alignment)),
            Err(error) => Err(error),
        }
    }

    pub fn build_graph(alignment : &Alignment) -> VariationGraph {
        let vg = VariationGraph::build_vg(&alignment);
        VariationGraph {graph : vg,}
    }

    fn build_vg(alignment : &Alignment) -> HashGraph {
        let (mut vg, path, mut prev_handle) = VariationGraph::init(&alignment);
    
        println!("After Initialization : ");
        println!("vg : {:#?}", vg);
        println!("path : {:#?}", path);
        println!("prev_handle : {:#?}", prev_handle);
        println!("graph occ mem : {}", std::mem::size_of_val(&vg));
    
        for i in 0..alignment.0[0].seq.len() {
            let mut current_nucleotide = Vec::new();
    
            for sequence in alignment.0.iter() {
                if sequence.seq[i] != INDEL && !current_nucleotide.contains(&sequence.seq[i]) {
                    current_nucleotide.push(sequence.seq[i]);
                }
            }
    
            println!("{:?}", current_nucleotide);
    
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
        //Add Last Node
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
/*
pub fn build_vg_V1(alignment : &Alignment) -> () {
    let mut current_index : Vec<usize> = vec![0; alignment.0.len()];
    let (mut vg, path) = initialize_graph(&alignment);

    while current_index.iter().enumerate().any(|(i, &current)| current < alignment.0[i].seq.len()) {
        current_index = set_current_index(&alignment, current_index);
        
        println!("current_index : {:?}", current_index);

        //Build Graph

        let mut current_nucleotide = Vec::new();

        for (i, &current) in current_index.iter().enumerate() {
            if current < alignment.0[i].seq.len() && !current_nucleotide.contains(&alignment.0[i].seq[current]) {
                current_nucleotide.push(alignment.0[i].seq[current]);
            }
        }

        println!("{:?}", current_nucleotide);

        for nucleotide in current_nucleotide {
            let handle = vg.append_handle(&vec![nucleotide]);
            for (i, &current) in current_index.iter().enumerate() {
                if current < alignment.0[i].seq.len() && alignment.0[i].seq[current] == nucleotide {
                    vg.path_append_step(path[i], handle);
                }
            } 
        }
        
        current_index = current_index.iter().map(|&current| current + 1).collect();
    }

    for path_item in path.iter() {
        vg.print_path(path_item);
    }
}

fn forward_until_not_hyphen(seq : &[u8], current : usize) -> Option<usize>{
    match seq[current..].iter().position(|&ch| ch != INDEL) {
        Some(pos) => Some(pos + current),
        _ => None,
    }
}

fn initialize_graph(alignment : &Alignment) -> (HashGraph, Vec<PathId>) {
    let mut graph = HashGraph::new();
    let mut path = Vec::new();
    let first_handle = graph.append_handle(b"First_node");

    for seq in alignment.0.iter() {
        let p = graph.create_path(seq.name.as_bytes(), false).unwrap();
        graph.path_append_step(p, first_handle);
        path.push(p);
    }

    (graph, path)
}

fn set_current_index(alignment : &Alignment, current_index : Vec<usize>) -> Vec<usize> {
    current_index.iter()
            .enumerate()
            .map(|(i, &current)| {
                if current >= alignment.0[i].seq.len() {
                    current
                } else {
                    match alignment.0[i].seq[current] {
                        INDEL => match forward_until_not_hyphen(&alignment.0[i].seq, current) {
                            Some(pos) => pos,
                            _ => alignment.0[i].seq.len(),
                        },
                        _ => current,
                    }
                }
            })
            .collect()
}
*/