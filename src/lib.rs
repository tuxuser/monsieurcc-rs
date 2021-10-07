pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub mod api;
pub mod schemas;

// Re-export serde
pub use serde;
pub use serde_json;

#[cfg(test)]
mod tests {
    use super::Result;
    use std::path::PathBuf;

    pub fn get_testdata(filename: &str) -> Result<Vec<u8>> {
        let mut filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        filepath.push("testdata");
        filepath.push(filename);
        Ok(std::fs::read(filepath)?)
    }
}
