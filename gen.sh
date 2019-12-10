#! /bin/bash

DAY=$1

touch "./inputs/day$DAY.txt"
cp ./src/solutions/dayN.rs ./src/solutions/day$DAY.rs