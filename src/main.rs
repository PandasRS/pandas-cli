use clap::{App, Arg, SubCommand};
use include_dir::{include_dir, Dir};
use std::fs;
use std::io::{self, Write};

/// Directory containing the template files for generating project structure.
static TEMPLATE_DIR: Dir<'_> = include_dir!("src/templates");

/// Main entry point for the `pandas-cli` application.
fn main() {
    let matches = App::new("pandas-cli")
        .version("1.0")
        .author("Marcus Gomes <viniciusllgomes@gmail.com>")
        .about("CLI for PandasAPI")
        .subcommand(
            SubCommand::with_name("new")
                .about("Creates a new PandasAPI project")
                .arg(Arg::with_name("name")
                    .help("The name of the project")
                    .required(true)
                    .index(1))
        )
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generates new modules")
                .arg(Arg::with_name("type")
                    .help("The type of item to generate (module)")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("name")
                    .help("The name of the item to generate")
                    .required(true)
                    .index(2))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let project_name = matches.value_of("name").unwrap();
        create_new_project(project_name);
    } else if let Some(matches) = matches.subcommand_matches("generate") {
        let item_type = matches.value_of("type").unwrap();
        let item_name = matches.value_of("name").unwrap();

        match item_type {
            "module" => generate_module(item_name),
            _ => println!("Unknown type: {}", item_type),
        }
    }
}

