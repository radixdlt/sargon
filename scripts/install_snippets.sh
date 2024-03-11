#!/usr/bin/env zsh

set -e
set -u

me=$(basename "$0")
REL_DIR=$0:P
DIR="$( cd "$( dirname "$REL_DIR" )" && pwd )";
cd "$DIR" 

TARGET="$HOME/Library/Application\ Support/Code/User/snippets/"
eval "mkdir -p $TARGET"
eval "cp rust_snippets.code-snippets $TARGET"