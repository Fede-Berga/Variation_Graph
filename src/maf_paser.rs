use std::fs;
use std::io;
use multiple_alignment_format::MAFItem;
use multiple_alignment_format::MAFBlock;
use multiple_alignment_format::parser::next_maf_item;
use std::fmt;

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
        writeln!(f, "[")?;
        for seq in self.0.iter() {
            writeln!(f, "{}", seq)?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : [", self.name)?;
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
    /// Returns the first mutiple alignment contained in 'file_name'
    /// 
    /// # Arguments
    /// 
    /// * 'file_name' - File from which the alignment block is fetched
    pub fn new(file_name : &str) -> Result<Alignment, &'static str> {
        let contents = match Alignment::get_file_content(file_name) {
            Err(_) => return Err("Error in file reading"),
            Ok(result) => result,
        };
    
        let alignment_block = match Alignment::get_block(contents) {
            Err(error) => return Err(error),
            Ok(result) => result,
        };
        Ok(Alignment::get_alignments(alignment_block))
    }

    /// Returns the content of 'file_name'
    /// 
    /// # Arguments
    /// 
    /// * 'file_name' - File from which the content is fetched
    fn get_file_content(file_name : & str) -> io::Result<String> {
        fs::read_to_string(file_name)
    }
    
    /// Returns the first alignment block contained in 'maf_contents'.
    /// Returns an error if there is no alignment block in 'maf_contents'.
    /// 
    /// # Arguments
    /// 
    /// * 'maf_contents' - Content of a MAf file
    fn get_block(maf_contents : String) -> Result<MAFBlock, &'static str> {
        for line in maf_contents.lines() {
            let i = maf_contents.find(line).unwrap();
            if let Ok(item) = next_maf_item(&mut maf_contents[i..].trim().as_bytes()) {
                if let MAFItem::Block(block) = item {
                    return Ok(block);
                }
            }
        }
        
        Err("Alignment block not found")
    }
    
    /// Returns an [Alignment] containing, for each sequence, 
    /// the name and the relative aligned sequence
    /// 
    ///  # Arguments
    /// 
    /// * 'block' - Block containing the alignment
    fn get_alignments(block : MAFBlock) -> Alignment{
        let alignment = block.aligned_entries()
            .map(|aligned_entry| {
                Sequence {
                    name : aligned_entry.seq.clone(),
                    seq : aligned_entry.alignment.clone(),
                }
            })
            .collect();
        Alignment(alignment)
    }
}

