cargo build --release \
  --target x86_64-pc-windows-gnu \
  --target x86_64-unknown-linux-gnu

cp ./target/x86_64-pc-windows-gnu/release/dark-souls-quick-save.exe ./target/dark-souls-quick-save_x86_64.exe &&
cp ./target/x86_64-unknown-linux-gnu/release/dark-souls-quick-save ./target/dark-souls-quick-save_x86_64
