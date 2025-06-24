#!/usr/bin/env bash
cd "$(dirname "$0")"

echo "https://helpx.adobe.com/pl/indesign/using/using-fonts.html" | ../target/release/twars-url2md --stdin -o out
