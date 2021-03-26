use variation_graph::{
    maf_paser::{Alignment},
    variation_graph:: {VariationGraph},
};

fn main() {
    let alignment = Alignment::new("./dataset/test_2.maf").unwrap();

    let graph = VariationGraph::new(&alignment, 1 as usize).unwrap();

    for seq in alignment.0.iter() {
        match graph.print_path("gianni") {
            Err(e) => println!("Error : {:?}", e),
            _ => {},
        }
    }

    graph.print_graph();
    println!("paths : {}", graph.get_possible_paths());
}