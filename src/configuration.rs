use std::env;

use askama::Template;
use harsh::Harsh;
use once_cell::sync::Lazy;
use rocksdb::{BlockBasedOptions, DBWithThreadMode, MultiThreaded, Options};

use crate::message::Index;

/// Base 35, note that the `l` is skipped.
const ALPHABET: &str = "abcdefghijkmnopqrstuvwxyz0123456789";

pub static VANITY_DOMAIN: Lazy<String> =
    Lazy::new(|| env::var("VANITY_DOMAIN").expect("Unable to find VANITY_DOMAIN from env"));

pub static ASSETS_URL: Lazy<String> = Lazy::new(|| {
    env::var("ASSETS_URL").unwrap_or_else(|_| format!("{}/assets", *VANITY_DOMAIN))
});

pub static ADDRESS: Lazy<String> = Lazy::new(|| {
    format!(
        "{}:{}",
        env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        env::var("PORT").unwrap_or_else(|_| "80".to_string())
    )
});

pub static DATABASE: Lazy<DBWithThreadMode<MultiThreaded>> = Lazy::new(|| {
    let path: &str = &env::var("DATABASE_PATH").expect("Unable to find DATABASE_PATH from env");

    // We take the suggested defaults from RocksDB Wiki
    // https://github.com/facebook/rocksdb/wiki/Setup-Options-and-Basic-Tuning
    let mut options = Options::default();
    options.create_if_missing(true);
    options.set_max_background_jobs(4);
    options.set_bytes_per_sync(1 << 20);

    let mut table_options = BlockBasedOptions::default();
    table_options.set_block_size(16 * 1024);
    table_options.set_cache_index_and_filter_blocks(true);
    table_options.set_pin_l0_filter_and_index_blocks_in_cache(true);
    table_options.set_format_version(5);

    options.set_block_based_table_factory(&table_options);

    rocksdb::DB::open(&options, path).expect("Unable to open database")
});

pub static HASHER: Lazy<Harsh> = Lazy::new(|| {
    Harsh::builder()
        .alphabet(ALPHABET)
        .salt(&*env::var("SALT").expect("Unable to find SALT from env"))
        .length(3)
        .build()
        .expect("Unable to construct hasher")
});

pub static BLANK_INDEX_TEMPLATE: Lazy<String> =
    Lazy::new(|| Index::default().render().expect("Failed to render index template"));

pub fn is_accepted_uri(uri: &str) -> bool {
    match uri {
        "http" | "https" => true,
        "ftp" | "sftp" => true,
        "spotify" => true,
        "steam" => true,
        "git" => true,
        "magnet" => true,
        _ => false,
    }
}
