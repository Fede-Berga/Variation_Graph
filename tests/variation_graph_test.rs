//Run the tests using :
//cargo test [test_name] -- --nocapture

use variation_graph::{
    alignment_parser::*,
    partitioner::*,
    variation_graph::{VariationGraph, VariationGraphError},
};

fn run_test(file_name : &str) -> Result<(), String>{
    let alignment = match MafParser::get_alignment(file_name){
        Ok(al) => al,
        Err(e) => return Err(format!("{:?}", e)),
    };

    let partition = match <GreedyPartitioner as Partitioner>::new(&alignment, 1 as usize){
        Ok(partition) => partition,
        Err(e) => return Err(format!("{:?}", e)),
    };

    let graph = VariationGraph::new(&alignment, &partition);

    let path : Vec<_> = alignment.sequences().map(|seq| seq.name.clone()).collect();

    println!("Sequences : {:?} \n", path);

    let mut iter = path.iter();
    while let Some(name) = iter.next() {
        match graph.print_path(&name) {
            Err(e) => return Err(format!("{:?}", e)),
            _ => {},
        }
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

#[test]
fn multiple_sequences_with_info_block() {
    match run_test("./dataset/one_al_block_with_info.maf") {
        Ok(_) => println!("Successful \n\n"),
        Err(error) => println!("Error : {} \n\n", error),
    }
}

#[test]
fn test_1() {
    match run_test("./dataset/test_1.maf") {
        Ok(_) => println!("Successful \n\n"),
        Err(error) => println!("Error : {} \n\n", error),
    }
}

#[test]
fn test_2() {
    match run_test("./dataset/test_2.maf") {
        Ok(_) => println!("Successful \n\n"),
        Err(error) => println!("Error : {} \n\n", error),
    }
}

#[test]
fn test_3() {
    match run_test("./dataset/test_3.maf") {
        Ok(_) => println!("Successful \n\n"),
        Err(error) => println!("Error : {} \n\n", error),
    }
}
