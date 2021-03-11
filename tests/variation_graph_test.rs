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

fn run_test(file_name : &str) {
    let alignment = Alignment::new("./dataset/two_sequence.maf").unwrap();
    let graph = VariationGraph::new(&alignment);
    let path : Vec<_> = alignment.0.iter().map(|seq| seq.name.clone()).collect();

    println!("Sequences : {:?}", path);

    let mut iter = path.iter();
    while let Some(name) = iter.next() {
        graph.print_path(name.as_bytes());
        println!();
    }
    
    /*for handle in graph.graph.handles() {
        println!("{}", handle.id());
        let left : Vec<_> = graph.graph.neighbors(handle, Direction::Left).map(|h| (h.id(), graph.graph.get_node_unchecked(&h.id()).sequence.as_slice())).collect();
        let right : Vec<_> = graph.graph.neighbors(handle, Direction::Right).map(|h| (h.id(), graph.graph.get_node_unchecked(&h.id()).sequence.as_slice())).collect();
        println!("{:?}", left);
        println!("{:?}", right);
    }*/
}

#[test]
fn two_sequence() {
    run_test("./dataset/two_sequence.maf")
}