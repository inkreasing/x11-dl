echo "old solver"
cargo clean --workspace -p x11-dl
cargo +nightly check
# cargo clean
echo "new solver"
cargo clean --workspace -p x11-dl
RUSTFLAGS="-Znext-solver=globally" cargo +nightly check
# cargo clean
