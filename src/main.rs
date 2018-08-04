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
    run(&args[1]).expect("Something broke");
}

#[allow(dead_code)]
fn run(file: &str) -> Result<(), Error> {
    let lower_threshold = 5;
    let upper_threshold = 500;
    let quiet_threshold = 44100;

    let mut reader = hound::WavReader::open(file)?;
    let mut reader2 = hound::WavReader::open(file)?;
    let mut writer = hound::WavWriter::create("copy.wav", reader.spec())?;

    let ss = reader
        .samples::<i16>()
        .map(|s| s.expect("Error reading audio"));
    let mut state = Heads::Searching(0);
    for (n, sample) in ss.enumerate() {
        let magnitude = sample.abs();

        match state {
            Heads::Copying => {
                if magnitude < lower_threshold {
                    state = Heads::Searching(n);
                } else {
                    writer.write_sample(sample).unwrap();
                }
            }
            Heads::Searching(from) => {
                if magnitude > upper_threshold {
                    if n - from < quiet_threshold {
                        reader2.seek(from as u32)?;
                        let mut rs = reader2.samples::<i16>();
                        for _ in from..=n {
                            let r = rs.next().unwrap()?;
                            writer.write_sample(r).unwrap();
                        }
                    }
                    state = Heads::Copying;
                }
            }
        }
    }
    Ok(())
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
