#!/bin/bash
deno run --env-file \
    --allow-env \
    --allow-read \
    --allow-write \
    --allow-net \
    src/main/index.ts "$@"
