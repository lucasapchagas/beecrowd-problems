#! /bin/bash

# Var inialization through command line
_category=$1
_number=$2

if [ $# == 2 ]
then	
	if [ $_category -gt 0 -a $_category -lt 10 ]
	then
		folder=$(ls -d */ | grep $_category)
		
		# Save new file with second given argument name
		echo -e "use std::io;\n\nfn main() {\n\n}" > "$folder$_number.rs"

		# Use vscode to open the new problem file
		code "$folder$_number.rs"
		exit 0
	fi
fi

echo "Please give valid arguments"
echo "[1] Folder Category by a number from 1-9"
echo "[2] Problem number"
