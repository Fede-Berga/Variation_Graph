use std::fs;
use std::io;
use multiple_alignment_format::MAFItem;
use multiple_alignment_format::MAFBlock;
use multiple_alignment_format::parser::next_maf_item;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Alignment(pub Vec<Sequence>);

#[derive(Debug, PartialEq, Eq)]
pub struct Sequence {
    pub nome : String,
    pub seq : Vec<u8>,
}

impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[")?;
        for seq in self.0.iter() {
            writeln!(f, "{}", seq)?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : [", self.nome)?;
        for ch in self.seq.iter() {
            write!(f, "{}", *ch as char)?;
            if ch != self.seq.iter().last().unwrap() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl Alignment {
    pub fn new(file_name : &str) -> Result<Alignment, &'static str> {
        let contents = match Alignment::get_file_content(file_name) {
            Err(_) => return Err("Errore nella lettura del file"),
            Ok(result) => result,
        };
    
        let alignment_block = match Alignment::get_block(contents) {
            Err(error) => return Err(error),
            Ok(result) => result,
        };
        Ok(Alignment::get_alignments(alignment_block))
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
        let alignment = block.aligned_entries()
            .map(|aligned_entry| {
                Sequence {
                    nome : aligned_entry.seq.clone(),
                    seq : aligned_entry.alignment.clone(),
                }
            })
            .collect();
        Alignment(alignment)
    }
}

