use ndarray::prelude::*;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use rand::Rng;
use std::io::{self, Write};

// constructing linear regression model
#[derive(Debug)]
pub struct LinearRegression {
    // input values
    pub items: Array2<f64>,

    // output labels
    pub label: Array1<f64>,

    // number of iterations
    pub iterations: u64,

    // coefficients of each feature
    pub weight: Array1<f64>,

    // y-intercept of the linear equation
    pub bias: f64,
    
    pub mean_y: f64,

    pub learning_rate: f64,
}


// Implement
impl LinearRegression {
    // creating model
    fn new(items: Array2<f64>, label: Array1<f64>, iterations: u64, learning_rate: f64) -> Self {
        //getting mean value of output labels
        let mean_y = label.mean().unwrap();

        // setting up the model
        LinearRegression {
            items: items.clone(),
            label,
            iterations,
            weight: Array::zeros(items.shape()[1]),
            bias: 0.,
            learning_rate,
            mean_y,
        }
    }

    // Firring function
    pub fn fit(&mut self) {
        // number of rows
        let i: usize = self.items.shape()[0];

        // number of columns
        let j: usize = self.items.shape()[1];

        // initialzing
        self.weight = Array::<f64, _>::zeros(j);
        self.bias = 0.;

        // performing gradient descent
        for _ in 0..self.iterations {
            // making a prediction
            let pred = self.items.dot(&self.weight) + self.bias;

            // calculating gradients with formula
            let dif = &self.label - &pred;
            let dw = &self.items.t().dot(&dif).map(|x| (*x) * -2. / i as f64);
            let db = dif.sum() * -2. / i as f64;

            // store weight and bias value
            self.weight = &self.weight - self.learning_rate * dw;
            self.bias -= self.learning_rate * db;
        }
    }

    // Making a prediction
    pub fn predict(&self, items: &Array2<f64>) -> Array1<f64> {
        items.dot(&self.weight) + self.bias
    }

    // calculating r^2 value with formula
    pub fn calculate_r_2(&self, predictions: &Array1<f64>) -> f64 {
        let mean_y = self.label.mean().unwrap();
        1.0 - (predictions.iter().map(|&yhat| (yhat - mean_y).powi(2)).sum::<f64>()
            / self.label.iter().map(|&yi| (yi - mean_y).powi(2)).sum::<f64>())
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    // reading modified_insurance.csv file
    let file = File::open("modified_insurance.csv")?;
    let mut temp = ReaderBuilder::new().has_headers(true).from_reader(file);

    // getting features and labels
    let mut item_x: Vec<Vec<f64>> = Vec::new();
    let mut label_y: Vec<f64> = Vec::new();

    for i in temp.records() {
        let rd = i?;
        let rd_len = rd.len();
        let item_temp: Vec<f64> = rd.iter().take(rd_len - 1).map(|s| s.parse::<f64>().unwrap()).collect();
        let label_temp: f64 = rd[rd_len - 1].parse()?;
        item_x.push(item_temp);
        label_y.push(label_temp);
    }
    // initializing number of rows and columns
    let r = item_x.len();
    let c = item_x[0].len();

    // converting items to array
    let x: Array2<f64> = Array::from_shape_fn((r, c), |(i, j)| item_x[i][j]);

    // converting labels to array
    let y: Array1<f64> = Array::from_vec(label_y);

    // constructing a linear regression with those values we calculated
    let mut model = LinearRegression::new(x.clone(), y.clone(), 1_000, 0.0001);

    // fitting
    model.fit();

    // making a prediction
    let prediction = model.predict(&x);

    // calculating R^2 with calculate_r_squared function
    let r_2 = model.calculate_r_2(&prediction);

    // returning R^2 value
    println!("R^2: {}", r_2);
    
    // Charge prediction
    // Getting variables from user
    let age = read_input("Age: ")?;
    let sex = read_input("Sex (0 for female, 1 for male): ")?;
    let bmi = read_input("BMI: ")?;
    let children = read_input("Children: ")?;
    let smoker = read_input("Smoker (0 for no, 1 for yes): ")?;
    let region = read_input("Region (0 for northwest, 1 for northeast, 2 for southwest, 3 for southeast): ")?;

    // Compute the estimate charge
    let user_data = arr2(&[[age, sex, bmi, children, smoker, region]]);
    let prediction = model.predict(&user_data);

    println!("Predicted Insurance Charge: {:.2}", prediction[0]);

    Ok(())
}

fn read_input(prompt: &str) -> Result<f64, Box<dyn Error>> {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();
    let num: f64 = trimmed.parse()?;
    Ok(num)
}


//testing the model

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        let mut r = rand::thread_rng();
        let item_x: Vec<Vec<f64>> = (0..100)
            .map(|_| vec![r.gen_range(0.0..10.0), r.gen_range(0.0..10.0)])
            .collect();
        let label_y: Vec<f64> = (0..100).map(|_| r.gen_range(0.0..10.0)).collect();

        let r = item_x.len();
        let c = item_x[0].len();

        // converting items to array
        let x: Array2<f64> = Array::from_shape_fn((r, c), |(i, j)| item_x[i][j]);

        //converting labels to array
        let y: Array1<f64> = Array::from_vec(label_y);

        // constructing a linear regression
        let mut model = LinearRegression::new(x.clone(), y.clone(), 1_000, 0.0001);

        // fitting
        model.fit();

        // making a prediction
        let prediction = model.predict(&x);

        // find if prediction and y have same shape
        assert_eq!(prediction.shape(), y.shape());

        // calculating R^2 with calculate_r_squared function and find it works or not
        let r_2 = model.calculate_r_2(&prediction);
        assert!(r_2 >= 0.0 && r_2 <= 1.0);
    }
}