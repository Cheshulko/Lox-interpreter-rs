#!/bin/sh

set -e

cargo test --verbose --target-dir /tmp/lox/target
rm -rf /tmp/lox/target