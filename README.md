# auth-rs

![GitHub top language](https://img.shields.io/github/languages/top/Opserva-io/auth-rs)
![GitHub](https://img.shields.io/github/license/Opserva-io/auth-rs)
![GitHub release (with filter)](https://img.shields.io/github/v/release/Opserva-io/auth-rs)
[![Test](https://github.com/Opserva-io/auth-rs/actions/workflows/test.yml/badge.svg)](https://github.com/Opserva-io/auth-rs/actions/workflows/test.yml)

`auth-rs` provides a simple authentication and authorization service for use in other services.
The service is written in Rust and uses the [actix-web](https://crates.io/crates/actix-web) framework.

Users, when authenticated, will be given a JWT token which can be used to access other services.

A [React](https://react.dev/)-based frontend for `auth-rs` is also
available [here](https://github.com/Opserva-io/auth-js).

## Table of Contents

- [Features](#features)
- [Usage](#usage)
  - [Development](#development)
  - [Docker](#docker)
- [Configuration](#configuration)
- [API](#api)
- [Building](#building)
- [Dependencies](#dependencies)
- [About](#about)

## Features

- Authentication
- Authorization
- Audit trail
- Password hashing
- JWT generation
- JWT verification
- Pagination
- OpenAPI / Swagger UI
- CORS support
- Docker support
- MongoDB integration
- MongoDB Atlas integration

## Usage

### Development

1. Clone the repository
2. Copy `.env.example` to `.env` in the root of the project and fill in / adjust the values
3. Execute `cargo run` to start the service

### Docker

You can execute `auth-rs` using Docker:

```bash
docker run -d -p 8080:8080 --env-file .env opserva/auth-rs
```

## Configuration

`auth-rs` can be configured using environment variables to fit your environment and requirements. 

For more information about the environment variables, see the [Configuration documentation](/docs/CONFIGURATION.md).

## API

`auth-rs` exposes a REST API that can be used to interact with the service using Create, Read, Update and Delete (CRUD) requests.
Other (micro)services can use this API to authenticate and authorize users (and generate and verify JWT tokens).

See the [full API documentation](/docs/API.md) for more information.

## Building

In order to build `auth-rs`, you will need to have Rust installed.
You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

### cargo

You can build `auth-rs` using `cargo`:

```shell
cargo build
```

You can build an optimized `release` version of `auth-rs` using `cargo`:

```shell
cargo build --release
```

### Docker

You can build a docker image of `auth-rs` using the provided `Dockerfile`:

```shell
docker build -t auth-rs .
```

## Dependencies

A couple of dependencies are required in order to build `auth-rs`:

* [actix-web](https://crates.io/crates/actix-web)
* [actix-cors](https://crates.io/crates/actix-cors)
* [actix-web-grants](https://crates.io/crates/actix-web-grants)
* [mongodb](https://crates.io/crates/mongodb)
* [chrono](https://crates.io/crates/chrono)
* [serde](https://crates.io/crates/serde)
* [serde_json](https://crates.io/crates/serde_json)
* [futures](https://crates.io/crates/futures)
* [dotenvy](https://crates.io/crates/dotenvy)
* [argon2](https://crates.io/crates/argon2)
* [regex](https://crates.io/crates/regex)
* [jsonwebtoken](https://crates.io/crates/jsonwebtoken)
* [env_logger](https://crates.io/crates/env_logger)
* [log](https://crates.io/crates/log)
* [utoipa](https://crates.io/crates/utoipa)
* [utoipa-swagger-ui](https://crates.io/crates/utoipa-swagger-ui)

## About

This library is maintained by CodeDead. You can find more about us using the following links:

* [Website](https://codedead.com)
* [Twitter](https://twitter.com/C0DEDEAD)
* [Facebook](https://facebook.com/deadlinecodedead)

Copyright © 2023 [CodeDead](https://codedead.com)
