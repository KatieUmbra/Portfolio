#!/bin/sh

(cd ./client && bun run dev) &
(cd ./server && cargo run)   &
