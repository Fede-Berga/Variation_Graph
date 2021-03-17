use variation_graph::{
    maf_paser::{Alignment},
    variation_graph:: {VariationGraph, get_partitioning},
};

fn main() {
    let alignment = Alignment::new("./dataset/test_01.maf").unwrap();

    
    get_partitioning(&alignment, 3 as usize);
}