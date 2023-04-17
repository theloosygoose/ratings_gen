use std::error::Error;
use std::fs;
use csv::Reader;

pub fn gen_name() -> Result<(), Box<dyn Error>> {
    
    let csv_string:String = fs::read_to_string("../../data/nationality.txt")?.parse()?;

    println!("{:#?}", csv_string);
    
    let mut rdr = Reader::from_reader(csv_string.as_bytes());

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
