use std::env;
use std::fs;
use serde_yaml::Value;

fn set_env_vars_recursive(prefix: &str, value: &Value) {
    match value {
        Value::Mapping(map) => {
            for (key, val) in map {
                let new_prefix = if prefix.is_empty() {
                    key.as_str().unwrap().to_string()
                } else {
                    format!("{}.{}", prefix, key.as_str().unwrap())
                };
                set_env_vars_recursive(&new_prefix, val);
            }
        },
        Value::Sequence(seq) => {
            for (i, val) in seq.iter().enumerate() {
                let new_prefix = format!("{}.{}", prefix, i);
                set_env_vars_recursive(&new_prefix, val);
            }
        },
        _ => {
            let env_var_name = prefix;
            let env_var_value = value.as_str().unwrap_or_default().to_string();
            env::set_var(env_var_name, env_var_value);
        }
    }
}

pub fn load_yaml_to_env(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    let config: Value = serde_yaml::from_str(&contents)?;
    set_env_vars_recursive("", &config);
    Ok(())
}