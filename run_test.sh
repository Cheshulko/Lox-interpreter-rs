#!/bin/sh

set -e

exec /tmp/lox/target/debug/lox-interpreter "$@"