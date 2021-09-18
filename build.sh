cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-unknown-linux-gnu/release/stackcell dist/x86_64-unknown-linux-gnu/stackcell
cp target/x86_64-pc-windows-gnu/release/stackcell.exe dist/x86_64-pc-windows-gnu/stackcell.exe
