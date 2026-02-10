

# ===== Create workspace root =====

mkdir -p my_project
cd my_project
touch Cargo.toml

# ===== Write workspace config =====

echo "[workspace]
members = [
    "crates/core-lib",
    "crates/wasm-wrapper",
    "crates/server-app",
    "crates/proto-defs"
]
"
echo "$ECHO_OUTPUT" > Cargo.toml

# ===== Make crates dir =====

mkdir -p crates

# ===== Create crates =====

cargo new crates/core-lib --lib
cargo new crates/wasm-wrapper --lib
cargo new crates/server-app --bin
cargo new crates/proto-defs

# ===== Write proto-defs/build.rs =====

echo "fn main() {
    tonic_build::compile_protos("proto/service.proto").unwrap();
}
"
echo "$ECHO_OUTPUT" > crates/proto-defs/build.rs

# ===== Make proto folder =====

mkdir -p crates/proto-defs/proto
touch crates/proto-defs/proto/service.proto


# ===== Create workspace root =====

mkdir -p my_project
cd my_project
touch Cargo.toml

# ===== Write workspace config =====

echo "[workspace]
members = [
    "crates/core-lib",
    "crates/wasm-wrapper",
    "crates/server-app",
    "crates/proto-defs"
]
"
echo "$ECHO_OUTPUT" > Cargo.toml

# ===== Make crates dir =====

mkdir -p crates

# ===== Create crates =====

cargo new crates/core-lib --lib
cargo new crates/wasm-wrapper --lib
cargo new crates/server-app --bin
cargo new crates/proto-defs

# ===== Write proto-defs/build.rs =====

echo "fn main() {
    tonic_build::compile_protos("proto/service.proto").unwrap();
}
"
echo "$ECHO_OUTPUT" > crates/proto-defs/build.rs

# ===== Make proto folder =====

mkdir -p crates/proto-defs/proto
touch crates/proto-defs/proto/service.proto


# ===== Create workspace root =====

mkdir -p my_project
cd my_project
touch Cargo.toml

# ===== Write workspace config =====

echo "[workspace]
members = [
    "crates/core-lib",
    "crates/wasm-wrapper",
    "crates/server-app",
    "crates/proto-defs"
]
"
echo "$ECHO_OUTPUT" > Cargo.toml

# ===== Make crates dir =====

mkdir -p crates

# ===== Create crates =====

cargo new crates/core-lib --lib
cargo new crates/wasm-wrapper --lib
cargo new crates/server-app --bin
cargo new crates/proto-defs

# ===== Write proto-defs/build.rs =====

echo "fn main() {
    tonic_build::compile_protos("proto/service.proto").unwrap();
}
"
echo "$ECHO_OUTPUT" > crates/proto-defs/build.rs

# ===== Make proto folder =====

mkdir -p crates/proto-defs/proto
touch crates/proto-defs/proto/service.proto


# ===== Create workspace root =====

mkdir -p my_project
cd my_project
touch Cargo.toml

# ===== Write workspace config =====

echo "[workspace]
members = [
    "crates/core-lib",
    "crates/wasm-wrapper",
    "crates/server-app",
    "crates/proto-defs"
]
"
echo "$ECHO_OUTPUT" > Cargo.toml

# ===== Make crates dir =====

mkdir -p crates

# ===== Create crates =====

cargo new crates/core-lib --lib
cargo new crates/wasm-wrapper --lib
cargo new crates/server-app --bin
cargo new crates/proto-defs

# ===== Write proto-defs/build.rs =====

echo "fn main() {
    tonic_build::compile_protos("proto/service.proto").unwrap();
}
"
echo "$ECHO_OUTPUT" > crates/proto-defs/build.rs

# ===== Make proto folder =====

mkdir -p crates/proto-defs/proto
touch crates/proto-defs/proto/service.proto


# ===== Create workspace root =====

mkdir -p my_project
cd my_project
touch Cargo.toml

# ===== Write workspace config =====

echo "[workspace]
members = [
    "crates/core-lib",
    "crates/wasm-wrapper",
    "crates/server-app",
    "crates/proto-defs"
]
"
echo "$ECHO_OUTPUT" > Cargo.toml

# ===== Make crates dir =====

mkdir -p crates

# ===== Create crates =====

cargo new crates/core-lib --lib
cargo new crates/wasm-wrapper --lib
cargo new crates/server-app --bin
cargo new crates/proto-defs

# ===== Write proto-defs/build.rs =====

echo "fn main() {
    tonic_build::compile_protos("proto/service.proto").unwrap();
}
"
echo "$ECHO_OUTPUT" > crates/proto-defs/build.rs

# ===== Make proto folder =====

mkdir -p crates/proto-defs/proto
touch crates/proto-defs/proto/service.proto
