#!/bin/bash

# List all words on line
# sort to group, uniq to count
# sort to reorder by frequency, then awk to match formatting
cat words.txt | while read line; do for word in $line; do echo $word; done; done | sort | uniq -c | sort -r | awk '{print $2 " " $1}'