/// Creates a new PandasAPI project with the given name.
///
/// # Arguments
///
/// * `name` - The name of the project.
///
/// # Panics
///
/// This function will panic if it encounters any error while creating directories or writing files.
///
/// # Example
///
/// ```
/// create_new_project("example_project");
/// ```
fn create_new_project(name: &str) {
    println!("Creating new project: {}", name);

    // Create project directories
    fs::create_dir_all(format!("{}/src/modules/hello", name)).expect("Error creating project directory");
    fs::create_dir_all(format!("{}/src/config", name)).expect("Error creating config directory");
    fs::create_dir_all(format!("{}/src/database", name)).expect("Error creating database directory");
    fs::create_dir_all(format!("{}/src/modules", name)).expect("Error creating modules directory");

    let cargo_toml_content = r#"
[package]
name = "{{project_name}}"
version = "0.1.0"
edition = "2021"

[dependencies]
rocket = "0.5.0-rc.3"
serde = { version = "1.0", features = ["derive"] }
rocket_okapi = { version = "0.8.0-rc.3", features = ["swagger"] }
schemars = "0.8.16"
mongodb = "2.8.0"
futures = "0.3.30"
tokio = "1.35.1"
async-trait = "0.1"

[dev-dependencies]
reqwest = "0.11"
"#;
    fs::write(format!("{}/Cargo.toml", name), cargo_toml_content.replace("{{project_name}}", name)).expect("Error writing Cargo.toml");

    let main_rs_content = r#"
#[macro_use] extern crate rocket;

use rocket_okapi::{openapi_get_routes, swagger_ui::*};

mod config;
mod database;
mod modules;

use crate::config::AppConfig;
use crate::database::establish_mongo_connection;
use crate::modules::hello::controller::{
    get_hello,
    okapi_add_operation_for_get_hello_, 
};

#[launch]
async fn rocket() -> _ {
    let config = AppConfig::new();
    let db = establish_mongo_connection(&config).await;

    rocket::build()
        .manage(db)
        .mount("/", openapi_get_routes![get_hello])
        .mount("/swagger-ui/", make_swagger_ui(&SwaggerUIConfig {
            url: "/openapi.json".to_owned(),
            ..Default::default()
        }))
}
"#;
    fs::write(format!("{}/src/main.rs", name), main_rs_content).expect("Error writing main.rs");

    let mod_rs_content = "pub mod hello;";
    fs::write(format!("{}/src/modules/mod.rs", name), mod_rs_content).expect("Error writing mod.rs");

    let hello_mod_content = "pub mod controller;";
    fs::write(format!("{}/src/modules/hello/mod.rs", name), hello_mod_content).expect("Error writing hello/mod.rs");

    let hello_controller_content = r#"
use rocket::serde::json::Json;
use rocket_okapi::openapi;

#[openapi]
#[get("/hello")]
pub async fn get_hello() -> Json<&'static str> {
    Json("Hello, world!")
}
"#;
    fs::write(format!("{}/src/modules/hello/controller.rs", name), hello_controller_content).expect("Error writing hello/controller.rs");

    let config_mod_content = format!(r#"
pub struct AppConfig {{
    pub mongo_url: String,
}}

impl AppConfig {{
    pub fn new() -> Self {{
        AppConfig {{
            mongo_url: "mongodb://localhost:27017/{}".to_string(),
        }}
    }}
}}
"#, name);
    fs::write(format!("{}/src/config/mod.rs", name), config_mod_content).expect("Error writing config/mod.rs");

    let database_mod_content = format!(r#"
use mongodb::{{Client, Database, options::ClientOptions}};
use crate::config::AppConfig;

pub async fn establish_mongo_connection(config: &AppConfig) -> Database {{
    let client_options = ClientOptions::parse(&config.mongo_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client.database("{}")
}}
"#, name);
    fs::write(format!("{}/src/database/mod.rs", name), database_mod_content).expect("Error writing database/mod.rs");
}

/// Generates a new module with the given name.
///
/// # Arguments
///
/// * `name` - The name of the module to generate.
///
/// # Panics
///
/// This function will panic if it encounters any error while creating directories or writing files.
///
/// # Example
///
/// ```
/// generate_module("example_module");
/// ```
fn generate_module(name: &str) {
    let params = prompt_for_parameters();
    let module_dir = format!("src/modules/{}", name);
    let controller_path = format!("{}/controller.rs", module_dir);
    let dto_path = format!("{}/dto.rs", module_dir);
    let schema_path = format!("{}/schema.rs", module_dir);
    let repository_path = format!("{}/repository.rs", module_dir);
    let service_path = format!("{}/service.rs", module_dir);
    let mod_path = format!("{}/mod.rs", module_dir);

    let mod_template = get_template_content("module_template.rs");
    let controller_template = get_template_content("controller_template.rs");
    let dto_template = get_template_content("dto_template.rs");
    let schema_template = get_template_content("schema_template.rs");
    let repository_template = get_template_content("repository_template.rs");
    let service_template = get_template_content("service_template.rs");

    // Create module directory
    fs::create_dir_all(&module_dir).expect("Error creating module directory");

    // Write templates to files
    write_template(&mod_path, &mod_template, name, &params);
    write_template(&controller_path, &controller_template, name, &params);
    write_template(&dto_path, &dto_template, name, &params);
    write_template(&schema_path, &schema_template, name, &params);
    write_template(&repository_path, &repository_template, name, &params);
    write_template(&service_path, &service_template, name, &params);

    // Update modules/mod.rs
    update_modules_mod_rs(name);

    // Update main.rs
    update_main_rs(name);
}

/// Prompts the user to enter parameters for the module.
///
/// # Returns
///
/// A vector of tuples containing parameter names and types.
///
/// # Example
///
/// ```
/// let params = prompt_for_parameters();
/// for (name, ty) in params {
///     println!("Parameter: {} of type {}", name, ty);
/// }
/// ```
fn prompt_for_parameters() -> Vec<(String, String)> {
    let mut params = Vec::new();
    loop {
        println!("Enter parameter name (or press Enter to finish):");
        let mut param_name = String::new();
        io::stdin().read_line(&mut param_name).expect("Failed to read line");
        let param_name = param_name.trim().to_string();
        if param_name.is_empty() {
            break;
        }

        println!("Enter parameter type (e.g., String, i32, bool):");
        let mut param_type = String::new();
        io::stdin().read_line(&mut param_type).expect("Failed to read line");
        let param_type = param_type.trim().to_string();

        params.push((param_name, param_type));
    }
    params
}

/// Gets the content of the specified template file.
///
/// # Arguments
///
/// * `template_name` - The name of the template file.
///
/// # Returns
///
/// The content of the template file as a string.
///
/// # Panics
///
/// This function will panic if it encounters any error while reading the template file.
///
/// # Example
///
/// ```
/// let content = get_template_content("module_template.rs");
/// ```
fn get_template_content(template_name: &str) -> String {
    let file = TEMPLATE_DIR.get_file(template_name).expect("Template not found");
    file.contents_utf8().expect("Error reading template").to_string()
}

/// Writes the template content to the specified destination file.
///
/// # Arguments
///
/// * `destination_path` - The path to the destination file.
/// * `template_content` - The content of the template.
/// * `name` - The name to replace in the template content.
/// * `params` - The parameters to include in the template.
///
/// # Panics
///
/// This function will panic if it encounters any error while writing the destination file.
///
/// # Example
///
/// ```
/// write_template("path/to/file.rs", "template content", "module_name", &params);
/// ```
fn write_template(destination_path: &str, template_content: &str, name: &str, params: &[(String, String)]) {
    let mut content = template_content
        .replace("{{module_name}}", name)
        .replace("{{ModuleName}}", &capitalize_first_letter(name));

    let params_struct = params.iter()
        .map(|(name, ty)| format!("    pub {}: {},", name, ty))
        .collect::<Vec<_>>()
        .join("\n");

    let params_struct_optional = params.iter()
        .map(|(name, ty)| format!("    pub {}: Option<{}>,", name, ty))
        .collect::<Vec<_>>()
        .join("\n");

    let params_impl = params.iter()
        .map(|(name, _)| format!("        {}: dto.{},", name, name))
        .collect::<Vec<_>>()
        .join("\n");

    let params_struct_update = params.iter()
        .map(|(name, _)| format!("            if let Some({}) = dto.{} {{ update_doc.insert(\"{}\", {}); }}", name, name, name, name))
        .collect::<Vec<_>>()
        .join("\n");

    content = content.replace("{{params_struct}}", &params_struct);
    content = content.replace("{{params_struct_optional}}", &params_struct_optional);
    content = content.replace("{{params_impl}}", &params_impl);
    content = content.replace("{{params_struct_update}}", &params_struct_update);

    fs::write(destination_path, content).expect("Error writing destination file");
}


/// Capitalizes the first letter of the given string.
///
/// # Arguments
///
/// * `s` - The string to capitalize.
///
/// # Returns
///
/// The string with the first letter capitalized.
///
/// # Example
///
/// ```
/// let capitalized = capitalize_first_letter("example");
/// assert_eq!(capitalized, "Example");
/// ```
fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Updates the `src/modules/mod.rs` file to include the new module.
///
/// # Arguments
///
/// * `module_name` - The name of the new module.
///
/// # Panics
///
/// This function will panic if it encounters any error while updating the file.
///
/// # Example
///
/// ```
/// update_modules_mod_rs("new_module");
/// ```
fn update_modules_mod_rs(module_name: &str) {
    let mod_rs_path = "src/modules/mod.rs";
    let mod_rs_content = format!("pub mod {};\n", module_name);
    fs::OpenOptions::new()
        .append(true)
        .open(mod_rs_path)
        .expect("Error opening modules/mod.rs")
        .write_all(mod_rs_content.as_bytes())
        .expect("Error writing to modules/mod.rs");
}

/// Updates the `src/main.rs` file to include the new module's routes.
///
/// # Arguments
///
/// * `module_name` - The name of the new module.
///
/// # Panics
///
/// This function will panic if it encounters any error while updating the file.
///
/// # Example
///
/// ```
/// update_main_rs("new_module");
/// ```
fn update_main_rs(module_name: &str) {
    let main_rs_path = "src/main.rs";

    // Import statement
    let import_statement = format!(
        "use crate::modules::{}::controller::{{create_{}, get_{}, get_{}_by_id, update_{}, delete_{}, okapi_add_operation_for_create_{}_, okapi_add_operation_for_get_{}_, okapi_add_operation_for_get_{}_by_id_, okapi_add_operation_for_update_{}_, okapi_add_operation_for_delete_{}_}};\n",
        module_name, module_name, module_name, module_name, module_name, module_name, module_name, module_name, module_name, module_name, module_name
    );

    // New methods to be added
    let new_methods = format!(
        "create_{}, get_{}, get_{}_by_id, update_{}, delete_{}",
        module_name, module_name, module_name, module_name, module_name
    );

    // Read main.rs
    let mut content = fs::read_to_string(main_rs_path).expect("Error reading main.rs");

    // Insert import statement
    content = content.replacen(
        "#[launch]\nasync fn rocket() -> _ {",
        &format!(
            "#[launch]\nasync fn rocket() -> _ {{\n    {}",
            import_statement
        ),
        1,
    );

    // Find existing mount statement and update it
    if let Some(start) = content.find(".mount(\"/\", openapi_get_routes![") {
        if let Some(end) = content[start..].find("])") {
            let end = start + end + 2;
            let existing_methods = &content[start + 32..end - 2]; // Capture existing methods
            let mut all_methods = existing_methods.to_string();

            if !all_methods.is_empty() {
                all_methods.push_str(", ");
            }
            all_methods.push_str(&new_methods);

            let new_mount_statement = format!(".mount(\"/\", openapi_get_routes![{}])", all_methods);
            content.replace_range(start..end, &new_mount_statement);
        }
    } else {
        let new_mount_statement = format!(
            ".mount(\"/\", openapi_get_routes![get_hello, {}])",
            new_methods
        );
        content = content.replacen(
            ".mount(\"/\", openapi_get_routes![get_hello])",
            &new_mount_statement,
            1,
        );
    }

    // Write back to main.rs
    fs::write(main_rs_path, content).expect("Error writing main.rs");
}
