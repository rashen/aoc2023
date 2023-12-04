#!/usr/bin/env bash

day=$1

# Fetch input
if [ -e .session ]
then
    session=$(<.session)
    curl "https://adventofcode.com/2023/day/$day/input" --compressed -H "Cookie: session=$session" > input/day$day
else
    echo "Could not find .session file"
fi

# Generate template
if [ ! -e src/day$day.rs ]
then
    cp template.rs src/day$day.rs
fi

