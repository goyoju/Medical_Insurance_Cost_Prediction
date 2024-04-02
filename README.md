# Medical Insurance Cost Prediction Model

The primary goal of this project is to develop a **regression model** capable of predicting medical insurance costs, utilizing key features such as age, BMI, number of children, region, and smoking status. In this project, we have successfully implemented a model that receives values for these characteristics from the user and, based on them, predicts the medical insurance costs.

This model analyzes the personal information provided by the user to forecast the cost of medical insurance. It allows users to understand how their lifestyle or conditions could potentially affect their insurance costs. Moreover, if necessary, it can aid in making informed decisions about adopting healthier lifestyle choices or reassessing their insurance plans.

By employing data science and machine learning techniques, this project tackles a real-world issue, providing practical insights and valuable predictions to users regarding their medical insurance costs.

**Input :**

![image](https://github.com/goyoju/Medical_Insurance_Cost_Prediction/assets/61122366/e9c3f347-c1b1-4646-a9a8-6040725dd63c)

**Output :**

![image](https://github.com/goyoju/Medical_Insurance_Cost_Prediction/assets/61122366/b9297205-1eac-44a3-8db9-8b157ecfd2a7)

This example illustrates how the model utilizes the provided personal information to estimate medical insurance costs, offering users insights into the factors that significantly impact their insurance premiums.

## How to use

**System**
- Ensure you have Rust installed on your system. You can install Rust through [rustup](https://www.rust-lang.org/tools/install).
- Use a version control tool like Git to clone the repository.

**Running the Prediction Model**
- Clone the Repository

{

    $ git clone https://github.com/goyoju/Medical_Insurance_Cost_Prediction.git .
}

- Use the **command prompt (cmd)** to navigate to the cloned repository's directory:
 
{

    $ cd <your_file_location>\Prediction
}

- Use the following command to compile and run the prediction model

{

    $ cargo run
}

Follow the on-screen prompts to input your Age, Sex (0 for female, 1 for male), BMI, Number of Children, Smoker (0 for no, 1 for yes), and Region (0 for northwest, 1 for northeast, 2 for southwest, 3 for southeast).

## Example Outputs
**Input :**

- Age: **19**
- Sex (0 for female, 1 for male): **1**
- BMI: **25.05**
- Children: **0**
- Smoker (0 for no, 1 for yes): **0**
- Region (0 for northwest, 1 for northeast, 2 for southwest, 3 for southeast): **1**

**Output :**

Predicted Insurance Charge: **$8126.05**


## Data Overview
**Source:** [Kaggle](https://www.kaggle.com/datasets/mirichoi0218/insurance/data)

![datasetmedical](https://github.com/goyoju/Earthquake_Data_Visualization/assets/61122366/da728c9b-7236-490e-85bc-a81b181ee26c)

**Columns** : 
- **age:** age of primary beneficiary
-  **sex** insurance contractor gender, female, male
-  **bmi:** Body mass index, providing an understanding of body, weights that are relatively high or low relative to height, objective index of body weight (kg / m ^ 2) using the ratio of height to weight, ideally 18.5 to 24.9
-  **children:** Number of children covered by health insurance / Number of dependents
-  **smoker:** Smoking
-  **region:** the beneficiary's residential area in the US, northeast, southeast, southwest, northwest.
-  **charges:**  Individual medical costs billed by health insurance

## Visualization
**Age vs Charges**
![age_plot](https://github.com/goyoju/Earthquake_Data_Visualization/assets/61122366/f20b0a0b-5b2e-4a75-b9db-d20e72724652)

**BMI vs Charges**
![bmi_plot](https://github.com/goyoju/Earthquake_Data_Visualization/assets/61122366/d3075c01-14e9-4a4d-8b86-849c625f5294)

**Chindren vs Charges**
![children_plot](https://github.com/goyoju/Earthquake_Data_Visualization/assets/61122366/fb13cbce-bdea-4823-b537-4173e2f61761)

**Smoker vs Charges**
![smoker_plot](https://github.com/goyoju/Earthquake_Data_Visualization/assets/61122366/89feb21c-6337-4a18-9b5f-19cabd9e7e2a)

**Region vs Charges**
![region_plot](https://github.com/goyoju/Earthquake_Data_Visualization/assets/61122366/60472e96-0ab0-4c9a-b68c-3424a2aca4f5)

## Encoded dataset
**Modified_csv.csv:**

![2024-03-30 223331](https://github.com/goyoju/Earthquake_Data_Visualization/assets/61122366/faa6f2a3-943a-4d48-ab45-6dae5544ed94)


## Prediction Model ( Linear Regression )
Using **Gradient Descent Algorithm**

![2024-03-30 223637](https://github.com/goyoju/Earthquake_Data_Visualization/assets/61122366/572a33ee-3ced-49a5-82c4-32eb434d8d11)

**Imange Source:**

[Geeksforgeeks](https://www.geeksforgeeks.org/gradient-descent-in-linear-regression/)

Result of the model ( **R^2** score):

![2024-03-30 224612](https://github.com/goyoju/Earthquake_Data_Visualization/assets/61122366/82ac5531-80ee-4221-b984-f17574520069)

## Test

![2024-03-30 224811](https://github.com/goyoju/Earthquake_Data_Visualization/assets/61122366/165508fa-6562-41a3-a525-a74cae9dc56c)
