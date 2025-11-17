use crate::error::NetavarkResult;
use crate::network::{self};
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
        let network_options = network::types::NetworkCreate::load(input_file)?;
        println!("{:?}", network_options);

        let create = network::create::new_network(network_options);

        Ok(())
    }
}
