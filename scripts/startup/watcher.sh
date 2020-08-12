#!/bin/bash
# A custom command that specifies what builds VScode should be linting...
cargo watch \
    --shell scripts/check/all.py