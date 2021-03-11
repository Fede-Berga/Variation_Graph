use variation_graph::{
    maf_paser::{Alignment, Sequence},
    variation_graph:: {VariationGraph},
};

use handlegraph::{
    handle::{Direction, Edge, Handle, NodeId},
    handlegraph::*,
    hashgraph::{
        path::{Step, StepIx},
        HashGraph,
    },
    mutablehandlegraph::*,
    pathhandlegraph::*,
};

fn run_test(file_name : &str) -> Result<(), String>{
    let alignment = Alignment::new(file_name)?;
    let graph = VariationGraph::new(&alignment);
    let path : Vec<_> = alignment.0.iter().map(|seq| seq.name.clone()).collect();

    println!("Sequences : {:?} \n", path);

    let mut iter = path.iter();
    while let Some(name) = iter.next() {
        graph.print_path(&name)?;
        println!();
    }
    
    graph.print_graph();

    Ok(())
}

#[test]
fn file_not_found() {
    match run_test("./dataset/file_not_found.maf") {
        Ok(_) => println!("Successful"),
        Err(error) => println!("Error : {}", error),
    }
}

#[test]
fn empty_file() {
    match run_test("./dataset/empty.maf") {
        Ok(_) => println!("Successful"),
        Err(error) => println!("Error : {}", error),
    }
}

#[test]
fn two_sequence() {
    match run_test("./dataset/two_sequence.maf") {
        Ok(_) => println!("Successful \n\n"),
        Err(error) => println!("Error : {} \n\n", error),
    }
}

#[test]
fn multiple_sequences() {
    match run_test("./dataset/one_al_block.maf") {
        Ok(_) => println!("Successful \n\n"),
        Err(error) => println!("Error : {} \n\n", error),
    }
}