# rust_deserialize_config
Rust deserialization examples of common configuration file types for learning and profit

Current examples include passing json, toml and yaml files to strongly typed rust structs

To test run ./tests.sh or run the specific file in /src/bin with cargo run --bin 'filename' excluding the .rs

eg:
`cargo run --bin yaml`

Using serde tools apart from toml which, uses the `toml` package

## Why?
I found clarity on how to do this longwinded online. Documentation is excellent but I want a quick reference to
be able to refer to, and hopefully this helps you too.
