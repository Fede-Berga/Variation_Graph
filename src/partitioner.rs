use crate::maf_paser::Alignment;
use std::cmp;
#[derive(Clone, Debug)]
pub struct Cell {
    payload : usize,
    prev : usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub begin : usize,
    pub end : usize,
}

impl Interval {
    fn next(&mut self) {
        self.begin = self.end + 1;
        self.end = self.begin;
    }
}

pub struct Partitioner;

impl Partitioner {

    pub fn new(alignment : &Alignment, threshold : usize) -> Vec<Interval> {
        let mut dyn_prog : Vec<Cell> = vec![Cell{payload : 0, prev : 0}; threshold - 1];
    
        //Base Case
        //println!("Base case : ");
        let base_case = Partitioner::segment_cardinality(alignment, 0, threshold - 1);
        dyn_prog.push(Cell{payload : base_case, prev : 0});
    
        //Recursion
        Partitioner::recursion(threshold, alignment, &mut dyn_prog);
    
        //println!("dyn_prog : {:#?}", dyn_prog);
    
        let tb = Partitioner::trace_back(dyn_prog);
    
        //println!("bounds : {:?}", tb);

        tb
    }

    fn segment_cardinality(alignment : &Alignment, begin : usize, end : usize) -> usize {
        //println!("[{}, {}]", begin, end - 1);
        let mut subsequences : Vec<String> = Vec::new();
        for i in 0..alignment.0.len() {
            let sub_as_string = String::from_utf8(alignment.0[i].seq[begin..=end].to_vec()).unwrap();
            if !subsequences.contains(&sub_as_string) {
                subsequences.push(sub_as_string);
            }
        }
        //println!("{:?}", subsequences);
        subsequences.len()
    }

    fn trace_back(dyn_prog : Vec<Cell>) -> Vec<Interval> {
        let mut current = &dyn_prog[dyn_prog.len() - 1];
        let mut res = Vec::new();
        //println!("n : {}", dyn_prog.len());
        let mut interval = Interval {begin : current.prev + 1, end : dyn_prog.len() - 1};
    
        while current.prev != 0 {
            res.push(interval);
            interval.end = interval.begin - 1;
            current = &dyn_prog[current.prev];
            if current.prev != 0 {
                interval.begin = current.prev + 1;
            }
        }

        interval.begin = 0;
        res.push(interval);
        
        res.reverse();

        res
    }

    #[allow(dead_code)]
    fn recursion(threshold : usize, alignment : &Alignment, dyn_prog : &mut Vec<Cell>) {
        //println!("Partitioner");
        let n = alignment.sequences().next().unwrap().len();
    
        for j in threshold..n {
            //println!("dyn_prog : {:#?}", dyn_prog);
            let mut min = usize::MAX;
            let mut prev = 0;
            for h in 0..=(j - threshold) {
                let mut begin = h;
                let seg_card;
                if h < threshold - 1 {
                    begin = 0;
                    //println!("[{}, {}]", begin, j);
                    seg_card = Partitioner::segment_cardinality(alignment, begin, j);
                } else {
                    //println!("[{}, {}]", begin + 1, j);
                    seg_card = Partitioner::segment_cardinality(alignment, begin + 1, j);
                }
                //println!("M({}) = {}", begin, dyn_prog[begin].payload);
                //println!("C[{}, {}] = {}", begin + 1, j, seg_card);
                let max = cmp::max(seg_card, dyn_prog[begin].payload);
                if min > max {
                    min = max;
                    prev = begin;
                }
            }
            //println!("min : {}\n\n", min);
            //println!("dyn_prog : {:#?}", dyn_prog);
            dyn_prog.push(Cell{payload : min, prev : prev});
        }

        //println!("End Partitioner");
    }
}

pub struct GreedyPartitioner;

impl GreedyPartitioner {

    pub fn new(alignment : &Alignment, threshold : usize) -> Vec<Interval> {
        GreedyPartitioner::greedy(alignment, threshold)
    }

    fn segment_cardinality(alignment : &Alignment, interval : Interval) -> usize {
        //println!("[{}, {}]", interval.begin, interval.end);
        let indel_string : String = vec!['-'; interval.end - interval.begin + 1].iter().collect();
        let mut subsequences : Vec<String> = Vec::new();
        for seq in alignment.sequences() {
            let sub_as_string = String::from_utf8(seq.seq[interval.begin..=interval.end].to_vec()).unwrap();
            if sub_as_string != indel_string && !subsequences.contains(&sub_as_string) {
                subsequences.push(sub_as_string);
            }
        }
        //println!("{:?}", subsequences);
        subsequences.len()
    }

    fn greedy(alignment : &Alignment, threshold : usize) -> Vec<Interval> {
        let n = alignment.sequences().next().unwrap().len();
        let mut res : Vec<Interval> = Vec::new();
        let mut interval = Interval {begin : 0, end : 0};

        while interval.begin < n {
            let mut seg_car = GreedyPartitioner::segment_cardinality(alignment, interval);

            if seg_car > threshold {
                let mut curr_car = seg_car;
                let mut single_elem_car = seg_car;
                while curr_car == seg_car && single_elem_car == seg_car && interval.end < n {
                    interval.end += 1;
                    if interval.end < n {
                        curr_car = GreedyPartitioner::segment_cardinality(alignment, interval);
                        let single_interval = Interval {begin : interval.end, end : interval.end};
                        single_elem_car = GreedyPartitioner::segment_cardinality(alignment, single_interval);
                    }
                }
            } else {
                while seg_car <= threshold && interval.end < n {
                    interval.end += 1;
                    if interval.end < n {
                        seg_car = GreedyPartitioner::segment_cardinality(alignment, interval);
                    }
                }
            }

            interval.end -= 1;
            res.push(interval);

            interval.next();
        }
        
        res
    }
}