#!/bin/bash
echo "What day are you solving?"
read DAY_NUM

DIR="./src/day$DAY_NUM"
mkdir $DIR
touch "$DIR/part1.rs" "$DIR/part2.rs" "$DIR/input.txt" "$DIR/demo.txt"