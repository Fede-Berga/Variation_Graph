use variation_graph::{
    maf_paser::{Alignment},
    variation_graph:: {VariationGraph},
};

fn main() {
    let alignment = Alignment::new("./dataset/test_2.maf").unwrap();

    let graph = VariationGraph::new(&alignment, 1 as usize);

    graph.print_graph();
}