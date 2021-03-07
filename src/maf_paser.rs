use std::fs;
use std::io;
use multiple_alignment_format::MAFItem;
use multiple_alignment_format::MAFBlock;
use multiple_alignment_format::parser::next_maf_item;
use std::fmt;

/// Rappresenta un blocco di allineamento 
/// in un file .maf
#[derive(Debug, PartialEq, Eq)]
pub struct Alignment(pub Vec<Sequence>);

/// Rappresenta una sequenza di allineamento.
#[derive(Debug, PartialEq, Eq)]
pub struct Sequence {
    /// Nome della sequenza
    pub nome : String,
    /// Sequenza allineata
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
    /// Restituisce un Alignment contenuto nel file 'file_name'
    /// 
    /// # Arguments
    /// 
    /// * 'file_name' - Nome del file da cui prelevare l'allineamento
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

    /// Restituisce il contenuto del file 'file_name'
    /// 
    /// # Arguments
    /// 
    /// * 'file_name' - Nome del file da cui prelevare il contenuto
    fn get_file_content(file_name : & str) -> io::Result<String> {
        fs::read_to_string(file_name)
    }
    
    /// Restituisce il primo blocco di allineamento nel file .maf.
    /// Restituisce un errore nel caso non venga trovato nessun blocco.
    /// 
    /// # Arguments
    /// 
    /// * 'maf_contents' - Contenuto del file .maf
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
    
    /// Restituisce un Alignment contentente i nomi delle sequenze e la relativa
    /// sequenza allineata.
    /// 
    ///  # Arguments
    /// 
    /// * 'block' - Blocco contenente l'allineamento
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

