#[allow(unused_imports)]
use variation_graph::{
    maf_paser::*,
    partitioner::*,
    variation_graph:: {VariationGraph},
};
//../dataset/ENSG00000000005.6.aligned.fa
//../dataset/ENSG00000000419.13.aligned.fa
//../dataset/ENSG00000000457.14.aligned.fa
//../dataset/ENSG00000000460.17.aligned.fa
fn main() {
    let input_file_name = "../dataset/ENSG00000000419.13.aligned.fa";

    let alignment = FastaParser::get_alignment(input_file_name).unwrap();

    //let alignment = MafParser::get_alignment("./dataset/test_1.maf").unwrap();

    let output_file_name = "./dataset/ENSG00000000419.13.aligned.txt";

    let part = GreedyPartitioner::new(&alignment, 1);

    alignment.dump_on_file(output_file_name, &part);

    //println!("part : {:?}", part);
    
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

    graph.print_graph();
    println!("sum labels len : {}", graph.label_len_sum());
    println!("paths : {}", graph.get_possible_paths());
}

