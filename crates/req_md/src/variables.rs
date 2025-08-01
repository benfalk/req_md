use std::collections::HashMap;
use std::env;
use regex::Regex;

#[derive(Debug)]
pub struct Variables {
    vars: HashMap<String, String>,
    envs: HashMap<String, String>,
}

impl Variables {
    pub fn new(input: &str) -> Self {
        let mut vars = HashMap::new();
        let mut envs = HashMap::new();

        for (key, val) in env::vars() {
            envs.insert(
                ["$", key.trim()].concat().to_string(),
                val
            );
        }

        let matcher = Regex::new(r"^([^:\s]+):(.+)$").unwrap();
        for line in input.lines() {
            if let Some(cap) = matcher.captures(line) {
                vars.insert(
                    ["$", cap[1].trim()].concat().to_string(),
                    cap[2].trim().to_string()
                );
            }
        }

        Self {
            vars,
            envs,
        }
    }

    pub fn expand(&self, input: &str) -> String {
        let mut string = input.to_owned();

        for (var, val) in &self.vars {
            string = string.replace(var, val);
        }

        for (var, val) in &self.envs {
            string = string.replace(var, val);
        }

        string
    }
}
