#!/usr/bin/env zsh

set -e
set -u

cargo ws version patch --allow-branch $(git symbolic-ref --short HEAD) --yes --no-git-tag --no-git-push