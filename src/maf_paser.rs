use std::fs;
use std::io;
use multiple_alignment_format::MAFItem;
use multiple_alignment_format::MAFBlock;
use multiple_alignment_format::parser::next_maf_item;
use multiple_alignment_format::MAFBlockEntry;
use std::process;
use std::fmt;

#[derive(Debug)]
pub struct Alignment(pub Vec<Vec<u8>>);

impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[")?;
        for seq in self.0.iter() {
            let as_char : Vec<char> = seq.iter().map(|byte| *byte as char).collect();
            writeln!(f, "{:?}", as_char)?;
        }
        write!(f, "]")
    }
}

impl Alignment {
    pub fn new(file_name : &str) -> Alignment {
        let contents = match Alignment::get_file_content(file_name) {
            Err(error) => {
                println!("{:?}", error);
                process::exit(1);
            }
            Ok(result) => result,
        };
    
        let alignment_block = match Alignment::get_block(contents) {
            Err(error) => {
                println!("{:?}", error);
                process::exit(1);
            }
            Ok(result) => result,
        };
        Alignment::get_alignments(alignment_block)
    }

    fn get_file_content(file_name : & str) -> io::Result<String> {
        fs::read_to_string(file_name)
    }
    
    fn get_block(maf_contents : String) -> Result<MAFBlock, &'static str> {
        for line in maf_contents.lines() {
            let i = maf_contents.find(line).unwrap();
            if let Ok(item) = next_maf_item(&mut maf_contents[i..].trim().as_bytes()) {
                if let MAFItem::Block(block) = item {
                    return Ok(block);
                }
            }
        }
        
        Err("Blocco di allineamento non trovato")
    }
    
    fn get_alignments(block : MAFBlock) -> Alignment{
        let mut alignment = Vec::new();
    
        for entry in block.entries {
            match entry {
                MAFBlockEntry::AlignedEntry(aligned_entry) => alignment.push(aligned_entry.alignment),
                _ => {}
            }
        }
    
        Alignment(alignment)
    }
}

