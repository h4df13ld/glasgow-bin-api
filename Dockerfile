# 1. This tells docker to use the Rust official image
FROM rust:latest

## TEST 1
# 2. Copy the files in your machine to the Docker image
#COPY ./ ./

# Build your program for release
#RUN cargo build --release

#RUN ls

#RUN ls -l /target
#RUN ls -l /target/release

# Run the binary
#CMD ["./target/release/rust_scraper"]


## TEST 2
# 1. create an empty shell project
#RUN USER=root cargo new --bin bindicator
#WORKDIR /bindicator

# 2. Copy our manifests
#COPY ./Cargo.lock ./Cargo.lock
#COPY ./Cargo.toml ./Cargo.toml
#COPY ./Rocket.toml ./Rocket.toml

# 3. Build only the dependancies
#RUN cargo build --release
#RUN rm src/*.rs

# 4. Now the dependency is built, copy the source code
#COPY ./src ./src

# 5. Build for release
#RUN rm ./target/release/deps/bindicator*
#RUN cargo install --path .

#CMD ["bindicator"] 


## TEST 3
#FROM rust:latest as build

# 1. create an empty shell project
#RUN USER=root cargo new --bin bindicator
#WORKDIR /bindicator

# 2. Copy our manifests
#COPY ./Cargo.lock ./Cargo.lock
#COPY ./Cargo.toml ./Cargo.toml
#COPY ./Rocket.toml ./Rocket.toml

# 3. Build only the dependancies
#RUN cargo build --release
#RUN rm src/*.rs

# 4. Now the dependency is built, copy the source code
#COPY ./src ./src

# 5. Build for release
#RUN rm ./target/release/deps/bindicator*
#RUN cargo install --path .

# 6. Our final release
#FROM rust:latest

# 7. Copy the build artifact from the build stage
#COPY --from=build /bindicator/target/release/bindicator .

# 8. Set the startup command to run the binary
#CMD ["./bindicator"] 

## TEST 2
# 1. Create an empty shell project
RUN USER=root cargo new --bin bindicator
WORKDIR /bindicator

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./Rocket.toml ./Rocket.toml

# 3. Build only the dependencies
RUN cargo build --release

# 4. Remove the dummy source files and copy the actual source code
RUN rm src/*.rs
COPY ./src ./src

# 5. Build the final release
RUN cargo build --release

# 6. Install the binary
RUN cargo install --path .

# Ensure the binary directory is in the PATH
#ENV PATH="/root/.cargo/bin:$PATH"

CMD ["cargo", "run"]
