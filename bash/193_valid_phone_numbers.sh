#!/bin/bash

echo -e "987-123-4567\n123 456 7890\n(123) 456-7890" >> file.txt

grep -e "^[0-9]\{3\}\-[0-9]\{3\}\-[0-9]\{4\}$" -e "^([0-9]\{3\}) [0-9]\{3\}\-[0-9]\{4\}$" file.txt

rm file.txt
