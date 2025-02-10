use anyhow::Result;
use shared;

pub trait SessionOperations {
    fn do_something_in_database(&self, foo: u128) -> Result<i128>;
}

impl SessionOperations for shared::Session {
    fn do_something_in_database(&self, foo: u128) -> Result<i128> {
        Ok(foo as i128)
    }
}
