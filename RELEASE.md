1. change the version value specified in your Cargo.toml

2. git tag <version>
   git push origin <version>  - this triggers github actions to publish executables

3. cargo publish --dry-run
   cargo publish              - this updates https://crates.io/
