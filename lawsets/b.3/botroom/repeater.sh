#!/bin/bash

git pull origin master
for ((i=0;i<100;i++))
do
	cargo run
done
exec ./repeater.sh
