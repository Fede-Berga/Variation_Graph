use variation_graph::{
    maf_paser::{Alignment},
    variation_graph:: {VariationGraph},
};

fn main() {
    let alignment = Alignment::new("./dataset/one_al_block.maf").unwrap();

    let graph = VariationGraph::new(&alignment, 3 as usize).unwrap();

    for seq in alignment.0.iter() {
        match graph.print_path(&seq.name) {
            Err(e) => println!("Error : {:?}", e),
            _ => {},
        }
    }

    graph.print_graph();
    println!("paths : {}", graph.get_possible_paths());
}