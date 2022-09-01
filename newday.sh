#!/usr/bin/env sh

year=$1
day=$2

while true; do
    read -r "add year$year::day$day? [y/N]" yn
    case "$yn" in
        [yY]) break ;;
        *)    exit ;;
    esac
done

year="year$year"
day=$(printf 'day%02d' "$day")

touch "input/$year/$day.txt"

cp src/common/template.rs "src/$year/$day.rs"

echo "pub mod $day" >> "src/$year.rs"
