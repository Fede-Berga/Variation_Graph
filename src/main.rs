use variation_graph::{
    maf_paser::{Alignment},
    variation_graph:: {/*VariationGraph, get_partitioning,*/ build_vg},
};

fn main() {
    let alignment = Alignment::new("./dataset/test_2.maf").unwrap();

    
    build_vg(&alignment, 8 as usize);
}