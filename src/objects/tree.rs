use anyhow::{anyhow, Result};
use thiserror::Error;

use super::{tree_row::TreeRow, GitObject};

#[derive(Debug)]
pub struct FileTree {
    size: usize,
    pub rows: Vec<TreeRow>
}



#[derive(Error, Debug)]
pub enum ObjectParseError {
    #[error("First bytes don't represents tree")]
    NotTree,
}

const HASH_LENGTH: u8 = 20;

impl GitObject for FileTree {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut splitted = bytes.splitn(2, |b| *b == 0);
        let header = splitted.next().ok_or(anyhow!("Empty file"))?;

        let header_string = header.iter().map(|b| *b as char).collect::<String>();
        let mut splitted_header = header_string.split(32u8 as char); // split by space

        let object_type = splitted_header
            .next()
            .ok_or_else(|| anyhow!("Parsing error"))?;
        if object_type != "tree" {
            return Err(anyhow!(ObjectParseError::NotTree));
        }

        let object_size = splitted_header
            .next()
            .ok_or_else(|| anyhow!("Parsing error"))?
            .parse::<usize>()?;

        let mut rows = vec![];

        let rest = splitted.next().ok_or(anyhow!("Empty file"))?;
        let mut iter = rest.iter();

        loop {
            let next_byte = iter.next();
            if next_byte.is_none() {
                break;
            };
            let first_byte = next_byte.unwrap();

            let mut mode = vec![*first_byte];
            loop {
                let byte = *iter.next().unwrap();
                if byte == 32 {
                    break
                }

                mode.push(byte)
            }

            let mut name = vec![];
            loop {
                let byte = *iter.next().unwrap();
                if byte == 0 {
                    break;
                };

                name.push(byte);
            }

            let hash = (0..HASH_LENGTH)
                .map(|_| *iter.next().unwrap())
                .collect::<Vec<_>>();

            rows.push(TreeRow::from_bytes(&mode, &name, &hash).unwrap());
        }

        Ok(Self { size: object_size, rows })
    }
}
