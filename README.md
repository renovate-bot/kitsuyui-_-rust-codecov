# rust-codecov

[![Crates.io](https://img.shields.io/crates/d/codecov)](https://crates.io/crates/codecov)
[![codecov](https://codecov.io/gh/kitsuyui/rust-codecov/branch/main/graph/badge.svg?token=0OM9KWFZQC)](https://codecov.io/gh/kitsuyui/rust-codecov)
[![crates.io](https://img.shields.io/crates/v/codecov.svg)](https://crates.io/crates/codecov)

## Description

A thin wrapper for Codecov API (v2).
https://docs.codecov.com/reference/overview

## Usage

```rust
use codecov::{Client, Owner};

// let client = Client::new("1234-5678-9012-3456");
let client = Client::new_from_env();  // Read from CODECOV_OWNER_TOKEN
let owner = Owner {
    service: "github".to_string(),
    username: "codecov".to_string(),
};
let repos = client.get_all_repos(&owner).unwrap();
println!("{:?}", repos)
```

## LICENSE

BSD-3-Clause
