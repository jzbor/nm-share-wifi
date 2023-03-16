use std::fs;

use crate::qr;
use crate::networkmanager;


#[derive(PartialEq,Debug,Clone)]
pub struct WifiNetwork {
    /// SSID of the wifi network
    ssid: String,

    /// PSK to authenticate to the network
    passkey: String,

    /// Method of authentication
    auth_method: AuthMethod,
}

#[derive(PartialEq,Debug,Clone, Copy)]
pub enum AuthMethod {
    WPA
}

impl WifiNetwork {
    pub fn new(ssid: String, passkey: String, auth_method: AuthMethod) -> Self {
        WifiNetwork { ssid, passkey, auth_method }
    }

    pub fn from_nm_config(config: &str) -> Result<Self, String> {
        let nm_wifi: networkmanager::NMWifiNetwork = serde_ini::from_str(config)
            .map_err(|e| format!("unable to read nm config ({})", e))?;
        nm_wifi.try_into()
    }

    pub fn nm_wifis() -> Result <Vec<(String, Self)>, String> {
        let system_connections = networkmanager::system_connections()?;
        let wifis: Vec<(String, WifiNetwork)> = system_connections.into_iter().map(|(n, p)|
            fs::read_to_string(p)
                .map(|c| WifiNetwork::from_nm_config(&c).map(|w| (n, w)))
        ).flatten().flatten().collect();

        Ok(wifis)
    }

    pub fn auth_method(&self) -> AuthMethod {
        return self.auth_method;
    }

    pub fn passkey(&self) -> &str {
        return &self.passkey;
    }

    pub fn ssid(&self) -> &str {
        return &self.ssid;
    }

    pub fn qr_code(&self) -> Result<qr::QRCode, String> {
        let auth = match self.auth_method {
            AuthMethod::WPA => "WPA",
        };

        // template: "WIFI:S:<SSID>;T:<WEP|WPA|blank>;P:<PASSWORD>;H:<true|false|blank>;;"
        let text = format!("WIFI:S:{};T:{};P:{};;", self.ssid, auth, self.passkey);

        qr::QRCode::from_str(&text)
    }
}
