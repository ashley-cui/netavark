use regex::Regex;
use std::collections::HashMap;
use crate::network::constants;
use crate::network::types::{Network, Subnet, SubnetPool};
use crate::{
    error::{NetavarkError, NetavarkResult},
};
use ipnet::IpNet;

pub fn validate_name_id(name: &String, id: &String, existing_networks: &HashMap<String, String>) -> NetavarkResult<()>{
    if name == "" {
        return Err(NetavarkError::msg("Network name must be supplied"));
    }
    let name_regex = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_.-]*$").unwrap();
    if !name_regex.is_match(&name) {
        return Err(NetavarkError::msg("Invalid characters in network name"));
    }
    if existing_networks.contains_key(name) {
        return Err(NetavarkError::msg(format!("Network name {} already used", name)))
    }

    if id == "" {
        return Err(NetavarkError::msg("Network id must be supplied"));
    }
    Ok(())
}

// validate_interface_name validates the interface name based on the following rules:
// 1. The name must be less than MaxInterfaceNameLength characters
// 2. The name must not be "." or ".."
// 3. The name must not contain / or : or any whitespace characters
// ref to https://github.com/torvalds/linux/blob/81e4f8d68c66da301bb881862735bd74c6241a19/include/uapi/linux/if.h#L33C18-L33C20
pub fn validate_interface_name(if_name: &String) -> NetavarkResult<()>{
    if if_name.len() > constants::MAX_INTERFACE_NAME_LEN{
        return Err(NetavarkError::msg(format!("Interface name is too long: interface names must be {} characters or less: {}", constants::MAX_INTERFACE_NAME_LEN, if_name)))
    }
    if if_name == "" || if_name == ".."{
        return Err(NetavarkError::msg("Interface name cannot be . or .."))
    }

    if let Some(bad) = if_name
        .chars()
        .find(|&c| c == '/' || c == ':' || c.is_whitespace())
    {
         return Err(NetavarkError::msg(format!("Interface name cannot contain\"{}\"", bad)))
    }


    Ok(())
}

pub fn validate_ipam_driver(ipam_opts: &Option<HashMap<String, String>>, subnets: &Option<Vec<Subnet>>)-> NetavarkResult<()> {
    if ipam_opts.is_none() {
        return Ok(())
    }

    let ipam_driver = ipam_opts.as_ref().unwrap().get("driver");
    match ipam_driver {
        None => Ok(()),
        Some(driver) if driver == constants::IPAM_HOSTLOCAL || driver == constants::IPAM_DHCP => Ok(()),
        Some(driver) if driver == constants::IPAM_NONE => {
            let subnetlen = subnets.as_ref().map(|v| v.len()).unwrap_or(0);
            if subnetlen > 0 {
                return Err(NetavarkError::msg(format!("None ipam driver is set but subnets are given")))
            }
            Ok(())
        }
        Some(driver) => Err(NetavarkError::msg(format!("Unsupported ipam driver: {}", driver))),
    }
}


pub fn setup_bridge_options(network: &mut Network, default: bool) -> NetavarkResult<()> {
    if network.options.is_none(){
        return Ok(())
    }


    let mut check_used = true;
    let mut check_bridge_conflict = true;
    let opts = network.options.clone().unwrap_or_default();
    for (key, value) in opts {
        match key.as_str(){
            constants::OPTION_MTU => {parse_mtu(&value)?;},
            constants::OPTION_VLAN => {
                parse_vlan(&value)?;
                check_used = false;
                check_bridge_conflict = false
            },
            constants::OPTION_ISOLATE => {
                let iso = parse_isolate(&value)?;
                network.options.as_mut().unwrap().insert(key.clone(), iso);
            },
            constants::OPTION_METRIC => {parse_metric(&value)?;},
            constants::OPTION_NO_DEFAULT_ROUTE => {
                value.parse::<bool>()
                    .map_err(|_| NetavarkError::msg(format!("invalid no_default_route value: {}", value)))?;
                },
            constants::OPTION_VRF =>{
                if value.len() == 0 {
                    return Err(NetavarkError::msg(format!("invalid vrf name: {}", value)));
                }
            },
            constants::OPTION_MODE => {
                check_used = false;
                check_bridge_conflict = false},

            _ => return Err(NetavarkError::msg(format!("Unsupported bridge option driver: {}", key))),
        }

    }

    Ok(())
}

fn create_bridge(network: &mut Network, used: &Used, check_conflict: bool) -> NetavarkResult<())>{
    if network.network_interface.is_some()
            if used.interfaces.contains(network.network_interface) {
                return Err(NetavarkError::msg(format!("bridge name is already in use", network.network_interface)))
            }

        }
    }
    Ok(())
}

fn parse_mtu(mtu: &String) -> NetavarkResult<u32>{
    if mtu == ""{
        return Ok(0)
    }
    match mtu.parse::<u32>() {
        Ok(n) => Ok(n),
        Err(e) => return Err(NetavarkError::msg(format!("Failed to parse mtu: {}", e))),
    }
}

fn parse_vlan(vlan: &String) -> NetavarkResult<u32>{
    if vlan == ""{
        return Ok(0)
    }
    match vlan.parse::<u32>() {
        Ok(n) => {
            if n>4094 {
                return Err(NetavarkError::msg(format!("vlan id {} must be between 0 and 4094", n)))
            }
            Ok(n)
        },
        Err(e) => return Err(NetavarkError::msg(format!("Failed to parse: {}", e))),
    }
}

fn parse_isolate(isolate: &String) -> NetavarkResult<String>{
    match isolate.as_str() {
        "" => return Ok(String::from("false")),
        "strict" | "true" | "false" => return Ok(isolate.clone()),
        _ => return Err(NetavarkError::msg(format!("failed to parse isolate option {}", isolate)))

    }
}

fn parse_metric(metric: &String) -> NetavarkResult<u32>{
    match metric.parse::<u32>() {
        Ok(n) => {Ok(n)},
        Err(e) => return Err(NetavarkError::msg(format!("Failed to parse metric: {}", e))),
    }
}
