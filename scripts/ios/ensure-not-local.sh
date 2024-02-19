#!/usr/bin/env zsh

set -e
set -u

if grep -q "let useLocalFramework = true" Package.swift; then
    echo "You MUST let useLocalFramework be set to 'false'. Else Swift release will fail."
    exit 1;
fi

exit 0;