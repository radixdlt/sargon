#!/usr/bin/env zsh

set -e
set -u

if grep -q "let useLocalFramework = false" Package.swift; then
    echo "You MUST let useLocalFramework be set to 'true'. Else Swift test will fail."
    exit 1;
fi

exit 0;