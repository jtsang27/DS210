use plotters::prelude::*;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::error::Error;

pub fn plot_bar_chart(
    output_path: &str,
    title: &str,
    data: HashMap<String, usize>,
    limit: usize,
) -> Result<(), Box<dyn Error>> {

    let mut data: Vec<(String, usize)> = data.into_iter().collect();
    data.sort_by(|a, b| b.1.cmp(&a.1)); 
    let data = data.into_iter().take(limit).collect::<Vec<_>>();

    let root = BitMapBackend::new(output_path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_count = data.iter().map(|(_, count)| *count).max().unwrap_or(0);

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 20))
        .x_label_area_size(20) 
        .y_label_area_size(50)
        .margin(20)
        .build_cartesian_2d(0..data.len(), 0..max_count)?;
    chart.configure_mesh()
        .x_labels(data.len())
        .y_labels(10)
        .x_label_formatter(&|x| {
            if let Some((label, _)) = data.get(*x) {
                label.clone()
            } else {
                "".to_string()
            }
        })
        .x_desc("Categories") 
        .y_desc("Counts")     
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(data.iter().enumerate().map(|(i, (_label, value))| {
        Rectangle::new([(i, 0), (i + 1, *value)], BLUE.filled().stroke_width(1))
    }))?;
    root.present()?;
    Ok(())
}

pub fn plot_crime_over_time( data: HashMap<NaiveDate, usize>, output_path: &str,) -> Result<(), Box<dyn Error>> {
    let mut sorted_data: Vec<(NaiveDate, usize)> = data.into_iter().collect();
    sorted_data.sort_by_key(|&(date, _)| date);

    let root = BitMapBackend::new(output_path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;
    let max_count = sorted_data.iter().map(|&(_, count)| count).max().unwrap_or(0);

    let (min_date, max_date) = (
        sorted_data.first().map(|&(date, _)| date).unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
        sorted_data.last().map(|&(date, _)| date).unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
    );

    let mut chart = ChartBuilder::on(&root)
        .caption("Crime Trends Over Time", ("sans-serif", 20))
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(min_date..max_date, 0..max_count)?;

    chart.configure_mesh().x_labels(10).y_labels(10).draw()?;

    chart.draw_series(LineSeries::new(
        sorted_data.iter().map(|&(date, count)| (date, count)),
        &BLUE,
    ))?
    .label("Crime Count")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));

    chart.configure_series_labels().border_style(&BLACK).draw()?;

    root.present()?;
    Ok(())
}