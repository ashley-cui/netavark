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

mod validate;
use validate::*;

impl types::NetworkCreateConfig {
    pub fn load(path: Option<OsString>) -> NetavarkResult<types::NetworkCreateConfig> {
        wrap!(Self::load_inner(path), "failed to load network create")
    }

    fn load_inner(path: Option<OsString>) -> Result<types::NetworkCreateConfig, io::Error> {
        let newnetworkcreate = match path {
            Some(path) => serde_json::from_reader(BufReader::new(File::open(path)?)),
            None => serde_json::from_reader(io::stdin()),
        }?;
        Ok(newnetworkcreate)
    }
}


// fn driver_opts()
pub fn new_network(create: types::NetworkCreateConfig) -> NetavarkResult<types::Network>{
    use crate::network::constants;
    let mut network = create.network;
    validate_name_id(&network.name, &network.id, &create.used.networks)?;
    validate_interface_name(&network.name)?;
    validate_ipam_driver(&network.ipam_options, &network.subnets)?;

    match network.driver.as_ref(){
        constants::DRIVER_BRIDGE => setup_bridge_options(&mut network, create.default)?,
        constants::DRIVER_IPVLAN | constants::DRIVER_MACVLAN => todo!(),
        _ =>  todo!()

    }

    Ok(network)
}

