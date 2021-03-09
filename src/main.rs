use variation_graph::{
    maf_paser::{Alignment, Sequence},
    variation_graph::{
        build_vg_V1,
        build_vg_v2,
    },
};
/*use handlegraph::{
    handle::{Direction, Edge, Handle, NodeId},
    handlegraph::*,
    hashgraph::{
        path::{Step, StepIx},
        HashGraph,
    },
    mutablehandlegraph::*,
    pathhandlegraph::*,
};*/

fn main() {
    let al : Vec<Sequence> = 
    vec![
        Sequence {
            name : String::from("hg18.chr7"),
            seq : vec![65, 65, 65, 45, 71, 71, 71, 65, 65, 84, 71, 84, 84, 65, 65, 67, 67, 65, 65, 65, 84, 71, 65, 45, 45, 45, 65, 84, 84, 71, 84, 67, 84, 67, 84, 84, 65, 67, 71, 71, 84, 71],
        },
        Sequence{
            name : String::from("panTro1.chr6"),
            seq : vec![65, 65, 65, 45, 71, 71, 71, 65, 65, 84, 71, 84, 84, 65, 65, 67, 67, 65, 65, 65, 84, 71, 65, 45, 45, 45, 65, 84, 84, 71, 84, 67, 84, 67, 84, 84, 65, 67, 71, 71, 84, 71],
        },
        /*Sequence {
            name : String::from("baboon"),
            seq :  vec![65, 65, 65, 45, 71, 71, 71, 65, 65, 84, 71, 84, 84, 65, 65, 67, 67, 65, 65, 65, 84, 71, 65, 45, 45, 45, 71, 84, 84, 71, 84, 67, 84, 67, 84, 84, 65, 84, 71, 71, 84, 71],
        },
        Sequence {
            name : String::from("mm4.chr6"),
            seq : vec![45, 65, 65, 84, 71, 71, 71, 65, 65, 84, 71, 84, 84, 65, 65, 71, 67, 65, 65, 65, 67, 71, 65, 45, 45, 45, 65, 84, 84, 71, 84, 67, 84, 67, 84, 67, 65, 71, 84, 71, 84, 71],
        },
        Sequence {
            name :  String::from("rn3.chr4"),
            seq : vec![45, 65, 65, 45, 71, 71, 71, 71, 65, 84, 71, 67, 84, 65, 65, 71, 67, 67, 65, 65, 84, 71, 65, 71, 84, 84, 71, 84, 84, 71, 84, 67, 84, 67, 84, 67, 65, 65, 84, 71, 84, 71],
        }*/
    ];

    let expected = Alignment(al);
    build_vg_v2(&expected);
    println!("allignment occ mem : {}", std::mem::size_of_val(&expected));
}