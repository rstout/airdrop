use clap::Parser;
use std::path::PathBuf;
use std::fs;
use std::str::FromStr;
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[clap(version = "1.0")]
struct Opts {
    #[clap(long)]
    principal_ids_file_path: PathBuf,

    #[clap(long)]
    stake_e8s: u64,

    #[clap(long)]
    output_file_path: Option<PathBuf>,
}

fn main() {
    let args = Opts::parse();

    let principal_ids_file = fs::read_to_string(args.principal_ids_file_path)
        .expect("Unable to open principal_ids_file_path");

    let airdrop_ids: Vec<String> =
        serde_json::from_str(&principal_ids_file)
            .expect("Unable to deserialize principal_ids_file as JSON");

    let output_file_name = args.output_file_path
        .unwrap_or_else(|| PathBuf::from_str("formatted_airdrop_ids").unwrap());

    let mut output_file = fs::File::create(&output_file_name)
        .expect("Unable to create/open output file");

    let stake_e8s = args.stake_e8s;

    for airdrop_id in airdrop_ids {
        output_file.write_all(format!("        - controller: {}\n", airdrop_id).as_bytes()).unwrap();
        output_file.write_all(format!("          stake_e8s: {}\n", stake_e8s).as_bytes()).unwrap();
        output_file.write_all(b"          memo: 0\n").unwrap();
        output_file.write_all(b"          dissolve_delay_seconds: 0\n").unwrap();
    }
}
