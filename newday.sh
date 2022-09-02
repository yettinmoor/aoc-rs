#!/usr/bin/env sh

! [ "$#" -eq 2 ] && {
    echo './newday <year> <day>'
    exit 1
}

year=$1
day=$2

while true; do
    echo "add year$year::day$day? [y/N] "
    read -r yn
    case "$yn" in
        [yY]) break ;;
        *)    exit ;;
    esac
done

year="year$year"
day=$(printf 'day%02d' "$day")
src_file="src/$year/$day.rs"

[ -f "$src_file" ] && {
    echo "src file $($src_file) exists!"
    exit 1
}

cp src/common/template.rs "src/$year/$day.rs"
echo "pub mod $day;" >> "src/$year.rs"

$EDITOR "${AOC_INPUT_DIR:-input}/$year/$day.txt"
