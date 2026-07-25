#!/bin/bash
pnpm --dir ./frontend/sysinfo-frontend run build
mv ./frontend/dist ./frontend-dir
cargo run
