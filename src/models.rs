use rocket::{FromForm};
use std::sync::{Arc, Mutex};

#[derive(FromForm, Debug)]
pub struct PoolDTO {
    pub(crate) title: String,
    pub(crate) options: Vec<String>
}


#[derive(Debug)]
pub struct Pool{
    pub(crate) id: usize,
    pub(crate) title: String,
    pub(crate) options: Vec<String>
}

impl Pool {
    pub fn new(id: usize, title: String, options: Vec<String>) -> Self {
        Self { id, title, options }
    }
}

pub type Pools = Arc<Mutex<Vec<Pool>>>;