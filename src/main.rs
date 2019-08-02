extern crate rand;


use rand::seq::IteratorRandom;
use std::error;
use std::fmt;


fn main() -> Result<(), Box<dyn error::Error>> {
    let male_given_names = [
        "Alan", "Bob", "Chris", "David", "Ed", "Fred",
    ];
    let female_given_names = [
        "Alice", "Brenda", "Cathy", "Donna", "Emma", "Francis",
    ];
    let given_name_sources = [ male_given_names, female_given_names ];
    let family_names = [
        "Adams", "Bailey", "Campbell", "Davis", "Edwards", "Fisher",
    ];

    let mut rng = rand::thread_rng();
    println!("row, given_name, family_name");
    for row in 1..11 {
        let given_name_source = given_name_sources.iter().choose(&mut rng)
            .ok_or(Box::new(Error::DataSourceIsEmpty("given name sources".to_string())))?;
        let given_name = given_name_source.iter().choose(&mut rng)
            .ok_or(Box::new(Error::DataSourceIsEmpty("male given names".to_string())))?;
        let family_name= family_names.iter().choose(&mut rng)
            .ok_or(Box::new(Error::DataSourceIsEmpty("family names".to_string())))?;
        println!("{}, \"{}\", \"{}\"", row, given_name, family_name);
    }
    Ok(())
}


#[derive(Debug)]
pub enum Error {
    DataSourceIsEmpty(String),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DataSourceIsEmpty(ref source_name) => {
                write!(f, "Data source \"{}\" is empty", source_name)
            },
        }
    }
}
