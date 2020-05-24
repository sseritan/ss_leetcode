#!/bin/bash

cols=$( head -n 1 file.txt | tr ' ' '\n' | wc -l )
for i in $(seq 1 $cols); do
  line=$( cat file.txt | awk -v i=$i '{printf $i " " }' | sed 's/ *$//g' )
  echo "$line"
done
