use variation_graph::{
    maf_paser::{Alignment, Sequence},
    variation_graph:: {VariationGraph},
};

use handlegraph::{
    handle::{Direction, Edge, Handle, NodeId},
    handlegraph::*,
    hashgraph::{
        path::{Step, StepIx},
        HashGraph,
    },
    mutablehandlegraph::*,
    pathhandlegraph::*,
};

fn main() {
    let graph = VariationGraph::new("./dataset/two_sequence.maf").unwrap();
    //println!("{:#?}", graph.graph.print_occurrences());
    
    for handle in graph.graph.handles() {
        println!("{}", handle.id());
        let left : Vec<_> = graph.graph.neighbors(handle, Direction::Left).map(|h| (h.id(), graph.graph.get_node_unchecked(&h.id()).sequence.as_slice())).collect();
        let right : Vec<_> = graph.graph.neighbors(handle, Direction::Right).map(|h| (h.id(), graph.graph.get_node_unchecked(&h.id()).sequence.as_slice())).collect();
        println!("{:?}", left);
        println!("{:?}", right);
    }
}