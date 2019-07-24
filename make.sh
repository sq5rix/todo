cargo build --release
cargo build --release --target x86_64-pc-windows-gnu
cp target/release/todo ~/.cargo/bin
git push -f -u origin master
