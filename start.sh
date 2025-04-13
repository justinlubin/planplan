#!/bin/bash
cargo build
./run.sh; fswatch -o ~/Dropbox/notes/todo.txt | xargs -n1 -I{} ./run.sh
