extern crate rand;
extern crate itertools;
extern crate flate2;

#[macro_use]
extern crate clap;

use clap::{Arg, App};

use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use itertools::Itertools;
use flate2::GzBuilder;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;
use rand::Rng;

fn main() {
    let matches = App::new("FastDownsample")
                          .version("1.0")
                          .author("Aaron M.")
                          .about("downsamples a fastq file")
                          .arg(Arg::with_name("fastq1")
                               .short("o")
                               .long("fastq1")
                               .value_name("FILE")
                               .help("input fastq1")
                               .required(true)
                               .takes_value(true))
                          .arg(Arg::with_name("fastq2")
                               .short("t")
                               .long("fastq2")
                               .value_name("FILE")
                               .help("input fastq2")
                               .required(true)
                               .takes_value(true))
                          .arg(Arg::with_name("outfq1")
                               .short("x")
                               .long("outfq1")
                               .value_name("FILE")
                               .help("output fastq2")
                               .required(true)
                               .takes_value(true))
                          .arg(Arg::with_name("outfq2")
                               .short("z")
                               .long("outfq2")
                               .value_name("FILE")
                               .help("output fastq2")
                               .required(true)
                               .takes_value(true))
                          .arg(Arg::with_name("downsample")
                               .short("d")
                               .long("downsample")
                               .value_name("FILE")
                               .help("output fastq2")
                               .required(true)
                               .takes_value(true))
                          .get_matches();
    
    let downsample = value_t!(matches, "downsample", f64).unwrap_or(1.0);
    let read1 = File::open(matches.value_of("fastq1").unwrap_or("read1")).unwrap();
    let read2 = File::open(matches.value_of("fastq2").unwrap_or("read2")).unwrap();
    let out1 = File::create(matches.value_of("outfq1").unwrap_or("out1")).unwrap();
    let out2 = File::create(matches.value_of("outfq2").unwrap_or("out2")).unwrap();

    // make our random number generator
    let mut rng = rand::thread_rng();

    // setup the input files
    let dc1 = GzDecoder::new(read1);
    let igz1 = BufReader::new(dc1).lines().chunks(4);
    let dc2 = GzDecoder::new(read2);
    let igz2 = BufReader::new(dc2).lines().chunks(4);

    // setup the output files
    let mut outw1 = BufWriter::new(GzEncoder::new(out1,Compression::default()));
    let mut outw2 = BufWriter::new(GzEncoder::new(out2,Compression::default()));

    let mut chunk2_iter = igz2.into_iter();
    for chunk1 in igz1.into_iter() {
        let chunk2 = chunk2_iter.next().unwrap();
        
        let y: f64 = rng.gen::<f64>(); // generates a float between 0 and 1

        if y <= downsample {
            for c in chunk1 {
                write!(outw1, "{}\n", c.unwrap());
            }
            for c in chunk2 {
                outw2.write(c.unwrap().as_bytes());
            }
        }
    }
}
