#!/usr/bin/env zsh

set -e
set -u

me=$(basename "$0")
REL_DIR=$0:P
DIR="$( cd "$( dirname "$REL_DIR" )" && pwd )";


ZIP_PATH=$(sh $DIR/build-sargon.sh --release | tail -n 1)
`git fetch --prune --tags`
function last_tag() {
    local out=`git tag --sort=taggerdate | tail -1`
    echo $out
}
echo "🔮 Last tag: $(last_tag)"

# one liner from: https://stackoverflow.com/a/8653732
NEXT_TAG=$(echo $(last_tag) | awk -F. -v OFS=. 'NF==1{print ++$NF}; NF>1{if(length($NF+1)>length($NF))$(NF-1)++; $NF=sprintf("%0*d", length($NF), ($NF+1)%(10^length($NF))); print}')

`git tag $NEXT_TAG`
echo "🔮 Last tag: $(last_tag)"
RELEASE_CMD="gh release create $NEXT_TAG $ZIP_PATH"
echo "📦 will now run command: '$RELEASE_CMD'"
eval $RELEASE_CMD

echo "End of script ✅"