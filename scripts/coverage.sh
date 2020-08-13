#!/bin/bash
# Chose Xml as output for integration with Jenkins/Pycobertura
cargo tarpaulin --exclude-files /usr/* --out Xml