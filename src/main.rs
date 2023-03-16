use clap::Parser;

use crate::network::*;


mod network;
mod networkmanager;
mod qr;


const INDENT_QR: usize = 2;
const INDENT_CONTENT: usize = 4;


/// View and share NetworkManager wifi connections via QR code or via PSK
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Subcommand,
}

#[derive(Clone, Debug, PartialEq, clap::Subcommand)]
enum Subcommand {
    /// Abort current timer
    List {
        /// Show secrets
        #[clap(short, long)]
        secrets: bool,
    },

    /// Show details for the specified wifi
    Print {
        /// Name of the wifi profile you want to print
        name: String,
    },
}


fn list_wifis(secrets: bool) -> Result<(), String> {
    let wifis = WifiNetwork::nm_wifis()?;
    for (name, wifi) in wifis {
        if secrets {
            println!("{}:\n\tSSID: {}\n\tPASS: {}\n", name, wifi.ssid(), wifi.passkey());
        } else {
            println!("{}", name);
        }
    }

    Ok(())
}

fn print_wifi(name: &str) -> Result<(), String> {
    let wifis = WifiNetwork::nm_wifis()?;

    if let Some((_name, wifi)) = wifis.iter().find(|(n, _)| n == name) {
        let qr_code = wifi.qr_code()?;
        println!("\n{:indent$}WIFI NETWORK '{}':\n", "", name.to_uppercase(), indent=INDENT_CONTENT);

        for line in qr_code.to_ascii().lines() {
            println!("{:indent$}{}", "", line, indent=INDENT_QR);
        }

        println!("\n");

        println!("{:indent$}SSID: {}", "", wifi.ssid(), indent=INDENT_CONTENT);
        println!("{:indent$}PASS: {}", "", wifi.passkey(), indent=INDENT_CONTENT);
        Ok(())
    } else {
        Err(format!("unable to find wifi '{}'", name))
    }
}

fn print_qr(name: &str) -> Result<(), String> {
    let wifis = WifiNetwork::nm_wifis()?;

    if let Some((name, wifi)) = wifis.iter().find(|(n, _)| n == name) {
        let qr_code = wifi.qr_code()?;
        println!("QR-Code for '{}':\n{}", name, qr_code.to_ascii());
        Ok(())
    } else {
        Err(format!("unable to find wifi '{}'", name))
    }
}

fn main() {
    let args = Args::parse();

    let result = match args.command {
        Subcommand::List { secrets } => list_wifis(secrets),
        Subcommand::Print { name } => print_wifi(&name),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}