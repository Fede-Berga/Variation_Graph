use variation_graph::{
    maf_paser::Alignment,
    maf_paser::Sequence,
};

#[test]
fn parse_file_non_presente() {
    let al = Alignment::new("./dataset/non_presente.maf");
    let expected : Result<Alignment, &str> = Err("Errore nella lettura del file");
    assert_eq!(al, expected);
}

#[test]
fn parse_emply_file() {
    let al = Alignment::new("./dataset/empty.maf");
    let expected : Result<Alignment, &str> = Err("Errore nella lettura del file");
    assert_eq!(al, expected);
}

#[test]
fn parse_file_no_alignment_block() {
    let al = Alignment::new("./dataset/no_alignment_block.maf");
    let expected : Result<Alignment, &str> = Err("Blocco di allineamento non trovato");
    assert_eq!(al, expected);
}

#[test]
fn parse_file_one_alignment_block() {
    let al = Alignment::new("./dataset/input.maf").unwrap();
    let expected : Vec<Sequence> = 
    vec![
        Sequence {
            nome : String::from("hg18.chr7"),
            seq : vec![65, 65, 65, 45, 71, 71, 71, 65, 65, 84, 71, 84, 84, 65, 65, 67, 67, 65, 65, 65, 84, 71, 65, 45, 45, 45, 65, 84, 84, 71, 84, 67, 84, 67, 84, 84, 65, 67, 71, 71, 84, 71],
        },
        Sequence{
            nome : String::from("panTro1.chr6"),
            seq : vec![65, 65, 65, 45, 71, 71, 71, 65, 65, 84, 71, 84, 84, 65, 65, 67, 67, 65, 65, 65, 84, 71, 65, 45, 45, 45, 65, 84, 84, 71, 84, 67, 84, 67, 84, 84, 65, 67, 71, 71, 84, 71],
        },
        Sequence {
            nome : String::from("baboon"),
            seq :  vec![65, 65, 65, 45, 71, 71, 71, 65, 65, 84, 71, 84, 84, 65, 65, 67, 67, 65, 65, 65, 84, 71, 65, 45, 45, 45, 71, 84, 84, 71, 84, 67, 84, 67, 84, 84, 65, 84, 71, 71, 84, 71],
        },
        Sequence {
            nome : String::from("mm4.chr6"),
            seq : vec![45, 65, 65, 84, 71, 71, 71, 65, 65, 84, 71, 84, 84, 65, 65, 71, 67, 65, 65, 65, 67, 71, 65, 45, 45, 45, 65, 84, 84, 71, 84, 67, 84, 67, 84, 67, 65, 71, 84, 71, 84, 71],
        },
        Sequence {
            nome :  String::from("rn3.chr4"),
            seq : vec![45, 65, 65, 45, 71, 71, 71, 71, 65, 84, 71, 67, 84, 65, 65, 71, 67, 67, 65, 65, 84, 71, 65, 71, 84, 84, 71, 84, 84, 71, 84, 67, 84, 67, 84, 67, 65, 65, 84, 71, 84, 71],
        }
    ];
    assert_eq!(al.0, expected);
}

#[test]
fn parse_file_one_alignment_block_with_info_line() {
    let al = Alignment::new("./dataset/input_2.maf").unwrap();
    let expected : Vec<Sequence> = 
    vec![
        Sequence {
            nome : String::from("hg16.chr7"),
            seq : vec![103, 99, 97, 103, 99, 116, 103, 97, 97, 97, 97, 99, 97],
        },
        Sequence{
            nome : String::from("panTro1.chr6"),
            seq : vec![103, 99, 97, 103, 99, 116, 103, 97, 97, 97, 97, 99, 97],
        },
        Sequence {
            nome : String::from("baboon"),
            seq :  vec![103, 99, 97, 103, 99, 116, 103, 97, 97, 97, 97, 99, 97],
        }
    ];
    assert_eq!(al.0, expected);
}

