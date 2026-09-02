#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
    echo "Usage: ./build.sh VERSION [dx build arguments...]" >&2
    exit 2
fi

export ASTTE_VERSION="$1"
shift
dx build "$@"