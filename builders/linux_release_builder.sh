cargo build --release
cp target/release/pen release_files/unix/linux/core
cd release_files/unix/linux
sha256sum core > core.sha256
