use crate::network::types;
use std::{
    ffi::OsString,
    fs::File,
    io::{self, BufReader},
};

use crate::{
    error::{NetavarkError, NetavarkResult},
    wrap,
};

impl types::NetworkCreate {
    pub fn load(path: Option<OsString>) -> NetavarkResult<types::NetworkCreate> {
        wrap!(Self::load_inner(path), "failed to load network create")
    }

    fn load_inner(path: Option<OsString>) -> Result<types::NetworkCreate, io::Error> {
        let newnetworkcreate = match path {
            Some(path) => serde_json::from_reader(BufReader::new(File::open(path)?)),
            None => serde_json::from_reader(io::stdin()),
        }?;
        Ok(newnetworkcreate)
    }
}

// fn driver_opts()
pub fn new_network(create: types::NetworkCreate) -> NetavarkResult<types::Network>{
 Ok(create.network)
}
