#!/bin/sh

set -ex

wasm-pack build --target web -d site/pkg
