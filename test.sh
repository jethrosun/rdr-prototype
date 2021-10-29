#!/bin/bash
set -ex

for (( c=1; c<=10; c++ ))
do
	echo "$c"
	cargo r --release $c  | tee $c.log
done
