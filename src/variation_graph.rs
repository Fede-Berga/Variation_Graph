use crate::maf_paser::Alignment;
/*use handlegraph::{
    handle::{Direction, Edge, Handle, NodeId},
    handlegraph::*,
    hashgraph::{
        path::{Step, StepIx},
        HashGraph,
    },
    mutablehandlegraph::*,
    pathhandlegraph::*,
};*/

const NUCLOTIDES : &[u8]= ['A' as u8, 'C' as u8, 'G' as u8,'T' as u8];
const INDEL : u8 = '-' as u8;

pub fn build_vg_from_seq_alignment(alignment : &Alignment) -> () {
    let mut current_index : Vec<usize> = vec![23; alignment.0.len()];
    //let mut vg = HashGraph::new();

    println!("prima stampa{:?}", current_index);
    println!("len : {}", alignment.0[0].seq.len());

    while current_index.iter().enumerate().any(|(i, &current)| current < alignment.0[i].seq.len()) {
        current_index = current_index
            .iter()
            .enumerate()
            .map(|(i, &current)| {
                match alignment.0[i].seq[current] {
                    INDEL => match forward_until_not_hyphen(&alignment.0[i].seq, current) {
                        Some(pos) => pos,
                        _ => alignment.0[i].seq.len(),
                    },
                    _ => current,
                }
            })
            .collect();
        
        println!("{:?}", current_index);

        /*for nucleotide in NUCLOTIDES {
            
        }*/
    }
}

fn forward_until_not_hyphen(seq : &[u8], current : usize) -> Option<usize>{
    match seq[current..].iter().position(|&ch| ch != INDEL) {
        Some(pos) => Some(pos + current),
        _ => None,
    }
}