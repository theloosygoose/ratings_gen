use std::{error::Error, io, process};
use csv;

pub fn gen_name() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("../../data/nationalities.csv")?;

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
