use variation_graph::{
    maf_paser::{Alignment, Sequence},
    variation_graph:: {build_vg_from_seq_alignment},
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

#[test]
fn first_test() {
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
        Sequence{
            name : String::from("panTro1.chr6"),
            seq : vec![65, 65, 65, 45, 71, 71, 71, 65, 65, 84, 71, 84, 84, 65, 65, 67, 67, 65, 65, 65, 84, 71, 65, 45, 45, 45, 65, 84, 84, 71, 84, 67, 84, 67, 84, 84, 65, 67, 71, 71, 84, 71],
        },
    ];

    let expected = Alignment(al);
    build_vg_from_two_seq_alignment(&expected);
    assert_eq!(true, false);
}