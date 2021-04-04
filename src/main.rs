#[allow(unused_imports)]
use variation_graph::{
    maf_paser::*,
    variation_graph:: {VariationGraph},
};

fn main() {
    let alignment = FastaParser::get_alignment("../dataset/out.fa").unwrap();

    //let part = Partitioner::new(&alignment, 2);

    //println!("part : {:?}", part);
    
    //println!("al : {}", alignment);

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

    //graph.print_graph();
    println!("paths : {}", graph.get_possible_paths());
}