use std::env;

use harsh::Harsh;
use lazy_static::lazy_static;
use rocksdb::{BlockBasedOptions, DBWithThreadMode, MultiThreaded, Options};

extern crate rocksdb;

pub mod actions;
mod error;
pub mod message;

pub mod prelude {
    pub use crate::error::Error;
}

/// Base 35, note that the `l` is skipped.
pub const ALPHABET: &str = "abcdefghijkmnopqrstuvwxyz0123456789";

lazy_static! {
    static ref SALT: String = env::var("SALT").expect("Unable to find SALT from env");
    pub static ref VANITY_HOST: String =
        env::var("VANITY_HOST").expect("Unable to find VANITY_HOST from env");
    static ref HOST: String = env::var("HOST").expect("Unable to find HOST from env");
    static ref PORT: String = env::var("PORT").expect("Unable to find PORT from env");
    pub static ref ADDRESS: String = format!("{}:{}", *HOST, *PORT);
    static ref DATABASE_PATH: String =
        env::var("DATABASE_PATH").expect("Unable to find DATABASE_PATH from env");
    pub static ref DATABASE: DBWithThreadMode<MultiThreaded> = initialize(&*DATABASE_PATH);
    pub static ref HASHER: Harsh = Harsh::builder()
        .alphabet(ALPHABET)
        .salt(&*SALT.to_string())
        .length(3)
        .build()
        .expect("Unable to construct hasher");
}

pub fn initialize(path: &str) -> DBWithThreadMode<MultiThreaded> {
    // We take the suggested defaults from RocksDB Wiki
    // https://github.com/facebook/rocksdb/wiki/Setup-Options-and-Basic-Tuning

    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.set_max_background_jobs(4);
    opts.set_bytes_per_sync(1024 * 1024);

    let mut table_opts = BlockBasedOptions::default();
    table_opts.set_block_size(16 * 1024);
    table_opts.set_cache_index_and_filter_blocks(true);
    table_opts.set_pin_l0_filter_and_index_blocks_in_cache(true);
    table_opts.set_format_version(5);

    rocksdb::DB::open_default(path).expect("Unable to open database")
}

pub fn is_accepted_uri(uri: &str) -> bool {
    match uri {
        "http" | "https" => true,
        "ftp" | "sftp" | "ftps" => true,
        "spotify" => true,
        "steam" => true,
        "git" => true,
        "magnet" => true,
        _ => false,
    }
}
