#!/usr/bin/env bash
# Clone test corpora for smoke testing and benchmarks.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CORPORA_DIR="$SCRIPT_DIR/../benchmark/corpora"

mkdir -p "$CORPORA_DIR"

clone_if_missing() {
    local name="$1"
    local url="$2"
    local dest="$CORPORA_DIR/$name"
    if [ -d "$dest" ]; then
        echo "  $name already present, skipping"
    else
        echo "  Cloning $name..."
        git clone --depth 1 "$url" "$dest"
    fi
}

echo "Bootstrapping test corpora into $CORPORA_DIR/"
clone_if_missing "requests" "https://github.com/psf/requests.git"
clone_if_missing "flask"    "https://github.com/pallets/flask.git"
clone_if_missing "rich"     "https://github.com/Textualize/rich.git"
clone_if_missing "pytest"   "https://github.com/pytest-dev/pytest.git"
clone_if_missing "click"    "https://github.com/pallets/click.git"
clone_if_missing "httpx"    "https://github.com/encode/httpx.git"
clone_if_missing "black"    "https://github.com/psf/black.git"
clone_if_missing "pydantic" "https://github.com/pydantic/pydantic.git"
clone_if_missing "fastapi"  "https://github.com/fastapi/fastapi.git"
echo "Done."
