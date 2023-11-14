echo "Packaging universal app for MacOS"

# building for Apple Silicon
cargo build --release --target aarch64-apple-darwin

# building for Intel macs
cargo build --release --target x86_64-apple-darwin

# creating universal binary using lipo
lipo -create -output path_finding target/x86_64-apple-darwin/release/path_finding target/aarch64-apple-darwin/release/path_finding

#packaging into the mac app
mv path_finding ./build/astar.rs.app/Contents/MacOS/.
