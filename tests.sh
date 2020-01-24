#!/bin/bash

CARGO=$(command -v cargo)

if [ -z $CARGO ]; then
  echo "No cargo binary found with which, exiting"
  exit 1
fi

echo "Running toml"
$CARGO run --bin toml && echo "TOML Success" || echo "TOML run failed"

echo "Running yaml"
$CARGO run --bin yaml && echo "YAML Success" || echo "YAML run failed"

echo "Running json"
$CARGO run --bin json && echo "JSON Success" || echo "JSON run failed"