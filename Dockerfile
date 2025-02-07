FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/sum_root_leaf_numbers

COPY . .

# Build the Rust program
RUN cargo build --release

FROM ubuntu:22.04

# Set the working directory
WORKDIR /usr/src/sum_root_leaf_numbers

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/sum_root_leaf_numbers/target/release/sum_root_leaf_numbers .

# Run the executable
CMD ["./sum_root_leaf_numbers"]
