use cli_table::{print_stdout, Table, WithTitle};
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

#[derive(Table)]
struct Employee {
    #[table(title = "First Name")]
    first_name: String,
    #[table(title = "Last Name")]
    last_name: String,
    #[table(title = "Position")]
    position: String,
    #[table(title = "Separation Date")]
    separation_date: String,
}

fn main() {
    let dataset: Vec<HashMap<&str, &str>> = vec![
        HashMap::from([
            ("first_name", "John"),
            ("last_name", "Johnson"),
            ("position", "Manager"),
            ("separation_date", "2016-12-31"),
        ]),
        HashMap::from([
            ("first_name", "Tou"),
            ("last_name", "Xiong"),
            ("position", "Software Engineer"),
            ("separation_date", "2016-10-05"),
        ]),
        HashMap::from([
            ("first_name", "Michaela"),
            ("last_name", "Michaelson"),
            ("position", "District Manager"),
            ("separation_date", "2015-12-19"),
        ]),
        HashMap::from([
            ("first_name", "Jake"),
            ("last_name", "Jacobson"),
            ("position", "Programmer"),
            ("separation_date", ""),
        ]),
        HashMap::from([
            ("first_name", "Jacquelyn"),
            ("last_name", "Jackson"),
            ("position", "DBA"),
            ("separation_date", ""),
        ]),
        HashMap::from([
            ("first_name", "Sally"),
            ("last_name", "Weber"),
            ("position", "Web Developer"),
            ("separation_date", "2015-12-18"),
        ]),
    ];

    let mut sorted: Vec<HashMap<&str, &str>> = dataset.clone();
    sorted.sort_by(|a, b| {
        let a_first = a.get("first_name").unwrap_or(&"");
        let b_first = b.get("first_name").unwrap_or(&"");
        a_first.cmp(b_first)
    });

    let employees_table: Vec<Employee> = sorted
        .into_iter()
        .map(|mut employee| Employee {
            first_name: employee.remove("first_name").unwrap().to_string(),
            last_name: employee.remove("last_name").unwrap().to_string(),
            position: employee.remove("position").unwrap().to_string(),
            separation_date: employee.remove("separation_date").unwrap().to_string(),
        })
        .collect();

    assert!(print_stdout(employees_table.with_title()).is_ok());
}
