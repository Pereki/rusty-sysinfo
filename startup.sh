#!/bin/bash
pnpm --dir ./frontend run build
mv ./frontend/dist ./frontend-dir
cargo run
