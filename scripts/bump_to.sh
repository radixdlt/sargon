#!/usr/bin/env zsh

set -e
set -u

cargo ws version custom --exact $1 --no-git-commit --yes