use crate::errors::*;
use failure::Error;
use std::fs::File;

pub struct Key(Vec<u8>);
pub struct Value(Vec<u8>);

pub enum Operation {
    Put(Key, Value),
    Delete(Key),
}

pub struct DB {
    name: String,
    options: DBOptions,
}

impl DB {
    pub fn open(options: DBOptions, name: &str) -> Result<DB, Error> {
        // TODO:
        //   1. Read snapshot
        //   2. Replay WAL
        //   3. Create necessary files
        Ok(DB {
            name: name.to_owned(),
            options,
        })
    }

    pub fn write(&mut self, options: &WriteOptions, batch: Vec<Operation>) -> Result<(), Error> {
        panic!("not implemented")
    }

    pub fn put(&mut self, options: &WriteOptions, key: Key, value: Value) -> Result<(), Error> {
        self.write(options, vec![Operation::Put(key, value)])
    }

    pub fn delete(&mut self, options: &WriteOptions, key: Key) -> Result<(), Error> {
        self.write(options, vec![Operation::Delete(key)])
    }

    pub fn get(&self, options: &ReadOptions, key: Key) -> Result<(), Error> {
        panic!("not implemented")
    }
}

// TODO: Builder for DBOptions
pub struct DBOptions {
    // If true, the database will be created if it is missing.
    // Default: false
    create_if_missing: bool,

    // If true, an error is raised if the database already exists.
    // Default: false
    error_if_exists: bool,

    // Rdb will write up to this amount of bytes to a file before
    // switching to a new one.
    // Most clients should leave this parameter alone.  However if your
    // filesystem is more efficient with larger files, you could
    // consider increasing the value.  The downside will be longer
    // compactions and hence longer latency/performance hiccups.
    // Another reason to increase this parameter might be when you are
    // initially populating a large database.
    //
    // Default: 2MB
    max_file_size: u32,

    // Approximate size of user data packed per block.  Note that the
    // block size specified here corresponds to uncompressed data.  The
    // actual size of the unit read from disk may be smaller if
    // compression is enabled.  This parameter can be changed dynamically.
    //
    // Default: 4K
    block_size: u32,

    // Amount of data to build up in memory (backed by an unsorted log
    // on disk) before converting to a sorted on-disk file.
    //
    // Larger values increase performance, especially during bulk loads.
    // Up to two write buffers may be held in memory at the same time,
    // so you may wish to adjust this parameter to control memory usage.
    // Also, a larger write buffer will result in a longer recovery time
    // the next time the database is opened.
    //
    // Default: 4MB
    write_buffer_size: u32,
}

impl Default for DBOptions {
    fn default() -> DBOptions {
        DBOptions {
            create_if_missing: false,
            error_if_exists: false,
            max_file_size: 2<<20,
            block_size: 4096,
            write_buffer_size: 4<<20,
        }
    }
}

pub struct ReadOptions {}

pub struct WriteOptions {}