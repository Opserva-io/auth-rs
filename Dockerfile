# Use the latest Rust image to build the application
FROM rust:latest AS build

# Set the working directory
WORKDIR /usr/src/auth-rs

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# Copy the rest of the application code to the container
COPY src ./src

# Build the application
RUN cargo build --release

# Start a new stage for the runtime image
FROM debian:stable-slim

# Set the environment variables
ENV SERVER_ADDR=0.0.0.0
ENV SERVER_PORT=8080
ENV DB_CONNECTION_STRING=mongodb+srv://<username>:<password>@cluster.mongodb.net/?retryWrites=true&w=majority
ENV DB_DATABASE=test
ENV DB_PERMISSION_COLLECTION=permissions
ENV DB_ROLE_COLLECTION=roles
ENV DB_USER_COLLECTION=users
ENV DB_AUDIT_COLLECTION=audits
ENV DB_CREATE_INDEXES=true
ENV DB_AUDIT_ENABLED=false
ENV HASH_SALT=SGVsbG8sIHdvcmxkIQ
ENV JWT_SECRET=topSecretSecret
ENV JWT_EXPIRATION=3600
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1
ENV GENERATE_DEFAULT_USER=true
ENV DEFAULT_USER_USERNAME=admin
ENV DEFAULT_USER_EMAIL=admin@opserva.io
ENV DEFAULT_USER_PASSWORD=123456
ENV DEFAULT_USER_ENABLED=true
ENV MAX_FETCH_LIMIT=100

# Set the working directory
WORKDIR /usr/src/auth-rs

# Copy the binary from the build stage to the current stage
COPY --from=build /usr/src/auth-rs/target/release/auth-rs .

# Expose any ports your application might listen on (optional)
EXPOSE 8080

# Set the binary as the entrypoint of the container
CMD ["./auth-rs"]
