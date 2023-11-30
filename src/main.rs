use dirs::home_dir;
use dotenv::dotenv;
use jrutils::utils::type_of;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use toml::Value;

struct Application {
    name: String,
    path: String,
    args: String,
}

fn read_config(config_path: &Path, table_name: &str) -> Vec<Application> {
    let toml_content = fs::read_to_string(config_path).expect("Failed to read file!");
    let parsed_toml: Value = toml::from_str(&toml_content).expect("Failed to parse toml!");

    println!("Type Of Parsed Toml: {:?}", type_of(parsed_toml.clone()));

    let table = parsed_toml
        .get(table_name)
        .expect("Couldn't get table name!");

    let toml_applications = table
        .get(0)
        .unwrap()
        .get("applications")
        .expect("Couldn't get applications!");

    let v_applications = toml_applications.as_array().unwrap();

    let mut applications: Vec<Application> = Vec::new();

    for app in v_applications {
        let toml_name = app.get("name").unwrap();
        let toml_path = app.get("command").unwrap();
        let toml_args = app.get("args").unwrap();

        if let Some(name) = toml_name.as_str() {
            if let Some(path) = toml_path.as_str() {
                if let Some(args) = toml_args.as_str() {
                    let application = Application {
                        name: name.to_string(),
                        path: path.to_string(),
                        args: args.to_string(),
                    };

                    applications.push(application);
                }
            }
        }
    }

    return applications;
}

fn parse_argument(argument: &str) -> Result<(&str, &str), &'static str> {
    let mut split = argument.split(".");

    let config_file = match split.next() {
        Some(c) => c,
        None => {
            panic!("No config found!");
        }
    };

    let spec = match split.next() {
        Some(s) => s,
        None => {
            panic!("No spec found!");
        }
    };

    return Ok((config_file, spec));
}

fn main() {
    // Declare pathing
    const TOML_EXT: &str = ".toml";
    let home_path = dirs::home_dir().expect("Could not get home directory!");
    let conf_path = home_path.join(".config/cpl/confs/");

    // Take in arguments from CLI
    let args: Vec<String> = env::args().collect();
    let config_arg = &args[1];

    let config: &str;
    let table_name: &str;

    // Parse argument
    if let Ok((conf, spec)) = parse_argument(config_arg) {
        config = conf;
        table_name = spec;
    } else {
        panic!("Could not parse argument!");
    }

    let complete_path: String; // Define complete_path outside the scope

    if let Some(conf_str) = conf_path.to_str() {
        let formatted = format!("{}{}{}", conf_str, config, TOML_EXT);
        complete_path = String::from(formatted); // Allocate memory for complete_path
    } else {
        panic!("Path could not be made!");
    }

    let path = Path::new(&complete_path);

    let applications = read_config(path, table_name);

    for app in applications {
        let args: Vec<&str> = app.args.split_whitespace().collect();
        Command::new(app.path).args(&args).spawn();
    }
}
