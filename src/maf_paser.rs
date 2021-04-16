extern crate bio;
use multiple_alignment_format::{
    MAFItem,
    MAFBlock,
    parser::next_maf_item,
};
use std::fmt;
use bio::io::fasta;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
pub enum ParserError {
    FileNotFound(String),
    AlignmentBlockNotFound(String),
    FastaParserError(String)
}
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Represents a multiple sequence alignment 
#[derive(Debug, PartialEq, Eq)]
pub struct Alignment(pub Vec<Sequence>);

/// Represents an aligned sequence
#[derive(Debug, PartialEq, Eq)]
pub struct Sequence {
    /// Sequence name
    pub name : String,
    /// Aligned Sequence
    pub seq : Vec<u8>,
}

impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for seq in self.sequences() {
            writeln!(f, "{}", seq)?;
        }
        write!(f, "")
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : [", self.name)?;
        for ch in self.seq.iter() {
            write!(f, "{} ", *ch as char)?;
        }
        write!(f, "]")
    }
}

impl Alignment {
    pub fn sequences(&self) -> std::slice::Iter<'_, Sequence> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn dump_on_file(&self, file_name : &str) {
        let max_name_len = self.sequences().map(|sequence| sequence.name.len()).max().unwrap();

        let path = Path::new(file_name);

        let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", path.display(), why),
                Ok(file) => file,
        };

        for sequence in self.sequences() {
            match file.write_all(sequence.name.as_bytes()) {
                Err(why) => panic!("couldn't write NAME to {}: {}", path.display(), why),
                Ok(_) => {}, //println!("successfully wrote to {}", path.display()),
            }

            match file.write_all(&vec![' ' as u8; max_name_len - sequence.name.len() + 1]) {
                Err(why) => panic!("couldn't write SPACES to {}: {}", path.display(), why),
                Ok(_) => {}, //println!("successfully wrote to {}", path.display()),
            }

            match file.write_all(&sequence.seq) {
                Err(why) => panic!("couldn't write SEQUENCE to {}: {}", path.display(), why),
                Ok(_) => {}, //println!("successfully wrote to {}", path.display()),
            }

            match file.write_all("\n".as_bytes()) {
                Err(why) => panic!("couldn't write ENDLINE to {}: {}", path.display(), why),
                Ok(_) => {}, //println!("successfully wrote to {}", path.display()),
            }
        }
    }
}

impl Sequence {
    pub fn len(&self) -> usize {
        self.seq.len()
    }
}

pub trait Parser {
    fn get_alignment(file_name : &str) -> Result<Alignment, ParserError>;
}

pub struct FastaParser;

impl Parser for FastaParser {
    fn get_alignment(file_name : &str) -> Result<Alignment, ParserError> {
        let reader = match fasta::Reader::from_file(file_name) {
            Ok(content) => content,
            Err(e) => return Err(ParserError::FileNotFound(format!("{:?}", e)))
        };

        let seqs : Vec<Sequence> = reader.records()
            .map(|result| {
                let record = result.expect("Error during fasta record parsing");

                let seq : Vec<u8>= record.seq().iter().map(|&byte| (byte as char).to_ascii_uppercase() as u8).collect();
                Sequence{name : record.id().to_string(), seq : seq}
            })
            .collect();
        
        Ok(Alignment(seqs))
    }
}

pub struct MafParser;

impl Parser for MafParser {

    fn get_alignment(file_name : &str) -> Result<Alignment, ParserError> {
        let contents = match fs::read_to_string(file_name) {
            Err(_) => return Err(ParserError::FileNotFound(String::from("Error in file reading"))),
            Ok(result) => result,
        };

        let alignment_block = MafParser::get_block(contents)?;

        Ok(MafParser::get_alignments(alignment_block))
    }
}

impl MafParser {

    fn get_block(maf_contents : String) -> Result<MAFBlock, ParserError> {
        for line in maf_contents.lines() {
            let i = maf_contents.find(line).unwrap();
            if let Ok(item) = next_maf_item(&mut maf_contents[i..].trim().as_bytes()) {
                if let MAFItem::Block(block) = item {
                    return Ok(block);
                }
            }
        }
        
        Err(ParserError::AlignmentBlockNotFound(String::from("Alignment block not found")))
    }

    fn get_alignments(block : MAFBlock) -> Alignment {
        let alignment = block.aligned_entries()
            .map(|aligned_entry| {
                let seq : Vec<_>= aligned_entry.alignment.iter().map(|&byte| (byte as char).to_ascii_uppercase() as u8).collect();
                Sequence {
                    name : aligned_entry.seq.clone(),
                    seq : seq,
                }
            })
            .collect();
        Alignment(alignment)
    }
}


