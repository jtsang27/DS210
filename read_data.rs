use std::error::Error;
use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CrimeRecord {
    pub area: String,
    pub crime_desc: String,
    pub crime_date: String,
}

pub fn read_theft_data(file_path: &str) -> Result<Vec<CrimeRecord>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut theft_locations = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let crime_desc = record.get(9).unwrap_or("Unknown").to_string();
        let area_name = record.get(5).unwrap_or("Unknown").to_string();
        let crime_date = record.get(2).unwrap_or("1970-01-01").to_string();

        theft_locations.push(CrimeRecord {
            area: area_name,
            crime_desc,
            crime_date,
        });
    }

    Ok(theft_locations)
}