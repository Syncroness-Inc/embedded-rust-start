#!/bin/bash
# A custom command that specifies what builds VScode should be linting...
cargo watch \
    --shell scripts/check/stm32f407.sh\
    --shell scripts/check/tests.sh