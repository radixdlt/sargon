#!/bin/sh

set -e
set -u

sh base_display_code_cov.sh report
sh base_display_code_cov.sh export -format="lcov" > $1