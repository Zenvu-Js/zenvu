//! `Zenvu test` - Component testing framework
use anyhow::Result;
use zenvu_test::{run_tests, TestConfig};

pub async fn run(pattern: Option<&str>, watch: bool) -> Result<()> {
    let config = TestConfig {
        pattern: pattern.unwrap_or("").to_string(),
        watch,
    };
    run_tests(&config).await
}
