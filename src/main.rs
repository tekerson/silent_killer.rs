extern crate hound;

use std::{convert, env, i16, io};

#[derive(Debug)]
struct Error;

enum Heads {
    Copying,
    Searching(usize),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    run2(&args[1]).expect("Something broke");
}

#[allow(dead_code)]
fn run(file: &str) -> Result<(), Error> {
    let lower_threshold = 5;
    let upper_threshold = 500;
    let quiet_threshold = 44100;

    let mut reader = hound::WavReader::open(file)?;
    let mut scanner = hound::WavReader::open(file)?;
    let mut writer = hound::WavWriter::create("copy.wav", reader.spec())?;

    let mut rs = reader.samples::<i16>().map(|s| s.unwrap());
    let mut ss = scanner.samples::<i16>().map(|s| s.unwrap());
    let mut state = Heads::Searching(rs.len());
    loop {
        match ss.next() {
            None => {
                return Ok(());
            }
            Some(sample) => {
                let magnitude = sample.abs();

                match state {
                    Heads::Copying => {
                        let r = rs.next();
                        if magnitude < lower_threshold {
                            state = Heads::Searching(ss.len());
                        } else {
                            writer.write_sample(r.unwrap()).unwrap();
                        }
                    }
                    Heads::Searching(from) => {
                        if magnitude > upper_threshold {
                            if from - ss.len() > quiet_threshold {
                                while rs.len() > ss.len() {
                                    rs.next();
                                }
                            }
                            while rs.len() > ss.len() {
                                writer.write_sample(rs.next().unwrap()).unwrap();
                            }
                            assert!(rs.len() == ss.len());
                            state = Heads::Copying;
                        }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn run2(file: &str) -> Result<(), Error> {
    let lower_threshold = 5;
    let upper_threshold = 500;
    let quiet_threshold = 44100;

    let mut reader = hound::WavReader::open(file)?;
    let mut writer = hound::WavWriter::create("copy.wav", reader.spec())?;

    let ss = reader
        .samples::<i16>()
        .map(|s| s.expect("Error reading audio"))
        .collect::<Vec<_>>();
    let mut state = Heads::Searching(0);
    for (n, sample) in ss.iter().enumerate() {
        let magnitude = sample.abs();

        match state {
            Heads::Copying => {
                if magnitude < lower_threshold {
                    state = Heads::Searching(n);
                } else {
                    writer.write_sample(*sample).unwrap();
                }
            }
            Heads::Searching(from) => {
                if magnitude > upper_threshold {
                    if n - from < quiet_threshold {
                        for r in &ss[from..=n] {
                            writer.write_sample(*r).unwrap();
                        }
                    }
                    state = Heads::Copying;
                }
            }
        }
    }
    Ok(())
}

impl convert::From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Error
    }
}
impl convert::From<hound::Error> for Error {
    fn from(_: hound::Error) -> Self {
        Error
    }
}
