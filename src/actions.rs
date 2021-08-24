use rocksdb::{DBWithThreadMode, MultiThreaded};

pub fn get_link_by_key(db: &DBWithThreadMode<MultiThreaded>, key: u64) -> Option<String> {
    let result = db.get(key.to_be_bytes());

    match result {
        Ok(Some(value)) => match String::from_utf8(value) {
            Ok(value) => Some(value),
            Err(e) => panic!("{}", e),
        },
        Ok(None) => None,
        Err(_) => None,
    }
}

pub fn insert_link(
    db: &DBWithThreadMode<MultiThreaded>,
    link: &str,
) -> Result<u64, rocksdb::Error> {
    let key = db.latest_sequence_number() + 1;

    db.put(key.to_be_bytes(), link).map(|()| key)
}
