# Using offical Rust container
FROM rust:1.45.2

# Installing global tools that require root privalges.
# Tools are mentioned in https://rust-embedded.github.io/discovery/03-setup/linux.html
RUN apt-get update
RUN apt-get install -y \
    gdb-multiarch=8.2.1-2+b3 \
    minicom=2.7.1-1+b1 \
    openocd=0.10.0-5 \
    htop=2.2.0-1+b1

# Creating a soft link so a vscode debugger doesn't get confused with naming
RUN ln -s /usr/bin/gdb-multiarch /usr/bin/arm-none-eabi-gdb

# Setting the user to non-root privlages
RUN useradd --create-home --shell /bin/bash rustacean
USER rustacean

# Installing tools mentioned in the rust discovery book
# https://rust-embedded.github.io/discovery/03-setup/index.html
RUN rustup component add llvm-tools-preview
RUN rustup target add thumbv7em-none-eabihf
RUN cargo install itm --vers 0.3.1
RUN cargo install cargo-binutils --vers 0.3.1

# For debugging only
RUN cargo install cargo-expand --vers 1.0.0
RUN rustup install nightly 

# Needed to have gdb automatically flash the device. This allows the user to do `cargo run` 
# and have it automatically start running the application on the target.
RUN echo "add-auto-load-safe-path /workspaces/blinky/.gdbinit" > /home/rustacean/.gdbinit

# Not needed when working within VSCode but nice when debugging from terminal.
WORKDIR /workspaces/blinky