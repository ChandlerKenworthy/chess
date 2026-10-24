# Build & Install Instructions

# Build everything in the workspace
cargo build

# Run the backend
cargo run -p chess-backend

# Test the engine
cargo test -p chess-engine

# Test the entire workspace
cargo test --workspace

# Run engine benchmarks
cargo bench -p chess-engine

cargo run -p chess-engine --example testing