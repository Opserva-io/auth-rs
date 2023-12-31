# Configuration

`auth-rs` can be configured using environment variables to fit your environment and requirements.

## Table of contents

- [Environment variables](#environment-variables)
- [Changing the default configuration](#changing-the-default-configuration)
- [Docker](#docker)

## Environment variables

The following environment variables can be used to configure `auth-rs`:

| Variable                 | Default       | Required                                     | Type        | Description                                                             |
|--------------------------|---------------|----------------------------------------------|-------------|-------------------------------------------------------------------------|
| SERVER_ADDR              | `0.0.0.0`     | `false`                                      | `IPAddress` | The server address                                                      |
| SERVER_PORT              | `8080`        | `false`                                      | `u16`       | The port that the server will use                                       |
| SERVER_WORKERS           | `0`           | `false`                                      | `usize`     | Sets number of workers to start (per bind address).                     |
| MAX_FETCH_LIMIT          | `100`         | `false`                                      | `i64`       | The maximum amount of entity records that can be retrieved in one call  |
| DB_CONNECTION_STRING     | N/A           | `true`                                       | `String`    | The MongoDB connection string                                           |
| DB_DATABASE              | N/A           | `true`                                       | `String`    | The MongoDB Database that will be used by `auth-rs`                     |
| DB_PERMISSION_COLLECTION | `permissions` | `false`                                      | `String`    | The collection that holds the `Permission` entities                     |
| DB_ROLE_COLLECTION       | `roles`       | `false`                                      | `String`    | The collection that holds the `Role` entities                           |
| DB_USER_COLLECTION       | `users`       | `false`                                      | `String`    | The collection that holds the `User` entities                           |
| DB_AUDIT_COLLECTION      | `audits`      | `false`                                      | `String`    | The collection that holds the `Audit` entities                          |
| DB_CREATE_INDEXES        | `true`        | `false`                                      | `bool`      | Automatically create collection indexes                                 |
| DB_AUDIT_ENABLED         | `false`       | `false`                                      | `bool`      | Enable or disable audit trails                                          |
| DB_AUDIT_TTL             | `0`           | `false`                                      | `u64`       | Automatically remove `Audit` entities after a set amount of seconds     |
| JWT_SECRET               | N/A           | `true`                                       | `String`    | The JWT secret                                                          |
| JWT_EXPIRATION           | `3600`        | `false`                                      | `usize`     | The JWT expiration time in seconds                                      |
| RUST_LOG                 | N/A           | `false`                                      | `String`    | The default log level                                                   |
| RUST_BACKTRACE           | N/A           | `false`                                      | `String`    | Controls whether or not backtraces are displayed when a panic occurs    |
| GENERATE_DEFAULT_USER    | `true`        | `false`                                      | `bool`      | Sets whether a default administrator `User` should be generated         |
| DEFAULT_USER_USERNAME    | N/A           | `true` if `GENERATE_DEFAULT_USER` is enabled | `String`    | The default `User`'s username                                           |
| DEFAULT_USER_EMAIL       | N/A           | `false`                                      | `String`    | The default `User`'s email address                                      |
| DEFAULT_USER_PASSWORD    | N/A           | `true` if `GENERATE_DEFAULT_USER` is enabled | `String`    | The default `User`'s password                                           |
| DEFAULT_USER_ENABLED     | N/A           | `true` if `GENERATE_DEFAULT_USER` is enabled | `bool`      | Sets whether the default user is enabled or not                         |
| ENABLE_OPENAPI           | `true`        | `false`                                      | `bool`      | Enables or disables the OpenAPI endpoint                                |


> *Note*: `SERVER_WORKERS` will use the number of logical cores available on the system, if set to zero.

> *Note*: The audit trail feature is disabled by default and will have a noticeable performance impact when enabled.
> Audit trails can be set to expire automatically after a set amount of seconds by changing the `DB_AUDIT_TTL` environment variable
> to the desired amount of seconds (greater than zero), if the `DB_CREATE_INDEXES` variable is also enabled.

## Changing the default configuration

The default configuration can be changed by setting the environment variables before starting the service.
Alternatively, you can create a `.env` file in the root directory of the project and set the environment variables
there.

An example `.env` file can be found in the root of the repository, called `.env.example`.

## Docker

The `auth-rs` service can be executed using Docker. The `Dockerfile` is located in the root of the repository.
You can add environment variables when running the container using the `-e` flag.

An example of running the container using Docker:

```bash
docker run -d -p 8080:8080 -e DB_CONNECTION_STRING=mongodb://localhost:27017 -e DB_DATABASE=auth-rs -e -e JWT_SECRET=mysecret -e DEFAULT_USER_USERNAME=admin -e DEFAULT_USER_PASSWORD=secret -e DEFAULT_USER_ENABLED=true opserva/auth-rs
```

Alternatively, you can provide an `.env` file to the container using the `--env-file` flag:

```bash
docker run -d -p 8080:8080 --env-file .env opserva/auth-rs
```
