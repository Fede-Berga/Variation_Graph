use crate::maf_paser::Alignment;
use std::cmp;

#[derive(Clone, Debug)]
pub struct Cell {
    payload : i32,
    prev : usize,
}

pub trait Partitioner {
    fn new(alignment : &Alignment, threshold : usize) -> Vec<usize> {
        let mut dyn_prog : Vec<Cell> =vec![Cell{payload : i32::MIN, prev : 0}; threshold - 1];
    
        //Base Case
        println!("Base case : ");
        let base_case = PartitionerV1::segment_cardinality(alignment, 0, threshold);
        dyn_prog.push(Cell{payload : base_case, prev : 0});
    
        //Recursion
        PartitionerV1::recursion(threshold, alignment, &mut dyn_prog);
    
        println!("dyn_prog : {:#?}", dyn_prog);
    
        let tb = PartitionerV1::trace_back(dyn_prog);
    
        println!("bounds : {:?}", tb);

        tb
    }

    fn recursion(threshold : usize, alignment : &Alignment, dyn_prog : &mut Vec<Cell>);

    fn segment_cardinality(alignment : &Alignment, begin : usize, end : usize) -> i32 {
        println!("begin = {}, end = {}", begin, end - 1);
        let mut subsequences : Vec<String> = Vec::new();
        for i in 0..alignment.0.len() {
            let sub_as_string = String::from_utf8(alignment.0[i].seq[begin..end].to_vec()).unwrap();
            if !subsequences.contains(&sub_as_string) {
                subsequences.push(sub_as_string);
            }
        }
        println!("{:?}", subsequences);
        subsequences.len() as i32
    }

    fn trace_back(dyn_prog : Vec<Cell>) -> Vec<usize> {
        let mut current = &dyn_prog[dyn_prog.len() - 1];
        let mut res = Vec::new();
    
        while current.prev != 0 {
            res.push(current.prev);
            current = &dyn_prog[current.prev];
        }
    
        res
    }
}

#[derive(Debug)]
pub struct PartitionerV1;

impl Partitioner for PartitionerV1 {

    fn recursion(threshold : usize, alignment : &Alignment, dyn_prog : &mut Vec<Cell>) {
        let n = alignment.0[0].seq.len();

        for j in threshold..n {
            println!("dyn_prog : {:#?}", dyn_prog);
            let mut min = i32::MAX;
            let mut prev = 0;
            for h in 0..=(j - threshold) {
                let mut begin = h;
                let seg_card;
                if h < threshold - 1 {
                    begin = 0;
                    println!("[{}, {}]", begin, j);
                    seg_card = PartitionerV1::segment_cardinality(alignment, begin, j + 1);
                } else {
                    println!("[{}, {}]", begin + 1, j);
                    seg_card = PartitionerV1::segment_cardinality(alignment, begin + 1, j + 1);
                }
                println!("M(h) = {}", dyn_prog[begin].payload);
                println!("C[h + 1, j] = {}", seg_card);
                if min > seg_card {
                    min = seg_card;
                    prev = begin;
                }
            }
            println!("min : {}\n\n", min);
            dyn_prog.push(Cell{payload : min, prev : prev});
        }
    }
}

#[derive(Debug)]
pub struct PartitionerV2;

impl Partitioner for PartitionerV2 {

    fn recursion(threshold : usize, alignment : &Alignment, dyn_prog : &mut Vec<Cell>) {
        let n = alignment.0[0].seq.len();
    
        for j in threshold..n {
            println!("dyn_prog : {:#?}", dyn_prog);
            let mut min = i32::MAX;
            let mut prev = 0;
            for h in 0..=(j - threshold) {
                let mut begin = h;
                let seg_card;
                if h < threshold - 1 {
                    begin = 0;
                    println!("[{}, {}]", begin, j);
                    seg_card = PartitionerV2::segment_cardinality(alignment, begin, j + 1);
                } else {
                    println!("[{}, {}]", begin + 1, j);
                    seg_card = PartitionerV2::segment_cardinality(alignment, begin + 1, j + 1);
                }
                println!("M(h) = {}", dyn_prog[begin].payload);
                println!("C[h + 1, j] = {}", seg_card);
                let max = cmp::max(seg_card, dyn_prog[begin].payload);
                if min > max {
                    min = max;
                    prev = begin;
                }
            }
            println!("min : {}\n\n", min);
            dyn_prog.push(Cell{payload : min, prev : prev});
        }
    }

}