#!/bin/bash

# Check if cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Cargo could not be found. Please install Rust."
    exit
fi

# Navigate to the project directory
cd "$(dirname "$0")"

# Build the project
cargo build --release

# Move the executable to /usr/local/bin
sudo mv ./target/release/iea /usr/local/bin/

echo "Installation completed successfully. You can now run the program with the command 'iea'."