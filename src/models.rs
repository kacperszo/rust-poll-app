use rocket::FromForm;
use std::sync::Mutex;

#[derive(FromForm, Debug)]
pub struct PoolDTO {
    pub(crate) title: String,
    pub(crate) options: Vec<String>
}

#[derive(Debug, FromForm)]
pub struct VoteDTO {
    pub(crate) option: usize,
}


#[derive(Debug)]
pub struct Pool{
    pub(crate) id: usize,
    pub(crate) title: String,
    pub(crate) options: Vec<String>,
    pub(crate) votes: Vec<usize>

}

impl Pool {
    pub fn new(id: usize, title: String, options: Vec<String>) -> Self {
        let votes = vec![0; options.len()];
        Self { id, title, options, votes }
    }
}

pub type Pools = Mutex<Vec<Pool>>;