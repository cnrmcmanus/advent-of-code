#!/usr/bin/env bash
input=$(cat | tr -d '\n')
mul_and_sum() { rg -o --replace '$1 $2' 'mul\((\d{1,3}),(\d{1,3})\)' | awk '{sum += $1 * $2} END {print sum}'; }
echo $input | mul_and_sum
echo $input | rg -Po '(^|do\(\))(?:(?!don'\''t\(\)).)*mul\(\d{1,3},\d{1,3}\)' | mul_and_sum
