#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <program_name.rs>"
    exit 1
fi

PROGRAM_NAME=$1
rustc "${PROGRAM_NAME}" --test && ./"${PROGRAM_NAME}"

