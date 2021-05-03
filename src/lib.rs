//Parser Maf file
pub mod alignment_parser;

//Variation Graph Builder
pub mod variation_graph;

//Divide the Alignment in the correct way
pub mod partitioner;

pub mod multiple_alignment_format;

pub const INDEL : u8 = '-' as u8;