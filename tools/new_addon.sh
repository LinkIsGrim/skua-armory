#!/usr/bin/env bash
# Create a new addon by copying template/addon and substituting the name.
# Usage: tools/new_addon.sh <addon_name>
set -euo pipefail

if [[ $# -ne 1 || -z "$1" ]]; then
    echo "Usage: $0 <addon_name>" >&2
    exit 1
fi

ADDON_NAME="$1"

if [[ ! "$ADDON_NAME" =~ ^[a-z][a-z0-9_]*$ ]]; then
    echo "Error: addon name must be lowercase [a-z0-9_] starting with a letter" >&2
    exit 1
fi

REPO_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
SRC="$REPO_ROOT/template/addon"
DEST="$REPO_ROOT/addons/$ADDON_NAME"

if [[ ! -d "$SRC" ]]; then
    echo "Error: template not found at $SRC" >&2
    exit 1
fi

if [[ -e "$DEST" ]]; then
    echo "Error: $DEST already exists" >&2
    exit 1
fi

ADDON_BEAUTIFIED=""
IFS='_' read -ra _parts <<< "$ADDON_NAME"
for _p in "${_parts[@]}"; do
    [[ -z "$_p" ]] && continue
    ADDON_BEAUTIFIED+="$(tr '[:lower:]' '[:upper:]' <<< "${_p:0:1}")${_p:1} "
done
ADDON_BEAUTIFIED="${ADDON_BEAUTIFIED% }"

cp -r "$SRC" "$DEST"

while IFS= read -r -d '' file; do
    sed -i \
        -e "s/blank/$ADDON_NAME/g" \
        -e "s/Blank/$ADDON_BEAUTIFIED/g" \
        "$file"
done < <(find "$DEST" -type f -print0)

echo "Created addon at $DEST"
