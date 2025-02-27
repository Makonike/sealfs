#!/bin/bash

function finish() {
    trap 'kill $(jobs -p)' EXIT
    exit $1
}

function green_font() {
    echo -e "\033[32m$1\033[0m\c"
}

function fuse_test() {
    ./target/debug/client --log-level warn daemon&
    sleep 3
    ./target/debug/client --log-level warn mount ~/fs test1

    start_time=$[$(date +%s%N)/1000000]
    cd io500
    timeout -s SIGKILL 200 mpirun -np 5 ./io500 config-minimal.ini
    result=$?
    cd ..
    end_time=$[$(date +%s%N)/1000000]
    result_time=$[ $end_time - $start_time ]
    echo -e "fuse tests finish, cost: $(green_font ${result_time}ms)"
    return $result
}

function intercept_test() {
    start_time=$[$(date +%s%N)/1000000]
    cd io500
    SEALFS_LOG_LEVEL=warn SEALFS_VOLUME_NAME=test1 SEALFS_MOUNT_POINT=~/fs LD_PRELOAD=../target/debug/libintercept.so timeout -s SIGKILL 200 mpirun -np 5 ./io500 config-minimal.ini
    result=$?
    cd ..
    end_time=$[$(date +%s%N)/1000000]
    result_time=$[ $end_time - $start_time ]
    echo -e "intercept tests finish, cost: $(green_font ${result_time}ms)"
    return $result
}

echo "start fuse_client_run"

# exit with 1 if no argument
if [ $# -eq 0 ]
then
    echo "no argument"
    exit 1
fi

set +e

sudo umount ~/fs
rm /tmp/sealfs.sock
rm /tmp/sealfs.index
mkdir -p ~/fs

sudo rm -rf io500
sudo rm -rf $1/database*
sudo rm -rf $1/storage*

set -e

SEALFS_CONFIG_PATH=./examples ./target/debug/manager --log-level warn &

sleep 1

for ((i=0; i<5; i++))
do
    port=$[8085+$i]
    ./target/debug/server --server-address 127.0.0.1:${port} --database-path $1/database${i}/ --storage-path $1/storage${i}/ --log-level warn &
done

sleep 3


SELF_HOSTED=1

if [ $SELF_HOSTED -eq 1 ]
then
    cp -r ~/io500/io500 .
    cd io500
else
    git clone https://github.com/IO500/io500.git
    cd io500
    ./prepare.sh
fi

echo "[global]" > config-minimal.ini
echo "datadir = /home/sealos/fs" >> config-minimal.ini
echo "" >> config-minimal.ini
echo "[debug]" >> config-minimal.ini
echo "stonewall-time = 2" >> config-minimal.ini

cd ..

set +e

./target/debug/client --log-level warn create-volume test1 100000

fuse_test
fuse_result=$?
echo "fuse result: $fuse_result"

intercept_test
intercept_result=$?
echo "intercept result: $intercept_result"
result=$(($fuse_result||$intercept_result))

set -e
finish $result