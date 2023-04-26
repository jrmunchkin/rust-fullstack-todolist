#!/bin/bash
kill "$(<~/pids/rust-fullstack-todolist-frontend)" &&
cd ~/rust-fullstack-todolist/frontend &&
TRUNK_BUILD_RELEASE=true
trunk serve > ~/logs/rust-fullstack-todolist-frontend 2>~/logs/rust-fullstack-todolist-frontend & echo $! > ~/pids/rust-fullstack-todolist-frontend