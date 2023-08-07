# Use Debian as the base image for the first stage
FROM debian:bullseye AS base
WORKDIR /app/

# Set up the build environment using the Rust image
FROM rust:1.67-bullseye AS build
WORKDIR /build/

# Copy your application source code
COPY . .

# Build the application using Cargo
RUN cargo build --release

# Create the final stage based on the base image
FROM base AS final

# Copy the compiled binary from the build stage to the final image
COPY --from=build /build/target/release/taaskly /app/taaskly

# Define the command to run your application
CMD [ "/app/taaskly" ]


# Build and run
# docker build -t taaskly-app .
# docker run taaskly-app or docker run --rm taaskly-app