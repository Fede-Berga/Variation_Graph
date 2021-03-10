use variation_graph::{
    maf_paser::{Alignment, Sequence},
    variation_graph:: {VariationGraph},
};

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

#[test]
fn file_not_found() {
    let graph = VariationGraph::new("./dataset/file_not_found.maf");
    assert_eq!("Error in file reading", graph.err().unwrap());
}

#[test]
fn two_sequence() {
    let graph = VariationGraph::new("./dataset/two_sequence.maf");
    println!("{:#?}", graph);
    assert_eq!(true, false);
}