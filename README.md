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

## Features

- [X] Authentication
- [X] Authorization
- [ ] Audit trail
- [X] Password hashing
- [X] JWT generation
- [X] JWT verification
- [X] MongoDB integration
- [X] MongoDB Atlas integration

## Usage

1. Clone the repository
2. Copy `.env.example` to `.env` in the root of the project and fill in / adjust the values
3. Execute `cargo run` to start the service

## Configuration

The following environment variables can be used to configure `auth-rs`:

| Variable                 | Default       | Type        | Description                                                          |
|--------------------------|---------------|-------------|----------------------------------------------------------------------|
| SERVER_ADDR              | `127.0.0.1`   | `IPAddress` | The server address                                                   |
| SERVER_PORT              | `8080`        | `u16`       | The port that the server will use                                    |
| DB_CONNECTION_STRING     | N/A           | `String`    | The MongoDB connection string                                        |
| DB_DATABASE              | N/A           | `String`    | The MongoDB Database that will be used by `auth-rs`                  |
| DB_PERMISSION_COLLECTION | `permissions` | `String`    | The collection that holds the `Permission` entities                  |
| DB_ROLE_COLLECTION       | `roles`       | `String`    | The collection that holds the `Role` entities                        |
| DB_USER_COLLECTION       | `users`       | `String`    | The collection that holds the `User` entities                        |
| DB_CREATE_INDEXES        | `true`        | `bool`      | Automatically create collection indexes                              |
| HASH_SALT                | N/A           | `String`    | The salt to use to hash passwords using `argon2`                     |
| JWT_SECRET               | N/A           | `String`    | The JWT secret                                                       |
| JWT_EXPIRATION           | `3600`        | `usize`     | The JWT expiration time in seconds                                   |
| RUST_LOG                 | N/A           | `String`    | The default log level                                                |
| RUST_BACKTRACE           | N/A           | `String`    | Controls whether or not backtraces are displayed when a panic occurs |
| GENERATE_DEFAULT_USER    | `true`        | `bool`      | Sets whether a default administrator user should be generated        |
| DEFAULT_USER_USERNAME    | N/A           | `String`    | The default user's username                                          |
| DEFAULT_USER_EMAIL       | N/A           | `String`    | The default user's email address                                     |
| DEFAULT_USER_PASSWORD    | N/A           | `String`    | The default user's password                                          |
| DEFAULT_USER_ENABLED     | N/A           | `bool`      | Sets whether the default user is enabled or not                      |

For more information about the environment variables, see the [Configuration documentation](/docs/CONFIGURATION.md).

## API

`auth-rs` exposes a REST API that can be used to interact with the service using Create, Read, Update and Delete (CRUD) requests.
Other (micro)services can use this API to authenticate and authorize users (and generate and verify JWT tokens).

See the [full API documentation](/docs/API.md) for more information.

### Authentication

#### Login

##### Request

```http
POST /authentication/login
{
  "username": "username",
  "password": "password"
}
```

##### Response

```http
{
  "token": "Bearer <access token here>"
}
```

#### Verify JWT and retrieve current user

##### Request

```http
GET /authentication/current
Authorization: Bearer <access token here>
```

##### Response

```http
{
  "id": "d594989b-48bd-43d8-ab3e-d28671f145e6",
  "username": "admin",
  "email": "test@codedead.com",
  "first_name": "Test",
  "last_name": "Test",
  "roles": [
    {
      "id": "16a639cc-2240-4d2f-8def-bea0a729dd9e",
      "name": "DEFAULT",
      "description": "The default role",
      "permissions": [
        {
          "id": "078bb9bf-21c4-4a5f-8f30-f7367a1de1b9",
          "name": "CAN_UPDATE_SELF",
          "description": "The ability to update your own user"
        }
      ]
    }
  ]
}
```

### Authorization

#### Permissions

##### Request

```http
GET /permissions
Authorization: Bearer <access token here>
```

##### Response

```http
[
  {
    "id": "26646655-e950-41ca-91cf-79dd94eb7b09",
    "name": "CAN_CREATE_PERMISSION",
    "description": "The ability to create permissions",
    "createdAt": "2023-08-01T00:16:25.216136358+00:00",
    "updatedAt": "2023-08-01T00:16:25.216136358+00:00"
  }
]
```

#### Roles

##### Request

```http
GET /roles
Authorization: Bearer <access token here>
```

##### Response

```http
[
  {
    "id": "16a639cc-2240-4d2f-8def-bea0a729dd9e",
    "name": "DEFAULT",
    "description": "The default role",
    "permissions": [
      {
        "id": "078bb9bf-21c4-4a5f-8f30-f7367a1de1b9",
        "name": "CAN_UPDATE_SELF",
        "description": "The ability to update your own user",
        "createdAt": "2023-08-01T00:16:26.911565688+00:00",
        "updatedAt": "2023-08-01T00:16:26.911565688+00:00"
      }
    ],
    "createdAt": "2023-08-01T00:16:27.223266792+00:00",
    "updatedAt": "2023-08-01T00:16:27.223266792+00:00"
  }
]
```

#### Users

##### Request

```http
GET /users
Authorization: Bearer <access token here>
```

##### Response

```http
[
  {
    "id": "d594989b-48bd-43d8-ab3e-d28671f145e6",
    "username": "admin",
    "email": "test@codedead.com",
    "firstName": "Test",
    "lastName": "Test",
    "roles": [
      {
        "id": "16a639cc-2240-4d2f-8def-bea0a729dd9e",
        "name": "DEFAULT",
        "description": "The default role",
        "permissions": [
          {
            "id": "078bb9bf-21c4-4a5f-8f30-f7367a1de1b9",
            "name": "CAN_UPDATE_SELF",
            "description": "The ability to update your own user",
            "createdAt": "2023-08-01T00:16:26.911565688+00:00",
            "updatedAt": "2023-08-01T00:16:26.911565688+00:00"
          }
        ],
        "createdAt": "2023-08-01T00:16:27.223266792+00:00",
        "updatedAt": "2023-08-01T00:16:27.223266792+00:00"
      }
    ],
    "createdAt": "2023-08-01T00:17:43.807272087+00:00",
    "updatedAt": "2023-08-01T00:17:43.807272087+00:00",
    "enabled": true
  }
]
```

### Searching

You can search for `users`, `roles` and `permissions` by using the `text` query parameter. Searching is case-insensitive.

#### Request

```http
GET /users/?text=admin
Authorization: Bearer <access token here>
```

#### Response

```http
[
  {
    "id": "d594989b-48bd-43d8-ab3e-d28671f145e6",
    "username": "admin",
    "email": "test@codedead.com",
    "firstName": "Test",
    "lastName": "Test",
    "roles": [
      {
        "id": "16a639cc-2240-4d2f-8def-bea0a729dd9e",
        "name": "DEFAULT",
        "description": "The default role",
        "permissions": [],
        "createdAt": "2023-08-01T00:16:27.223266792+00:00",
        "updatedAt": "2023-08-01T00:16:27.223266792+00:00"
      }
    ],
    "createdAt": "2023-08-01T00:17:43.807272087+00:00",
    "updatedAt": "2023-08-01T00:17:43.807272087+00:00",
    "enabled": true
  }
]
```

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

## Dependencies

A couple of dependencies are required in order to build `auth-rs`:

* [actix-web](https://crates.io/crates/actix-web)
* [actix-cors](https://crates.io/crates/actix-cors)
* [actix-web-grants](https://crates.io/crates/actix-web-grants)
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

Copyright © 2023 [CodeDead](https://codedead.com)
