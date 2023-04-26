#!/bin/bash
kill "$(<~/pids/rust-fullstack-todolist-backend)" &&
cd ~/rust-fullstack-todolist/backend &&
cargo build --release > ~/logs/rust-fullstack-todolist-backend 2>&1 && (./target/release/rust-fullstack-todolist-backend > ~/logs/rust-fullstack-todolist-backend 2>&1 & echo $! > ~/pids/rust-fullstack-todolist-backend)