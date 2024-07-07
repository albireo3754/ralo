#[allow(dead_code)]
pub fn get_env_string(key: &str) -> String {
    std::env::var(key).unwrap()
}

#[allow(dead_code)]
pub fn get_env_string_with_default(key: &str, default: String) -> String {
    std::env::var(key).unwrap_or(default)
}
