extern crate byteorder;

mod build;
mod parse;
mod crypto;

use std::fs::File;
use std::path::Path;

static DEBUG: bool = false;
static PFH5_PREAMBLE: u32 = 0x35484650;
static PFH4_PREAMBLE: u32 = 0x34484650;
static HAS_BIG_HEADER: u32              = 0b0000_0001_0000_0000;
static INDEX_ENCRYPTED: u32             = 0b0000_0000_1000_0000;
static HAS_INDEX_WITH_TIMESTAMPS: u32   = 0b0000_0000_0100_0000;
static CONTENT_ENCRYPTED: u32           = 0b0000_0000_0001_0000;

#[derive(Debug)]
pub enum PFHVersion {
    PFH5,
    PFH4
}

impl PFHVersion {
    pub(crate) fn get_preamble(&self) -> u32 {
        match *self {
            PFHVersion::PFH5 => PFH5_PREAMBLE,
            PFHVersion::PFH4 => PFH4_PREAMBLE
        }
    }
}

#[derive(Debug)]
pub struct PackFile {
    raw_data: Vec<u8>
}

#[derive(Debug)]
pub struct PackedFile {
    pub timestamp: Option<u32>,
    pub name: String,
    pub content: Vec<u8>
}

#[derive(Debug)]
pub enum ParsePackError {
    InvalidHeaderError,
    InvalidFileError
}

#[derive(Debug)]
pub enum BuildPackError {
    EmptyInputError,
    InputTooBigError,
    IOError
}

pub fn parse_pack<'a>(bytes: Vec<u8>) -> Result<::PackFile, ParsePackError> {
    parse::parse_pack(bytes)
}

pub fn build_pack_from_filesystem(input_directory: &Path, output_file: &mut File, version: PFHVersion, bitmask: u32) -> Result<(), BuildPackError> {
    build::build_pack_from_filesystem(input_directory, output_file, version, bitmask)
}

pub fn build_pack_from_memory(input: &Vec<PackedFile>, output_file: &mut File, version: PFHVersion, bitmask: u32) -> Result<(), BuildPackError> {
    build::build_pack_from_memory(input, output_file, version, bitmask)
}
