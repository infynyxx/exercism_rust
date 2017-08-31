#!/bin/sh

cur_dir=$(pwd); 
for i in $(ls -d */)
do
    if [ $i == "etl/" ]; then
        continue
    fi

    cd ${i%%/}
    cargo test
    exit_code=$?
    cd $cur_dir
    if [ $exit_code -ne 0 ]; then
        echo "test failure for $i"
        exit $exit_code
    fi    
done