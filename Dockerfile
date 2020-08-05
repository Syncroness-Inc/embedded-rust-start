# Using offical Rust container
FROM rust:1.45.2

# Installing global tools that require root privalges.
# Tools are mentioned in https://rust-embedded.github.io/discovery/03-setup/linux.html
RUN apt-get update
RUN apt-get install -y \
    gdb-multiarch=8.2.1-2+b3 \
    minicom=2.7.1-1+b1 \
    openocd=0.10.0-5

# Setting the user to non-root privlages
RUN useradd --create-home --shell /bin/bash rustacean
USER rustacean

# Installing tools mentioned in the rust discovery book
# https://rust-embedded.github.io/discovery/03-setup/index.html
RUN rustup component add llvm-tools-preview
RUN rustup target add thumbv7em-none-eabihf
RUN cargo install itm --vers 0.3.1
RUN cargo install cargo-binutils --vers 0.3.1

WORKDIR /home/rustacean