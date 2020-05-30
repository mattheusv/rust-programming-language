/*
 *
 * Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
 * For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department
 * or all people in the company by department, sorted alphabetically.
 *
 * */
use std::collections::HashMap;
use std::{io, process};

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter with operation or empty to cancel: ");
        let mut command_input = String::new();

        io::stdin()
            .read_line(&mut command_input)
            .expect("Failed to read input");

        command_input = command_input.replace("\n", "");

        let commands: Vec<&str> = command_input.split(' ').collect();

        if commands.len() == 1 {
            println!("Bye!");
            process::exit(0);
        }

        let command = commands[0];

        match command {
            "Add" => {
                if commands.len() < 4 {
                    println!("Invalid syntax to Add command");
                    process::exit(1);
                }
                command_add(
                    commands[1].to_string(),
                    commands[3].to_string(),
                    &mut company,
                );
            }
            "List" => {
                if command.len() < 2 {
                    println!("Invalid syntax to List command");
                    process::exit(1);
                }
                let operation = commands[1];
                match operation {
                    "All" => {
                        command_list_all(&company);
                    }
                    other => {
                        command_list_by_departament(other.to_string(), &company);
                    }
                }
            }
            _ => {
                println!("Invalid command {}", command);
                process::exit(1);
            }
        }
    }
}

fn command_list_by_departament(department: String, company: &HashMap<String, Vec<String>>) {
    match company.get(&department) {
        Some(peoples) => {
            // TODO Maybe exist another way to do this???????
            let mut sorted_people = peoples.clone();
            sorted_people.sort();

            for people in sorted_people {
                println!("{}", people)
            }
        }
        None => {
            println!("Not found department {}", department);
        }
    }
}

fn command_list_all(company: &HashMap<String, Vec<String>>) {
    for (department, peoples) in company.iter() {
        // TODO Maybe exist another way to do this???????
        let mut sorted_people = peoples.clone();
        sorted_people.sort();

        for people in sorted_people {
            println!("{} - {}", department, people);
        }
    }
}

fn command_add(name: String, department: String, company: &mut HashMap<String, Vec<String>>) {
    let department_values = company.entry(department).or_insert(vec![]);
    department_values.push(name);
    println!("Ok");
}
