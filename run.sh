#! /bin/bash

# Var inialization through command line
_category=$1
_number=$2

if [ $# == 2 ]
then	
	if [ $_category -gt 0 -a $_category -lt 10 ]
	then
		folder=$(ls -d */ | grep $_category)
		compiled=$(rustc --out-dir "target/" "$folder$_number.rs")
		./target/"$_number"
        exit 0
	fi
fi

echo "Please give valid arguments"
echo "[1] Folder Category by a number from 1-9"
echo "[2] Problem number"
