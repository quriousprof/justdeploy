use random_word::{Lang, get};

/// Generate a random deployment name
pub fn generate_deployment_name() -> String {
    format!("{}-{}", get(Lang::En), get(Lang::En)).to_string()
}
