use random_word::{Lang, get};

/// Generate a random two-word deployment name (e.g. "bright-eagle")
pub fn generate_deployment_name() -> String {
    format!("{}-{}", get(Lang::En), get(Lang::En))
}
