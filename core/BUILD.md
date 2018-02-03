# Initial stuff

    rustup toolchain install nightly
    rustup update
    rustup target add wasm32-unknown-unknown --toolchain nightly
    cargo install --git https://github.com/alexcrichton/wasm-gc

# Build

    cargo +nightly build --target wasm32-unknown-unknown --release

# Minimize

    wasm-gc sodium_rust_demo.wasm sodium_rust_demo_min.wasm
