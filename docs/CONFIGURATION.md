# Configuration

`auth-rs` can be configured using environment variables to fit your environment and requirements.

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

## Changing the default configuration

The default configuration can be changed by setting the environment variables before starting the service.  Alternatively, you can create a `.env` file in the root directory of the project and set the environment variables there.

An example `.env` file can be found in the root of the repository, called `.env.example`.
