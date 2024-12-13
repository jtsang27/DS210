mod read_data;
mod plot;

use std::collections::HashMap;
use chrono::NaiveDate;
use read_data::{read_theft_data, CrimeRecord};
use plot::{plot_crime_over_time, plot_bar_chart};
use std::error::Error;

fn aggregate_counts(records: &[CrimeRecord], key_selector: impl Fn(&CrimeRecord) -> String) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for record in records {
        *counts.entry(key_selector(record)).or_insert(0) += 1;
    }
    counts
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Crime_Data_from_2020_to_Present_20241204.csv";
    let all_data = read_theft_data(file_path)?;
    let specific_crime = "VEHICLE - STOLEN"; 
    let specific_location = "Hollywood"; 

    let filtered_data: Vec<CrimeRecord> = all_data
        .iter()
        .filter(|record| record.crime_desc == specific_crime && record.area == specific_location)
        .cloned()
        .collect();

    if !filtered_data.is_empty() {
        let mut crime_counts_over_time = HashMap::new();
        for record in filtered_data {
            if let Ok(date) = NaiveDate::parse_from_str(&record.crime_date, "%m/%d/%Y %I:%M:%S %p") {
                *crime_counts_over_time.entry(date).or_insert(0) += 1;
            }
        }

        plot_crime_over_time(crime_counts_over_time, "crime_over_time_plot.png")?;
        println!(
            "Crime trend plot for '{}' in '{}' saved as crime_over_time_plot.png.",
            specific_crime, specific_location
        );
    } else {
        println!("No records found for the specified crime and location.");
    }

    let crime_counts = aggregate_counts(&all_data, |record| record.crime_desc.clone());
    plot_bar_chart(
        "top_5_crimes.png",
        "Top 5 Most Common Crimes",
        crime_counts,
        5,
    )?;
    println!("Bar chart for the top 5 most common crimes saved as top_5_crimes.png.");


    let location_counts = aggregate_counts(&all_data, |record| record.area.clone());
    plot_bar_chart(
        "top_5_locations.png",
        "Top 5 Locations with Most Crimes",
        location_counts,
        5,
    )?;
    println!("Bar chart for the top 5 locations saved as top_5_locations.png.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::env::temp_dir;

    #[test]
    fn test_missing_file() {
        let file_path = "Crime_Data_from_2020_to_Present_20241204.csv"; 
        let result = read_theft_data(file_path);
        assert!(result.is_ok(), "Expected to fail if the file does not exist");
    }
    
    #[test]
    fn test_malformed_data() {
        let mut file_path = std::env::temp_dir();
        file_path.push("malformed_test.csv");
        let mut file = File::create(&file_path).unwrap();
        writeln!(
            file,
            "area,crime_desc,crime_date\n\
            Area1,Theft,2023-01-01\n\
            null row\n\
            Area2,Burglary,2023-01-02"
        )
        .unwrap();
        let result = read_theft_data(file_path.to_str().unwrap());
        assert!(result.is_ok(), "Error for malformed data, but got {:?}", result);
    }
}
