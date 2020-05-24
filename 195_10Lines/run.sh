#!/bin/bash

[[ `cat file.txt | wc -l` -gt 9 ]] && cat file.txt | head -n 10 | tail -n 1
