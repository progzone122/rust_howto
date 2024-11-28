#! /bin/bash
set -euo pipefail
IFS=$'\n\t'

# Quick and dirty script
# Add all necessary references for the index of examples (in `examples_index.md`) to `src/refs.incl.md`

cd /code

# Look into every refs.incl.md
for file in $(find ./src -type f -name "refs.incl.md" -not -path "./src/refs.incl.md")
do
    echo "Processing ${file}"
    # Get the relative path from ./src to the directory of the current file
    rel=$(realpath --relative-to=./src $file | sed 's/refs.incl.md//');
    # Insert the path in each reference definition
    refs=$(sed -nE 's~(^\[ex-.+?\]:)\s?([^#]+?)(#.+)?$~\1 '${rel}'\2\3~p' $file)
    # Add the references to ./src/refs.incl.md
    echo "${refs}" >> ./src/refs.incl.md
done

# Sort and dedupe
sort -u -o ./src/refs.incl.md ./src/refs.incl.md
# Delete empty lines
sed -i -E '/^\s*$/d' ./src/refs.incl.md
echo "DONE"
