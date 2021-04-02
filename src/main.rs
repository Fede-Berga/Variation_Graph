use variation_graph::{
    maf_paser::{Alignment, Sequence, FastaParser, Parser, MafParser},
    partitioner::GreedyPartitioner,
    variation_graph:: {VariationGraph},
};

fn main() {
    let alignment = MafParser::get_alignment("./dataset/test_1.maf").unwrap();

    let part = GreedyPartitioner::new(&alignment, 2);

    println!("part : {:?}", part);
    
    /*println!("al : {}", alignment);

    for elem in alignment.sequences() {
        println!("Seq : {}", elem);
    }

    let graph = VariationGraph::new(&alignment, 1 as usize).unwrap();

    for seq in alignment.0.iter() {
        match graph.print_path(&seq.name) {
            Err(e) => println!("Error : {:?}", e),
            _ => {},
        }
    }

    graph.print_graph();
    println!("paths : {}", graph.get_possible_paths());*/
}