# Using offical Rust container
FROM rust:1.45.2

# Installing global tools that require root privalges.
# Tools are mentioned in https://rust-embedded.github.io/discovery/03-setup/linux.html
RUN apt-get update
RUN apt-get install -y \ 
    gdb-multiarch=8.2.1-2+b3 \
    minicom=2.7.1-1+b1 \
    openocd=0.10.0-5

# Other useful tools developers may want.
RUN apt-get install -y \
    htop=2.2.0-1+b1 \
    curl=7.64.0-4+deb10u1 \
    vim=2:8.1.0875-5
    
# Another text editor that is much more intuitve
WORKDIR /usr/bin
RUN curl https://getmic.ro | bash
RUN apt-get install xclip -y

# Creating a soft link so a vscode gdb debugger doesn't get confused with naming
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

# Tool for the rust analyzer
RUN cargo install cargo-watch --vers 7.5.0

# Not needed when working within VSCode but nice when debugging from terminal.
WORKDIR /workspaces/blinky