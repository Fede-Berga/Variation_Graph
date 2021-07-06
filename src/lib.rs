/// Parser Maf file
pub mod alignment_parser;

/// Variation Graph Builder
pub mod variation_graph;

/// Divide the Alignment in the correct way
pub mod partitioner;

/// Helper for Maf parsing
pub mod multiple_alignment_format;

/// INDEL character
pub const INDEL : u8 = '-' as u8;