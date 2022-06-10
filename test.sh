#!/bin/bash
set -ex

for (( c=1; c<=5; c++ ))
do
	echo "$c"
	sudo tcpdump
	cargo r --release $c  | tee $c.log
done
