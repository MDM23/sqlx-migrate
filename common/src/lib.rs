use fs::DirEntry;
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::{quote, TokenStreamExt};
use regex::Regex;
use sha2::{Digest, Sha256};
use std::convert::TryFrom;
use std::fs;
use thiserror::Error;

lazy_static! {
    static ref FILENAME_REGEX: Regex =
        Regex::new(r"^(?P<version>[0-9]+)_(?P<name>[a-z_]+)\.sql$").unwrap();
}

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("Filename is invalid")]
    FilenameError,

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct Migration {
    pub checksum: String,
    pub name: String,
    pub sql: String,
    pub version: i64,
}

impl TryFrom<DirEntry> for Migration {
    type Error = MigrationError;

    fn try_from(entry: DirEntry) -> Result<Self, Self::Error> {
        let file_name_os = entry.file_name();
        let file_name = file_name_os.to_str().ok_or(MigrationError::FilenameError)?;

        let cap = FILENAME_REGEX
            .captures(file_name)
            .ok_or(MigrationError::FilenameError)?;

        let name = cap
            .name("name")
            .map(|name| name.as_str())
            .ok_or(MigrationError::FilenameError)?
            .to_owned();

        let version = cap
            .name("version")
            .map(|version| version.as_str())
            .ok_or(MigrationError::FilenameError)?
            .parse()?;

        let sql = fs::read_to_string(&entry.path())?;
        let checksum = format!("{:x}", Sha256::digest(sql.as_bytes()));

        Ok(Self {
            checksum,
            name,
            sql,
            version,
        })
    }
}

impl ToTokens for Migration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Migration {
            checksum,
            name,
            sql,
            version,
        } = &self;

        let ts = quote! {
            sqlx_migrate::Migration {
                checksum: String::from(#checksum),
                name: String::from(#name),
                sql: String::from(#sql),
                version: #version,
            }
        };

        tokens.append_all(ts.into_iter());
    }
}

pub struct Migrator {
    pub migrations: Vec<Migration>,
}

impl Migrator {
    pub fn new(migrations: Vec<Migration>) -> Self {
        Migrator { migrations }
    }
}
