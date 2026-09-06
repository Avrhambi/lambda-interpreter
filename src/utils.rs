use std::collections::HashSet;

/// Generates a list of possible variable names (a-z, A-Z)
fn generate_possible_variables() -> Vec<String> {
    let mut vars = Vec::new();
    // a-z
    for c in b'a'..=b'z' {
        vars.push((c as char).to_string());
    }
    // A-Z
    for c in b'A'..=b'Z' {
        vars.push((c as char).to_string());
    }
    vars
}

/// Generates a fresh variable name that is not in the `used_vars` set.
pub fn fresh_var(used_vars: &HashSet<String>) -> String {
    let possible = generate_possible_variables();
    for v in possible {
        if !used_vars.contains(&v) {
            return v;
        }
    }
    panic!("OutOfVariablesError");
}