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