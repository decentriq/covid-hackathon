#!/usr/bin/env bash
set -euo pipefail

# Check if jq is installed
if ! [ -x "$(command -v jq)" ]; then
    echo "jq is not installed" >& 2
    exit 1
fi

# Clean pjg directory
rm -rf "pkg" && mkdir "pkg"

# Get the package name
PKG_NAME=$(jq -r .name pkg-bundler/package.json | sed 's/\-/_/g')

# Copy bundler package
cp -a "pkg-bundler/." "pkg/"

# Merge nodejs & bundler packages
cp "pkg-node/${PKG_NAME}.js" "pkg/${PKG_NAME}_main.js"
sed "s/require[\(]'\.\/${PKG_NAME}/require\('\.\/${PKG_NAME}_main/" "pkg-node/${PKG_NAME}_bg.js" > "pkg/${PKG_NAME}_bg.js"
jq ".files += [\"${PKG_NAME}_bg.js\"]" pkg/package.json \
    | jq ".main = \"${PKG_NAME}_main.js\"" > pkg/temp.json
mv pkg/temp.json pkg/package.json
