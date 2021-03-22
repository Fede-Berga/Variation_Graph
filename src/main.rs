use variation_graph::{
    maf_paser::{Alignment},
    variation_graph:: {VariationGraph},
};

fn main() {
    let alignment = Alignment::new("./dataset/test_2.maf").unwrap();

    let graph = VariationGraph::new(&alignment, 4 as usize);

    for seq in alignment.0.iter() {
        graph.print_path(&seq.name).unwrap();
    }

    graph.print_graph();
}