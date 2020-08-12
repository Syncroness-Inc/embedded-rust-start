#!/bin/bash
cargo check \
    --target thumbv7em-none-eabihf \
    --features stm32f407

# Return the exit status of the last run command
exit $?