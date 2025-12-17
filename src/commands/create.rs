use crate::error::NetavarkResult;
use crate::network::{self, create_config};
use clap::Parser;
use std::ffi::OsString;


#[derive(Parser, Debug)]
pub struct Create {}

impl Create {
    pub fn exec(
        &self,
        input_file: Option<OsString>,
    ) -> NetavarkResult<()> {
        println!("cliiiiiiiii");
        println!("{:?}", input_file);
        let network_options = network::types::NetworkCreateConfig::load(input_file)?;
        println!("{:?}", network_options);

        let create = network::create_config::new_network(network_options)?;

        Ok(())
    }
}
