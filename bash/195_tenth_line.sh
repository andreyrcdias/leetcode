#!/bin/bash

echo -e "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11" > file.txt

sed -n '10p' file.txt

rm file.txt
