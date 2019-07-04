extern crate clap;
extern crate pbr;

mod providers;
mod downloader;
mod request;

use clap::{Arg, App};
use std::fs::File;
use std::io::{Write};
use pbr::{ProgressBar, Units};
use downloader::Downloader;
use providers::{Providers, Provider};

fn main() {
    let matches = App::new("Download Manager")
        .version("0.1")
        .author("Saeed M.")
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("Output"),
        )
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let output = matches.value_of("output");

    let mut prs = Providers::new();
    // add custom providers
    prs.register(providers::Youtube);
    
    
    prs.register(providers::Direct);
    let fi = prs.fileinfo(url).unwrap();

    let mut dlr = Downloader::new(&fi.direct_url());
    let mut writer = File::create(output.unwrap_or(&fi.filename())).expect("something went wrong during file creation!");

    let mut pb = ProgressBar::new(dlr.file_size());
    pb.format("╢▌▌░╟");
    pb.set_units(Units::Bytes);

    let mut buf = [0; 128 * 1024];
    loop {
        match dlr.dl(&mut buf) {
            Ok(len) => {
                writer.write_all(&buf[..len]).unwrap();
                pb.add(len as u64);
                if len == 0 {
                    break;
                }
            }
            Err(why) => panic!("{}", why),
        };
    }
}