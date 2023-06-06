#!/bin/bash
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-unknown-linux-gnu/release/stackcell dist/x86_64-unknown-linux-gnu/stackcell
chmod +x dist/x86_64-unknown-linux-gnu/stackcell
cp target/x86_64-pc-windows-gnu/release/stackcell.exe dist/x86_64-pc-windows-gnu/stackcell.exe

cargo build --release --features u32 --target x86_64-unknown-linux-gnu
cargo build --release --features u32 --target x86_64-pc-windows-gnu
cp target/x86_64-unknown-linux-gnu/release/stackcell dist/x86_64-unknown-linux-gnu/stackcell-u32
chmod +x dist/x86_64-unknown-linux-gnu/stackcell-u32
cp target/x86_64-pc-windows-gnu/release/stackcell.exe dist/x86_64-pc-windows-gnu/stackcell-u32.exe

cargo build --release --bin assembler --target x86_64-unknown-linux-gnu
cargo build --release --bin assembler --target x86_64-pc-windows-gnu
cp target/x86_64-unknown-linux-gnu/release/assembler dist/x86_64-unknown-linux-gnu/assembler
chmod +x dist/x86_64-unknown-linux-gnu/assembler
cp target/x86_64-pc-windows-gnu/release/assembler.exe dist/x86_64-pc-windows-gnu/assembler.exe

# v2
cd new_vm || exit
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-unknown-linux-gnu/release/stackcell_v2 ../dist/x86_64-unknown-linux-gnu/stackcell2
chmod +x ../dist/x86_64-unknown-linux-gnu/stackcell2
cp target/x86_64-pc-windows-gnu/release/stackcell_v2.exe ../dist/x86_64-pc-windows-gnu/stackcell2.exe

cargo build --release --features u32 --target x86_64-unknown-linux-gnu
cargo build --release --features u32 --target x86_64-pc-windows-gnu
cp target/x86_64-unknown-linux-gnu/release/stackcell_v2 ../dist/x86_64-unknown-linux-gnu/stackcell2-u32
chmod +x ../dist/x86_64-unknown-linux-gnu/stackcell2-u32
cp target/x86_64-pc-windows-gnu/release/stackcell_v2.exe ../dist/x86_64-pc-windows-gnu/stackcell2-u32.exe
