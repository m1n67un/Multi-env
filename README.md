# Multi-env

This project is an environment variable branching library written in Rust. It helps users easily manage configurations for various environments (development, testing, production, etc.).

## Features

- Support for environment-specific configuration files (.env, YAML, etc.)
- Environment specification through command-line arguments
- Easy-to-use API

## Usage

1. Add dependency:

```toml
[dependencies]
multi_env = "0.0.1"
```

2. Use in your code

```rust
use multi_env::set_ok;

fn main() {
    // Set the configuration type to "env" (uses .env files)
    set_ok("env");

    // Set the configuration type to "yml" (uses YAML files with .yml extension)
    // set_ok("yml");

    // Set the configuration type to "yaml" (uses YAML files with .yaml extension)
    // set_ok("yaml");
}
```
In this example, "env" specifies the configuration file format to use. "env" uses .env files, while "yaml" uses YAML files.

3. Run the program
```bash
cargo run -- -b development
```
The -b flag specifies the environment (branch) to use. In this example, the "development" environment is used.

# Examples
For more detailed usage examples, please refer to the examples directory in this repository. It contains sample projects demonstrating various use cases and configurations:
- basic_usage: Shows the basic setup and usage of the library
- yaml_config: Demonstrates how to use YAML configuration files
- multiple_environments: Illustrates handling multiple environment configurations

To run an example, navigate to its directory and use:
```bash
cargo run -- -b <environment_name>
```

Replace <environment_name> with the desired environment (e.g., development, production).

## Notes
- When running, you must specify the environment with the -b flag.
- Supported configuration file formats: .env, YAML (default is .env)

## Contributing
Bug reports, feature requests, and pull requests are always welcome!