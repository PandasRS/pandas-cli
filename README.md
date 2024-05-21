# PandasCLI

PandasCLI is a command-line tool designed to help developers quickly generate and manage Rust projects using the PandasAPI structure. It streamlines the setup of new projects, module creation, and other essential tasks to boost your productivity.

## Features

- Create new PandasAPI projects
- Generate modules with controllers, services, DTOs, schemas, and repositories
- Automatically set up project structure and dependencies
- Easy integration with MongoDB
- Pre-configured Swagger UI for API documentation

## Installation

To install PandasCLI, ensure you have Rust and Cargo installed on your system, then run:

```sh
cargo install pandas-cli
```

## Usage

### Create a New Project

To create a new PandasAPI project, run:

```sh
pandas-cli new project-name
```

This command sets up a new project with the specified name, including all necessary directories and files.

### Generate a New Module

To generate a new module within your PandasAPI project, run:

```sh
pandas-cli generate module module-name
```

This command creates a new module with the specified name, including controller, service, DTO, schema, and repository files.

## Example

Here's how you can use PandasCLI to create a new project and generate a module:

```sh
# Create a new project
pandas-cli new my-awesome-api

# Navigate to the project directory
cd my-awesome-api

# Generate a new module
pandas-cli generate module users
```

## License

PandasCLI is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contributing

Contributions are welcome! If you have any suggestions, bug reports, or feature requests, please open an issue or submit a pull request on the [GitHub repository](https://github.com/yourusername/pandas-cli).

## Contact

For any inquiries, please reach out to Marcus Gomes at viniciusllgomes@gmail.com.
