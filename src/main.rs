use std::collections::HashMap;

// Create a program that sorts this dataset by last name and print by tabular format.
// Dataset:
// First Name Last Name Position Separation date
// John Johnson Manager 2016-12-31
// Tou Xiong Software Engineer 2016-10-05
// Michaela Michaelson District Manager 2015-12-19
// Jake Jacobson Programmer
// Jacquelyn Jackson DBA
// Sally Weber Web Developer 2015-12-18

// Inputs: the dataset, vector of hashmap
// Process: combine lname + fname to name, sort by name.
// Output: sorted data by name. fields: name, position, separation date

fn main() {
    let dataset: Vec<HashMap<String, String>> = vec![HashMap::from([
        ("first_name".to_string(), "John".to_string()),
        ("last_name".to_string(), "Johnson".to_string()),
        ("position".to_string(), "Manager".to_string()),
        ("separation_date".to_string(), "2016-12-31".to_string()),
    ])];
    println!("Hello, world!");
}
