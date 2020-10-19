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
    let s3d = match PFSArchive::from_file(filename) {
        Ok(v) => v,
        Err(e) => panic!("err {}", e),
    };

    let outdir = matches.value_of("OUTDIR").unwrap();
    fs::create_dir_all(outdir).unwrap();

    for f in &s3d.files {
        let outname = format!("{}/{}", outdir, f.name);
        println!("Writing {}", outname);
        fs::write(outname, f.data.clone()).expect("Unable to write file");
    }
}
