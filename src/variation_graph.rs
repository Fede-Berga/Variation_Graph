use crate::maf_paser::{
    Alignment,
    //Sequence,
};
use handlegraph::{
    handle::{Direction, Edge, Handle, NodeId},
    handlegraph::*,
    hashgraph::{
        path::{Step, StepIx},
        HashGraph,
        Node,
    },
    mutablehandlegraph::*,
    pathhandlegraph::*,
};

const INDEL : u8 = '-' as u8;

pub fn build_vg_v2(alignment : &Alignment) ->() {
    let mut vg = HashGraph::new();
    let mut path = Vec::new();
    let first_handle = vg.append_handle(b"First_node");
    let mut prev_handle = vec![first_handle; alignment.0.len()];

    for seq in alignment.0.iter() {
        let p = vg.create_path(seq.name.as_bytes(), false).unwrap();
        vg.path_append_step(p, first_handle);
        path.push(p);
    }

    println!("{:?}", prev_handle);

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

    for path_item in path.iter() {
        vg.print_path(path_item);
    }

    println!("graph occ mem : {}", std::mem::size_of_val(&vg));
}

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