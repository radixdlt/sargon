#!/bin/sh

set -e
set -u

REL_DIR=$0:P
DIR="$( cd "$( dirname "$REL_DIR" )" && pwd )";

sh $DIR/base_display_code_cov.sh "report"