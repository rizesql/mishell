use glob::glob;
use std::{
    collections::HashMap,
    env::{self, set_var},
    fs,
};

use nix::unistd::alarm::set;

#[derive(Debug, Clone)]
pub struct VariableManager {
    variables: HashMap<String, String>,
}

impl VariableManager {
    // Constructor: Initialize with environment variables
    pub fn new() -> Self {
        let mut variables = HashMap::new();
        // Populate the variables map with environment variables
        for (key, value) in env::vars() {
            variables.insert(key.to_lowercase(), value); // Store them in the variables map
        }

        VariableManager { variables }
    }

    pub fn set_var(&mut self, var: &str, val: &str) {
        self.variables
            // .insert(var[1..].to_string().to_lowercase(), val.to_string());
            .insert(var.to_string().to_lowercase(), val.to_string());
    }

    // Get a variable value by its name (without '$' prefix)
    pub fn get_var(&self, var: &str) -> Option<&String> {
        self.variables.get(&var[1..].to_lowercase())
    }

    // Remove a variable from the map
    pub fn remove_var(&mut self, var: &str) {
        self.variables.remove(var);
    }

    // Replace variables in command arguments with their values
    pub fn replace_vars_in_args(&mut self, args: &mut Vec<String>) {
        for arg in args.iter_mut() {
            if arg.starts_with('$') {
                // let var_name = &arg[1..].to_lowercase(); // Remove '$' prefix to get the variable name
                let var_name = &arg.to_lowercase(); // Remove '$' prefix to get the variable name

                // Try to get the value of the variable from the custom map
                if let Some(var_value) = self.get_var(var_name) {
                    *arg = var_value.clone(); // Replace the variable with its value
                } else {
                    *arg = String::from(""); // If not found, replace with empty string
                }
            }
        }
    }

    pub fn map_for_values(
        &mut self,
        values: &Vec<String>,
    ) -> Result<Vec<String>, glob::PatternError> {
        let mut files: Vec<String> = Vec::new();

        for val in values.iter() {
            if let Some(v) = self.get_var(val) {
                files.push(v.clone());
                continue;
            }

            for entry in glob(&val)? {
                match entry {
                    Ok(path) => {
                        files.push(path.display().to_string());
                    }
                    Err(_) => eprint!("Failed to {:?} pattern", val),
                }
            }
        }

        Ok(files)
    }
}
