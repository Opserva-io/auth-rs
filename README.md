# auth-rs

`auth-rs` provides a simple authentication and authorization service for use in other services.
The service is written in Rust and uses the [actix-web](https://crates.io/crates/actix-web) framework.

Users, when authenticated, will be given a JWT token which can be used to access other services.

## Features

- [ ] Authentication
- [ ] Authorization
- [X] Password hashing
- [X] JWT generation
- [ ] JWT verification
- [X] MongoDB integration
- [X] MongoDB Atlas integration

## Building

### cargo

You can build `auth-rs` using `cargo`:

```shell
cargo build
```

You can build a `release` version of `auth-rs` using `cargo`:

```shell
cargo build --release
```

## Dependencies

A couple of dependencies are required in order to build `auth-rs`:

* [actix-web](https://crates.io/crates/actix-web)
* [actix-cors](https://crates.io/crates/actix-cors)
* [uuid](https://crates.io/crates/uuid)
* [mongodb](https://crates.io/crates/mongodb)
* [chrono](https://crates.io/crates/chrono)
* [serde](https://crates.io/crates/serde)
* [serde_json](https://crates.io/crates/serde_json)
* [futures](https://crates.io/crates/futures)
* [dotenvy](https://crates.io/crates/dotenvy)
* [argon2](https://crates.io/crates/argon2)
* [regex](https://crates.io/crates/regex)
* [email_address](https://crates.io/crates/email_address)
* [jsonwebtoken](https://crates.io/crates/jsonwebtoken)
* [env_logger](https://crates.io/crates/env_logger)
* [log](https://crates.io/crates/log)

## About

This library is maintained by CodeDead. You can find more about us using the following links:
* [Website](https://codedead.com)
* [Twitter](https://twitter.com/C0DEDEAD)
* [Facebook](https://facebook.com/deadlinecodedead)

Copyright © 2023 CodeDead
