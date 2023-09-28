#/bin/sh

set -xe

./target/debug/brainfuck_plus ./examples/rule110.bf

./examples/rule110 > input.txt

python3 test.py
