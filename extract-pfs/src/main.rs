use std::fs;

use clap::{Arg, App};

use eqformat_pfs::pfs::PFSArchive;

fn main() {
    let matches = App::new("extract-pfs")
        .version("0.1.0")
        .arg(Arg::with_name("INPUT")
            .help("Set the PFS file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTDIR")
            .long("outdir")
            .takes_value(true)
            .required(true)
            .help("Set the output directory"))
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap();
    let archive = match PFSArchive::from_file(filename) {
        Ok(v) => v,
        Err(e) => panic!("err {}", e),
    };

    let out_dir = matches.value_of("OUTDIR").unwrap();
    fs::create_dir_all(out_dir).unwrap();

    for f in &archive.files {
        let out_name = format!("{}/{}", out_dir, f.name);
        println!("Writing {}", out_name);
        fs::write(out_name, f.data.clone()).expect("Unable to write file");
    }
}
