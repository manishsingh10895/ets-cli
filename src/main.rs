mod helper;
use colored::*;
use std::fs;
use std::io::Write;

const ROOT: &str = "src";

fn show_help() {
    let help = r#"
        usage: 
            g -> to generate a new file
            route || r -> generate route
            controller || c -> generate controller
            service || s -> generate service
            request-schema || rs -> generate api request schema
            middleware || mw generate new middleware
            model || m -> generate new model
    "#;

    println!("{}", help);
}

fn show_secondary_help(primary: &str) {
    let mut help: &str = "";
    match primary {
        "g" => {
            help = r#"
                you can usage: 
                    g -> to generate a new file
                    route || r -> generate route
                    controller || c -> generate controller
                    service || s -> generate service
                    request-schema || rs -> generate api request schema
                    middleware || mw generate new middleware
                    model || m -> generate new model
            "#;
        }

        _ => println!("Empty"),
    }

    println!("{}", help);
}


fn generate(to_generate: &str, name: &str) -> Result<(), exitfailure::ExitFailure> {
    let mut content: String = String::new();
    let file_name: String;
    let dir: String;
    let content_res: (String, String);

    match to_generate {
        "r" | "route" => {
            content_res = helper::gen_route_content(&name);
            dir = format!("{}{}", ROOT, "/routes");
            println!("{}", content);
        }
        "c" | "controller" => {
            content_res = helper::gen_controller_content(&name);
            dir = format!("{}{}", ROOT, "/controllers");
            println!("{}", content);
        }
        "rs" | "request-schema" => {
            content_res = helper::gen_request_schema(&name);
            dir = format!("{}{}", ROOT, "/request-schemas");
            println!("{}", content);
        }
        "m" | "model" => {
            content_res = helper::gen_model(&name);
            dir = format!("{}{}", ROOT, "/models");
            println!("{}", content);
        }
        "mw" | "middleware" => {
            content_res = helper::gen_middleware_content(&name);
            dir = format!("{}{}", ROOT, "/middlewares");
            println!("{}", content);
        }
        "s" | "service" => {
            content_res = helper::gen_service_content(&name);
            dir = format!("{}{}", ROOT, "/services");
            println!("{}", content);
        }
        _ => {
            show_secondary_help("g");
            std::process::exit(0);
        }
    }

    content = content_res.0;
    file_name = content_res.1;

    let _path = format!("{}/{}", dir.clone(), file_name.as_str());

    let path = std::path::Path::new(&_path);

    if fs::metadata(path).is_err() {
        fs::create_dir_all(path.parent().unwrap())?;
    }

    let mut file = fs::File::create(path)?;

    file.write_all(&content.as_bytes())?;

    println!("{} created", dir);

    Ok(())
}

fn main() -> Result<(), exitfailure::ExitFailure> {
    let args: Vec<String> = std::env::args().collect();

    println!("Arguments Provided, {}", args.len());

    match args.len() {
        2 => show_secondary_help(&args[1]),

        3 => {
            println!("{}", "Invalid number of arguments".to_string().red());

            std::process::exit(1);
        }
        4 => match args[1].as_str() {
            "g" | "generate" => generate(&args[2], &args[3])?,
            _ => println!("Invalid"),
        },

        _ => show_help(),
    }

    Ok(())
}
