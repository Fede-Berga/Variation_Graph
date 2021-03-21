use variation_graph::{
    maf_paser::{Alignment},
    variation_graph:: {VariationGraph},
};

fn main() {
    let alignment = Alignment::new("./dataset/test_01.maf").unwrap();

    let graph = VariationGraph::new(&alignment, 2 as usize);

    graph.print_graph();
}