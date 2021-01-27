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
                    scaffold || s -> generate service model and controller
            "#;
        }

        _ => println!("Empty"),
    }

    println!("{}", help);
}

fn _generate_route(name: &str) -> Result<(), exitfailure::ExitFailure> {
    let content_res = helper::gen_route_content(&name);
    let dir = format!("{}{}", ROOT, "/routes");

    _write_file(&dir, content_res)?;

    Ok(())
}

fn _generate_controller(name: &str) -> Result<(), exitfailure::ExitFailure> {
    let content_res = helper::gen_controller_content(&name);
    let dir = format!("{}{}", ROOT, "/controllers");
    _write_file(&dir, content_res)?;

    Ok(())
}

fn _generate_service(name: &str) -> Result<(), exitfailure::ExitFailure> {
    let content_res = helper::gen_service_content(&name);
    let dir = format!("{}{}", ROOT, "/services");
    _write_file(&dir, content_res)?;

    Ok(())
}

fn _generate_model(name: &str) -> Result<(), exitfailure::ExitFailure> {
    let content_res = helper::gen_model(&name);
    let dir = format!("{}{}", ROOT, "/models");
    _write_file(&dir, content_res)?;

    Ok(())
}

fn _write_file(dir: &str, content_res: (String, String)) -> Result<(), exitfailure::ExitFailure> {
    let content = content_res.0;
    let file_name = content_res.1;

    let _path = format!("{}/{}", dir.clone(), file_name.as_str());

    let path = std::path::Path::new(&_path);

    if fs::metadata(path).is_err() {
        fs::create_dir_all(path.parent().unwrap())?;
    }

    let mut file = fs::File::create(path)?;

    file.write_all(&content.as_bytes())?;

    println!("{:?} created", path);

    Ok(())
}

fn generate(to_generate: &str, name: &str) -> Result<(), exitfailure::ExitFailure> {
    let mut content: String = String::new();
    let file_name: String;
    let mut dir: String = String::new();
    let mut content_res: (String, String) = (String::from(""), String::from(""));

    match to_generate {
        "r" | "route" => {
            _generate_route(&name)?;
        }
        "c" | "controller" => {
            _generate_controller(&name)?;
        }
        "rs" | "request-schema" => {
            content_res = helper::gen_request_schema(&name);
            dir = format!("{}{}", ROOT, "/request-schemas");
            println!("{}", content);
        }
        "sc" | "scaffold" => {
            _generate_controller(&name)?;
            _generate_model(&name)?;
            _generate_service(&name)?;
            _generate_route(&name)?;
        }
        "m" | "model" => {
            _generate_model(&name)?;
        }
        "mw" | "middleware" => {
            content_res = helper::gen_middleware_content(&name);
            dir = format!("{}{}", ROOT, "/middlewares");
            println!("{}", content);
        }
        "s" | "service" => {
            _generate_service(&name)?;
        }
        _ => {
            show_secondary_help("g");
            std::process::exit(0);
        }
    }
    _write_file(&dir, content_res)?;

    Ok(())
}

fn main() -> Result<(), exitfailure::ExitFailure> {
    let args: Vec<String> = std::env::args().collect();

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
