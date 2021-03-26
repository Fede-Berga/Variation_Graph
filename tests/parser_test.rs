use variation_graph::{
    maf_paser::Alignment,
    maf_paser::Sequence,
    maf_paser::ParserError,
};

#[test]
fn parse_file_not_found() {
    let al = Alignment::new("./dataset/file_not_found.maf");
    let expected : Result<Alignment, ParserError> = Err(ParserError::FileNotFound(String::from("Error in file reading")));
    assert_eq!(al, expected);
}

#[test]
fn parse_emply_file() {
    let al = Alignment::new("./dataset/empty.maf");
    let expected : Result<Alignment, ParserError> = Err(ParserError::AlignmentBlockNotFound(String::from("Alignment block not found")));
    assert_eq!(al, expected);
}

#[test]
fn parse_file_no_alignment_block() {
    let al : Result<Alignment, ParserError> = Alignment::new("./dataset/no_alignment_block.maf");
    let expected : Result<Alignment, ParserError> = Err(ParserError::AlignmentBlockNotFound(String::from("Alignment block not found")));
    assert_eq!(al, expected);
}

#[test]
fn parse_file_one_alignment_block() {
    let al = Alignment::new("./dataset/one_al_block.maf").unwrap();
    let expected : Vec<Sequence> = 
    vec![
        Sequence {
            name : String::from("hg18.chr7"),
            seq : vec![65, 65, 65, 45, 71, 71, 71, 65, 65, 84, 71, 84, 84, 65, 65, 67, 67, 65, 65, 65, 84, 71, 65, 45, 45, 45, 65, 84, 84, 71, 84, 67, 84, 67, 84, 84, 65, 67, 71, 71, 84, 71],
        },
        Sequence{
            name : String::from("panTro1.chr6"),
            seq : vec![65, 65, 65, 45, 71, 71, 71, 65, 65, 84, 71, 84, 84, 65, 65, 67, 67, 65, 65, 65, 84, 71, 65, 45, 45, 45, 65, 84, 84, 71, 84, 67, 84, 67, 84, 84, 65, 67, 71, 71, 84, 71],
        },
        Sequence {
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
        }
    ];
    assert_eq!(al.0, expected);
}

#[test]
fn parse_file_one_alignment_block_with_info_lines() {
    let al = Alignment::new("./dataset/one_al_block_with_info.maf").unwrap();
    let expected : Vec<Sequence> = 
    vec![
        Sequence {
            name : String::from("hg16.chr7"),
            seq : vec![71, 67, 65, 71, 67, 84, 71, 65, 65, 65, 65, 67, 65],
        },
        Sequence{
            name : String::from("panTro1.chr6"),
            seq : vec![71, 67, 65, 71, 67, 84, 71, 65, 65, 65, 65, 67, 65],
        },
        Sequence {
            name : String::from("baboon"),
            seq : vec![71, 67, 65, 71, 67, 84, 71, 65, 65, 65, 65, 67, 65],
        }
    ];
    assert_eq!(al.0, expected);
}

#[test]
fn parse_file_mul_alignment() {
    let al = Alignment::new("./dataset/mul_al_block.maf").unwrap();
    let expected : Vec<Sequence> = 
    vec![
        Sequence {
            name : String::from("hg18.chr7"),
            seq : vec![65, 65, 65, 45, 71, 71, 71, 65, 65, 84, 71, 84, 84, 65, 65, 67, 67, 65, 65, 65, 84, 71, 65, 45, 45, 45, 65, 84, 84, 71, 84, 67, 84, 67, 84, 84, 65, 67, 71, 71, 84, 71],
        },
        Sequence{
            name : String::from("panTro1.chr6"),
            seq : vec![65, 65, 65, 45, 71, 71, 71, 65, 65, 84, 71, 84, 84, 65, 65, 67, 67, 65, 65, 65, 84, 71, 65, 45, 45, 45, 65, 84, 84, 71, 84, 67, 84, 67, 84, 84, 65, 67, 71, 71, 84, 71],
        },
        Sequence {
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
        }
    ];
    assert_eq!(al.0, expected);
}

#[test]
fn parse_file_upper_case() {
    let al = Alignment::new("./dataset/test_3.maf").unwrap();
    let expected : Vec<Sequence> = 
    vec![
        Sequence {
            name : String::from("hg18.chr7"),
            seq : vec![84, 65, 65, 65, 71, 65],
        },
        Sequence{
            name : String::from("panTro1.chr6"),
            seq : vec![84, 65, 65, 65, 71, 65],
        },
        Sequence {
            name : String::from("baboon"),
            seq :  vec![84, 65, 65, 65, 71, 65],
        },
        Sequence {
            name : String::from("mm4.chr6"),
            seq : vec![84, 65, 65, 65, 71, 65],
        },
        Sequence {
            name :  String::from("rn3.chr4"),
            seq : vec![84, 65, 65, 71, 71, 65],
        }
    ];
    assert_eq!(al.0, expected);
}


