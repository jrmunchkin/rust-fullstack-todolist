#!/bin/bash
cd ~/rust-fullstack-todolist/deploy &&
git pull > ~/logs/ust-fullstack-todolist-git-pull 2>&1 &
./run-backend.sh &&
./run-frontend.sh