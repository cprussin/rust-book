use std::collections::{BTreeMap, BTreeSet};

pub mod cli;

type Department = String;
type Name = String;

pub struct Company {
    pub name: String,
    departments: BTreeMap<Department, BTreeSet<Name>>,
}

impl Company {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            departments: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, department: &str) {
        self.departments
            .entry(String::from(department))
            .or_default()
            .insert(String::from(name));
    }

    pub fn get_department_members(&self, department: &str) -> Option<&BTreeSet<String>> {
        self.departments.get(department)
    }

    pub fn get_departments(&self) -> Vec<&Department> {
        self.departments.keys().collect::<Vec<_>>()
    }
}
