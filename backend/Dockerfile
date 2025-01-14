FROM rust:1.84 as planner



WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM rust:1.84 as cacher
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y libssl-dev cmake build-essential pkg-config 
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.84 as builder
# Install necessary build tools
# RUN apt-get update && apt-get upgrade -y
# RUN apt-get install -y libssl-dev cmake build-essential pkg-config 
COPY . /app
WORKDIR /app
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

# Copy the manifest files into the container
# COPY ./Cargo.lock ./
# COPY ./Cargo.toml ./

# this build step will cache dependencies
# RUN cargo build --release && ls -l /app/target/release/

# RUN rm src/*.rs

# copy your source tree
# COPY ./src ./src

# build for release
# RUN rm ./target/release/deps/backend*
# RUN cargo build --release

# Create a new stage for the final image
FROM debian:bookworm-slim

# Update and install CMake
# RUN apt-get update && \
#     apt-get upgrade -y && \
#     apt-get install -y cmake && \
#     rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/backend /app/backend

# Set the command to run the backend application
CMD ["./app/backend"]