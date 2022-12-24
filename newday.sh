#!/bin/bash
echo "What day are you solving?"
read DAY_NUM

DIR="./src/day$DAY_NUM"
mkdir $DIR

touch "$DIR/part1.rs" "$DIR/part2.rs" "./input/day$DAY_NUM.txt" "./src/day$DAY_NUM.rs";
printf "pub mod part1;\npub mod part2;\n" >> ./src/day$DAY_NUM.rs