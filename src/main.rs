pub mod maf_paser;

use maf_paser::Alignment;

fn main() {
    let al : Alignment = Alignment::new("./dataset/input_2.maf");

    println!("{}", al);

    /*let aschar : Vec<Vec<char>> = alignment.iter().map(|seq| seq.iter().map(|byte| *byte as char).collect::<Vec<char>>()).collect();

    println!("{:?}", aschar);*/
}


