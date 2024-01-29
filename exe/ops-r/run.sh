#!/bin/bash

set -eo pipefail

out=$(cargo run -p retrowin32 -- ops.exe | tr -d '\r')
diff -u out.txt <(echo "$out")
