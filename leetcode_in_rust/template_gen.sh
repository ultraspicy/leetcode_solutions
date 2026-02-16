#!/bin/bash

# Check if a problem number is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <problem_number>"
    exit 1
fi

problem_number=$1
file_path="src/problems/problem_$problem_number.rs"
mod_path="src/problems/mod.rs"

# Create the new problem file
cat << EOF > "$file_path"
use super::Solution;

impl Solution {
    // TODO: Implement the method for problem $problem_number
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }
}
EOF

echo "Created file: $file_path"

# Append to mod.rs
echo "pub mod problem_$problem_number;" >> "$mod_path"

echo "Updated $mod_path"
