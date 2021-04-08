#[allow(unused_imports)]
use variation_graph::{
    maf_paser::*,
    partitioner::*,
    variation_graph:: {VariationGraph},
};
//../dataset/ENSG00000000005.6.aligned.fa
//../dataset/ENSG00000000419.13.aligned.fa
fn main() {
    let alignment = FastaParser::get_alignment("../dataset/ENSG00000000419.13.aligned.fa").unwrap();

    let part = GreedyPartitioner::new(&alignment, 1);

    println!("part : {:?}", part);
    
    //println!("al : {}", alignment);
    
    for elem in alignment.sequences() {
        println!("Seq : {}", elem);
    }

    let graph = VariationGraph::new(&alignment, 1).unwrap();

    for seq in alignment.0.iter() {
        match graph.print_path(&seq.name) {
            Err(e) => println!("Error : {:?}", e),
            _ => {},
        }
    }

    println!("max : {}", usize::MAX);
    //println!("BITS : {}", usize::BITS);

    graph.print_graph();
    println!("sum labels len : {}", graph.label_len_sum());
    println!("paths : {}", graph.get_possible_paths());
}