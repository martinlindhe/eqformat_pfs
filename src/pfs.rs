use std::{fmt, str};
use std::path::Path;

use inflate::inflate_bytes_zlib;

#[cfg(test)]
#[path = "pfs_test.rs"]
mod s3d_test;

const DEBUG: bool = false;

/// PFSArchive describes the content of an PFS archive.
pub struct PFSArchive {
    pub basename: String,
    pub files: Vec<PFSFileEntry>,
}

impl PFSArchive {
    pub fn new(basename: &str) -> Self {
        Self {
            files: Vec::new(),
            basename: String::from(basename),
        }
    }

    /// parses file as a PFSArchive
    pub fn from_file(filename: &str) -> Result<Self, ParseError> {
        let data = read_binary(filename).unwrap();

        match Self::from_u8(&data) {
            Ok(mut pfs) => {
                pfs.basename = Path::new(filename).file_stem().unwrap().to_str().unwrap().to_string();
                Ok(pfs)
            }
            Err(e) => Err(e),
        }
    }

    /// parses [u8] as a PFSArchive
    pub fn from_u8(data: &[u8]) -> Result<Self, ParseError> {
        let content_offset = read_u32(&data, 0) as usize;
        let magic = read_u32(&data, 4);
        if magic != 0x20534650 {
            return Err(ParseError{message: "not a s3d".to_string()});
        }

        if DEBUG {
            println!("content_offset: {:08X}", content_offset);
        }

        let count = read_u32(&data, content_offset as usize) as usize;
        if DEBUG {
            println!("count {}", count);
        }

        let mut archive = Self {
            files: Vec::new(),
            basename: String::new(),
        };
        let mut dir_data = Vec::new();

        for i in 0..count {
            let cursor = content_offset as usize + 4 + (i * 12);
            let crc  = read_u32(&data, cursor);
            let offset = read_u32(&data, cursor + 4);
            let size = read_u32(&data, cursor + 8) as usize;

            if DEBUG {
                println!("s3d file {}: offset {:08X}, size {:08X}, crc {:08X}", i, offset, size, crc);
            }

            let mut file_entry = PFSFileEntry {
                data: Vec::new(),
                name: String::new(),
                crc,
                offset,
            };

            let mut read_cursor = offset as usize;

            while file_entry.data.len() < size {
                let compressed_len = read_u32(&data, read_cursor) as usize;
                read_cursor += 4;
                let expanded_len = read_u32(&data, read_cursor) as usize;
                read_cursor += 4;

                let expanded = inflate_bytes_zlib(&data[read_cursor..read_cursor+compressed_len]).unwrap();

                if expanded.len() != expanded_len {
                    return Err(ParseError{message: "zlib decompress failed".to_string()});
                }

                file_entry.data.extend(expanded);
                read_cursor += compressed_len;
            }

            if crc == 0x61580AC9 {
                dir_data.extend(file_entry.data);
            } else {
                archive.files.push(file_entry);
            }
        }

        if dir_data.is_empty() {
            return Err(ParseError{message: "no directory entry found".to_string()});
        }

        let mut dir_cursor = 0;
        let dirlen = read_u32(&dir_data, dir_cursor) as usize;
        dir_cursor += 4;
        if dirlen != archive.files.len() {
            return Err(ParseError{message: "directory does not match file length".to_string()});
        }

        // The list of filenames will only match the chunks if the chunk list is sorted by offset ascending
        archive.files.sort_by(|a, b| a.offset.cmp(&b.offset));

        for f in &mut archive.files {
            let filename_len = read_u32(&dir_data, dir_cursor) as usize;
            dir_cursor += 4;

            let buf = &dir_data[dir_cursor..dir_cursor + filename_len - 1];
            let filename = match str::from_utf8(buf) {
                Ok(v) => v,
                Err(e) => return Err(ParseError{message: format!("invalid utf-8 sequence: {}", e)}),
            };
            dir_cursor += filename_len;
            f.name = String::from(filename);

            if DEBUG {
                println!("s3d file: offset {:08X}, crc {:08X}, name = {}", f.offset, f.crc, f.name);
            }
        }

        Ok(archive)
    }

    /// Returns file entry by name
    pub fn find(&self, name: &str) -> Option<&PFSFileEntry> {
        for f in &self.files {
            if f.name == name {
                return Some(f);
            }
        }
        None
    }

    /// Returns the default wld, named infile-without-ext.wld.
    /// This function is only usable while working with .s3d archives
    pub fn default_wld(&self) -> Option<&PFSFileEntry> {
        if self.basename.is_empty() {
            return None;
        }
        let expected = format!("{}.wld", self.basename);
        for f in &self.files {
            if f.name == expected {
                return Some(f);
            }
        }
        None
    }
}

pub struct PFSFileEntry {
    /// uncompressed data
    pub data: Vec<u8>,
    pub crc: u32,
    pub offset: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Parses a PFS archive
#[deprecated(note = "Please use PFSArchive::from_file() instead")]
pub fn parse_pfs(filename: &str) -> Result<PFSArchive, ParseError> {
    PFSArchive::from_file(filename)
}

fn read_u32(data: &[u8], offset: usize) -> u32 {
    let mut bytes = [0; 4];
    bytes.copy_from_slice(&data[offset..offset+4]);
    u32::from_le_bytes(bytes)
}

fn read_binary(path: &str) -> Result<Vec<u8>, std::io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut buffer: Vec<u8> = Vec::new();

    let mut f = match File::open(path) {
        Ok(x) => x,
        Err(why) => return Err(why),
    };

    match f.read_to_end(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(why) => Err(why),
    }
}
