# Blinky
A simple starting point for bare metal rust projects. This repository ships with an environment that allows cross compilation, flashing the target device, remote debugging, local unit testing, local coverage reports, and an example for continuous integration. The environment was created with a [STM32F429I Discovery Board](https://www.st.com/content/st_com/en/products/evaluation-tools/product-evaluation-tools/mcu-mpu-eval-tools/stm32-mcu-mpu-eval-tools/stm32-discovery-kits/32f429idiscovery.html) in mind but can be easily configured to accommodate a different device. The remainder of the readme shall provide instructions for how to get the environment up and running, how to use the tools, and provide explanation for how some environment features work. 

Note: Much of the environment setup has been automated using docker following the instructions in the [embedded rust discovery book](https://rust-embedded.github.io/discovery/).

# Requirements
 - A linux development environment
 - [Docker](https://docs.docker.com/engine/install/)
 - [VSCode](https://code.visualstudio.com/)
 - VSCode Extention: [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

The remaining instructions will assume you have these requirements correctly installed.

# Setup
1. In your linux environment, create a udev rule following the instructions [here](https://rust-embedded.github.io/discovery/03-setup/linux.html#udev-rules). 

    Notes
    - The udev rule allows the target device to be flashed without root privileges.
    - It is convenient, but not required, to create a symbolic link to your device using the `SYMLINK` key. This allows you to refer to your device under a static name rather than dynamic numbers.
    - The udev rule used for the ST on board debugger was specified in the `/etc/udev/rules.d/99-openocd.rules` file and is as follows
        ```
        # STM32F3DISCOVERY rev A/B - ST-LINK/V2
        ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3748", MODE:="0666", SYMLINK+="ST-LINK-V2"

        # STM32F3DISCOVERY rev C+ - ST-LINK/V2-1
        ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", MODE:="0666", SYMLINK+="ST-LINK-V2-1"
        ```
    Your udev rule may differ if you are not using ST Hardware.

2. Open VSCode.

3. Open the repository within a remote container by selecting the green `><` icon in the bottom left corner and select `Remote-Containers: Reopen in Container`. ![remote container](images/open_remote_container.png)

4. Select the directory containing the `Dockerfile` of this repository. VSCode should reopen and begin building the docker image for the environment. This can take up to 5 minutes the first time.

Congratulations. You should now have a fully functional environment.

# Environment Overview
Most of the configurable items within this repository are in the `Makefile.toml` file. This file simply serves as a way of centralizing most configurable items to a single file and allowing many tasks (such as linting and debugging) to require minimum setup and be highly configurable. Its also worth noting that many of these tasks within the `Makefile.toml` file simply invoke cargo, the rust build system, for more detail on how cargo and cargo make works, feel free to refer to the following documentation.

- https://doc.rust-lang.org/cargo/
- https://sagiegurari.github.io/cargo-make/

# Checking the project compiles
`cargo make check-all`

# Building
`cargo make build-stm`

# Running the executable on the target device
Simply plug in the device and run the `cargo make run-stm` task. If you are using a device other than the *stm32f407 discovery board* see the configurations file category (at the end of this readme), to see what else needs to change. You will also need to use a different hardware abstraction layer. It is suggested, but required, to use one of the hardware abstraction layers provided by the rust embedded work group. See 

 - https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates
- https://github.com/rust-embedded/awesome-embedded-rust#board-support-crates

# Unit testing
To compile and run unit tests on your local machine run the `cargo make test-local` task. This script serves as a convenience to the programmer and is also invoked within continuous integration.

# Debugging Remote Device
In VSCode, press the debugger icon, select `Debug Microcontroller`, and press the play button.
![debug microcontroller](images/debug_microcontroller.png)

This will load the program and stop at the entry point of the program.

# Debugging Unit Tests
Simply place a breakpoint and press the debug icon hovering over the test you wish to debug ![debug unit tests](images/debug_unit_tests.png)

# Configuration Files
A brief description of configuration files for the environment and what fields that may need to change from project to project.

- [.cargo/config.toml](https://doc.rust-lang.org/cargo/reference/config.html) - Specifies default arguments to cargo when you run `cargo build` and `cargo run`. 

    `rustflags` may need to change depending on the hardware abstraction layer you are using. Appropriate configuration of the `rustflags` can usually be found in the hardware abstraction layer readme.

- [devcontainer.json](https://aka.ms/vscode-remote/devcontainer.json) - Specifies how the docker container should be run and VSCode extentions to be installed.

    `extentions` may change between projects.

    `postStartCommand` calls the `cargo make startup-openocd` task on startup. Configurations for the `openocd` invocation will need to change depending on the hardware you are using. See
    https://github.com/ntfreak/openocd/blob/master/README for valid interface and target options.


- [.vscode/launch.json](https://go.microsoft.com/fwlink/?linkid=830387) - Stores configurations for running the debuggers.

- [.vscode/settings.json](https://rust-analyzer.github.io/manual.html) - Stores configuration options for rust-analyzer (process that runs in the background that provides a pleasurable ide experience).

- [.vscode/tasks.json](https://code.visualstudio.com/Docs/editor/tasks) - Stores build tasks for the debugger and launches a linter using `cargo make startup-watcher`.

- [Dockerfile](https://docs.docker.com/engine/reference/builder/) - Specifies tools to be installed in the environment.
- `memory.x` - information specifying memory layout of the board.
    
    `memory.x` for the board can usually be found on github. See...
    
    https://github.com/stm32-rs

    https://github.com/nrf-rs/nrf-hal

- `run.gdb` - Specifies initial gdb commands to run when `cargo run` is used.

# Known Limitations
You must opt into tasks running in the background for each VSCode workspace. This means auto linting will not be automatically enabled the first time you open this project in VSCode. To enable this, manually run the `Watch files for linting task` by pressing `ctrl+shift+P` -> `Tasks: run task` -> `Watch files for linting`. A pop up should appear in the bottom right saying
```txt
This folder has tasks (Watch files for linting) defined in 'tasks.json' that run automatically when you open this folder. Do you allow automatic tasks to run when you open this folder?
```
select `Allow and run`