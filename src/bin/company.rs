use clap::Parser;
use inquire::CustomType;
use rust_book::company::Company;
use std::{
    fmt::{Display, Error, Formatter},
    str::FromStr,
};

const HELP_MESSAGE: &str = "\
    Please input a command.  Recognized commands:\n\
    \tadd <person> to <department>\n\
    \tlist <department>\n\
    \tlist\n\
    \tquit\
";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {}

#[derive(Clone)]
pub enum Command {
    Add { name: String, department: String },
    ListDepartment { department: String },
    List,
    Quit,
}

pub struct CommandErr;

impl FromStr for Command {
    type Err = CommandErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(' ').collect::<Vec<_>>()[..] {
            ["add", name, "to", department] => Ok(Command::Add {
                name: String::from(name),
                department: String::from(department),
            }),
            ["list", department] => Ok(Command::ListDepartment {
                department: String::from(department),
            }),
            ["list"] => Ok(Command::List),
            ["quit"] => Ok(Command::Quit),
            _ => Err(CommandErr),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Command::Add { name, department } => write!(f, "Add {name} to {department}"),
            Command::ListDepartment { department } => write!(f, "List {department}"),
            Command::List => write!(f, "List"),
            Command::Quit => write!(f, "Quit"),
        }
    }
}

fn main() {
    Cli::parse();

    let mut company = Company::new("Big Company Inc., Incorporated");

    println!("Welcome to {}!", company.name);

    loop {
        match CustomType::<Command>::new("> ")
            .with_help_message(HELP_MESSAGE)
            .prompt()
        {
            Ok(Command::Add { name, department }) => add(&mut company, &name, &department),
            Ok(Command::ListDepartment { department }) => list_department(&company, &department),
            Ok(Command::List) => list_company(&company),
            _ => break,
        }
    }
}

fn add(company: &mut Company, name: &str, department: &str) {
    company.add(name, department);
    println!("OK, I've added {name} to {department}")
}

fn list_department(company: &Company, department: &str) {
    match company.get_department_members(department) {
        None => println!("There is no {department} department!"),
        Some(members) => {
            println!("The members of {department} are:");
            for member in members {
                println!("  {member}")
            }
        }
    }
}

fn list_company(company: &Company) {
    for department in company.get_departments() {
        list_department(company, department);
    }
}
