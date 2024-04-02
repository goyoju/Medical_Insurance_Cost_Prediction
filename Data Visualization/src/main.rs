extern crate csv;
use plotters::prelude::*;
use std::fs::File;
use std::io::prelude::*;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Reading insurance.csv file
    let mut f = File::open("insurance.csv")?;
    let mut temp = String::new();
    //change to string so that rust can read
    f.read_to_string(&mut temp)?;

    // Parsing the data
    let mut pars = csv::ReaderBuilder::new().has_headers(true).from_reader(temp.as_bytes());
    let rec: Result<Vec<csv::StringRecord>, csv::Error> = pars.records().collect();
    let rec = rec?;

    // setting up the comparision with age, bmi, number of children, smoker, and region with charges
    let age_data: Vec<(f64, f64)> = rec
        .iter()
        .filter_map(|i| {
            let age = i.get(0)?.parse::<f64>().ok()?;
            let charges = i.get(6)?.parse::<f64>().ok()?;
            Some((age, charges))
        })
        .collect();

    let bmi_data: Vec<(f64, f64)> = rec
        .iter()
        .filter_map(|i| {
            let bmi = i.get(2)?.parse::<f64>().ok()?;
            let charges = i.get(6)?.parse::<f64>().ok()?;
            Some((bmi, charges))
        })
        .collect();

    let children_data: Vec<(f64, f64)> = rec
        .iter()
        .filter_map(|i| {
            let children = i.get(3)?.parse::<f64>().ok()?;
            let charges = i.get(6)?.parse::<f64>().ok()?;
            Some((children, charges))
        })
        .collect();

    let smoker_data: Vec<(f64, f64)> = rec
        .iter()
        .filter_map(|i| {
            let smoker = if i.get(4)? == "yes" { 1.0 } else { 0.0 };
            let charges = i.get(6)?.parse::<f64>().ok()?;
            Some((smoker, charges))
        })
        .collect();

    let region_data: Vec<(f64, f64)> = rec
        .iter()
        .filter_map(|i| {
            let region = match i.get(5)? {
                "northeast" => 1.0,
                "northwest" => 2.0,
                "southeast" => 3.0,
                "southwest" => 4.0,
                _ => 0.0,
            };
            let charges = i.get(6)?.parse::<f64>().ok()?;
            Some((region, charges))
        })
        .collect();


    // Plotting graphs
    // age vs charges
    let age_corr = BitMapBackend::new("age_plot.png", (800, 800)).into_drawing_area();
    age_corr.fill(&WHITE)?;

    let mut age_graph = ChartBuilder::on(&age_corr)
        .caption("Age vs Charges", ("FF Elementa Bold", 20))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..100.0, 0.0..60000.0)?;

    age_graph.configure_mesh().draw()?;
    age_graph.draw_series(age_data.iter().map(|(x, y)| Circle::new((*x, *y), 3, GREEN.filled())))?;

    // bmi vs charges
    let bmi_corr = BitMapBackend::new("bmi_plot.png", (800, 800)).into_drawing_area();
    bmi_corr.fill(&WHITE)?;

    let mut bmi_graph = ChartBuilder::on(&bmi_corr)
        .caption("BMI vs Charges", ("FF Elementa Bold", 20))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(15.0..40.0, 0.0..60000.0)?;

    bmi_graph.configure_mesh().draw()?;
    bmi_graph.draw_series(bmi_data.iter().map(|(x, y)| Circle::new((*x, *y), 3, BLUE.filled())))?;

    // children vs charges
    let children_corr = BitMapBackend::new("children_plot.png", (800, 800)).into_drawing_area();
    children_corr.fill(&WHITE)?;

    let mut children_graph = ChartBuilder::on(&children_corr)
        .caption("Children vs Charges", ("FF Elementa Bold", 20))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..5.0, 0.0..60000.0)?;

    children_graph.configure_mesh().draw()?;
    children_graph.draw_series(children_data.iter().map(|(x, y)| Circle::new((*x, *y), 3, BLACK.filled())))?;

    // smoker vs charges
    let smoker_corr = BitMapBackend::new("smoker_plot.png", (800, 800)).into_drawing_area();
    smoker_corr.fill(&WHITE)?;

    let mut smoker_graph = ChartBuilder::on(&smoker_corr)
        .caption("Smoker vs Charges", ("FF Elementa Bold", 20))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..1.0, 0.0..60000.0)?;

    smoker_graph.configure_mesh().draw()?;
    smoker_graph.draw_series(smoker_data.iter().map(|(x, y)| Circle::new((*x, *y), 3, BLACK.filled())))?;

    // region vs charges
    let region_corr = BitMapBackend::new("region_plot.png", (800, 800)).into_drawing_area();
    region_corr.fill(&WHITE)?;

    let mut region_graph = ChartBuilder::on(&region_corr)
        .caption("Region vs Charges", ("FF Elementa Bold", 20))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..5.0, 0.0..60000.0)?;

    region_graph.configure_mesh().draw()?;
    region_graph.draw_series(region_data.iter().map(|(x, y)| Circle::new((*x, *y), 3, BLACK.filled())))?;

    Ok(())

}
