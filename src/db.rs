use crate::errors::*;
use std::error::Error;

pub struct DB<E: Engine> {
    engine: E,
}

pub trait Engine {

}

impl<E: Engine> DB<E> {
    pub fn write(&mut self, options: &WriteOptions, batch: &WriteBatch) -> Result<(), Error> {}

    pub fn put(&mut self, options: &WriteOptions, key: &[u8], value: &[u8]) -> Result<(), Error> {}

    pub fn get(&self, options: &ReadOptions, key: &[u8]) -> Result<(), Error> {}
}

pub struct ReadOptions {}

pub struct WriteOptions {}

pub struct WriteBatch {}