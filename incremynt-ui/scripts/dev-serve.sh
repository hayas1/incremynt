#! /bin/sh -e

scripts=$(dirname "$(realpath "$0")")
"$scripts"/setup.sh

crate=$(dirname "$(dirname "$(realpath "$0")")")
repo=$(dirname "$crate")

PUBLIC_URL="${PUBLIC_URL:-"/incremynt/"}"
cd "$crate" && trunk serve --dist "$repo"/target/public --port 8080 --public-url "$PUBLIC_URL"
