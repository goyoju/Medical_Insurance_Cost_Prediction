extern crate csv;
use std::error::Error;

// encoding(replacing)

// female to 0, male to 1
fn encode_sex(value: &str) -> i32 {
    match value {
        "female" => 0,
        "male" => 1,
        _ => panic!("Invalid sex value"),
    }
}

//no to 0, yes to 1
fn encode_smoker(value: &str) -> i32 {
    match value {
        "no" => 0,
        "yes" => 1,
        _ => panic!("Invalid smoker value"),
    }
}

// northeast to 0, northwest to 1, southeast to 2, southwest to 3
fn encode_region(value: &str) -> i32 {
    match value {
        "northeast" => 0,
        "northwest" => 1,
        "southeast" => 2,
        "southwest" => 3,
        _ => panic!("Invalid region value"),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // reading csv file
    let mut temp = csv::Reader::from_path("insurance.csv")?;
    let headers = temp.headers()?.clone();

    // creating modified version of insurance file
    let mut temp2 = csv::Writer::from_path("modified_insurance.csv")?;
    temp2.write_record(&["age", "sex", "bmi", "children", "smoker", "region", "charges"])?;

    // Do iteration to replace all the values in sex, smoker, and region
    for i in temp.records() {
        let rd = i?;
        let sex = encode_sex(&rd[1]);
        let smoker = encode_smoker(&rd[4]);
        let region = encode_region(&rd[5]);

        temp2.write_record(&[
            rd[0].to_string(),
            sex.to_string(),
            rd[2].to_string(),
            rd[3].to_string(),
            smoker.to_string(),
            region.to_string(),
            rd[6].to_string(),
        ])?;
    }

    Ok(())
}
