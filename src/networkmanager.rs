use serde::{Serialize, Deserialize};
use std::path;
use std::fs;

use crate::network::AuthMethod;
use crate::network::WifiNetwork;


const NM_CONNECTIONS_PATH: &str = "/etc/NetworkManager/system-connections";
const NM_CONNECTION_EXT: &str = "nmconnection";


#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
#[serde(rename_all = "kebab-case")]
pub struct NMWifiNetwork {
    wifi: NMWifiSection,
    wifi_security: NMWifiSecuritySection,
}

#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
#[serde(rename_all = "kebab-case")]
pub struct NMWifiSection {
    ssid: String,
}

#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
#[serde(rename_all = "kebab-case")]
pub struct NMWifiSecuritySection {
    key_mgmt: String,
    psk: String,
}


impl TryFrom<NMWifiNetwork> for WifiNetwork {
    type Error = String;

    fn try_from(value: NMWifiNetwork) -> Result<Self, Self::Error> {
        let auth_method = match value.wifi_security.key_mgmt.as_str() {
            "wpa-psk" => AuthMethod::WPA,
            _ => return Err(format!("unknown authentication method '{}'", value.wifi_security.key_mgmt)),
        };

        let wifi = WifiNetwork::new(value.wifi.ssid, value.wifi_security.psk, auth_method);
        Ok(wifi)
    }
}


pub fn system_connections() -> Result<Vec<(String, path::PathBuf)>, String> {
    let directory = path::PathBuf::from(NM_CONNECTIONS_PATH);
    if !directory.exists() {
        return Err(format!("no network connections found - '{}' does not exist", NM_CONNECTIONS_PATH));
    } else if !directory.is_dir() {
        return Err(format!("no network connections found - '{}' is not a directory", NM_CONNECTIONS_PATH));
    }

    let get_name = |e: &fs::DirEntry| e.path().file_stem().unwrap().to_string_lossy().to_string();

    let connections = directory.read_dir().map_err(|e| format!("unable to find network connections ({})", e))?
        .flatten()  // quietly filter out errors
        .filter(|e| e.path().extension().and_then(|e| e.to_str()) == Some(NM_CONNECTION_EXT))
        .map(|e| (get_name(&e), e.path()))
        .collect();


    Ok(connections)
}
