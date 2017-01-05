#!/bin/bash

usage="Usage:\tcontrol add-fs-test [fs-test-name]
	control add-ls-test [ls-test-name]

	control add-fs [fs-name]
	control add-ls [ls-name] [featureset]

	control set-ls-status [lawset] [ls-test] [status]
	control set-fs-status [featureset] [fs-test] [status]

	control open-fs [fs-name]
	control open-ls [ls-name]
	control fs-table
	control ls-table [fs]"

print_usage() {
	echo -e "$usage"
}

die() {
	echo "$1"
	exit
}

get_ls_status() { # lawset test
	lawset="$1"
	tst="$2"

	if [ -z "$lawset" ]; then
		die 'get_ls_status; no lawset given'
	fi
	if [ -z "$tst" ]; then
		die 'get_ls_status; no test given'
	fi

	if [ "a$(awk '/^'"$tst"'$/{print "1"}' "lawsets/$lawset/successful-tests.txt")" == a1 ]; then
		echo "y"
	elif [ "a$(awk '/^'"$tst"'$/{print "1"}' "lawsets/$lawset/failed-tests.txt")" == a1 ]; then
		echo "n"
	else
		echo "?"
	fi
}

set_ls_status() { # lawset test status
	lawset="$1"
	tst="$2"
	status="$3"

	if [ -z "$lawset" ]; then
		die 'set_ls_status; no lawset given'
	fi
	if [ -z "$tst" ]; then
		die 'set_ls_status; no test given'
	fi
	if [ -z "$status" ]; then
		die 'set_ls_status; no status given'
	fi

	if [ "$status" == "y" ]; then
		sed '/^'"$tst"'$/d' "lawsets/$lawset/failed-tests.txt"
		echo "$tst" >> "lawsets/$lawset/successful-tests.txt"
	elif [ "$status" == "n" ]; then
		sed '/^'"$tst"'$/d' "lawsets/$lawset/successful-tests.txt"
		echo "$tst" >> "lawsets/$lawset/failed-tests.txt"
	elif [ "$status" == "?" ]; then
		sed '/^'"$tst"'$/d' "lawsets/$lawset/successful-tests.txt"
		sed '/^'"$tst"'$/d' "lawsets/$lawset/failed-tests.txt"
	else
		die "set_ls_status called with invalid status: $status"
	fi
}

get_fs_status() { # featureset test
	featureset="$1"
	tst="$2"

	if [ -z "$featureset" ]; then
		die 'get_fs_status; no featureset given'
	fi
	if [ -z "$tst" ]; then
		die 'get_fs_status; no test given'
	fi

	if [ "a$(awk '/^'"$tst"'$/{print "1"}' "featuresets/$featureset/successful-tests.txt")" == a1 ]; then
		echo "y"
	elif [ "a$(awk '/^'"$tst"'$/{print "1"}' "featuresets/$featureset/failed-tests.txt")" == a1 ]; then
		echo "n"
	else
		echo "?"
	fi
}

set_fs_status() { # featureset test status
	featureset="$1"
	tst="$2"
	status="$3"

	if [ -z "$featureset" ]; then
		die 'set_fs_status; no featureset given'
	fi
	if [ -z "$tst" ]; then
		die 'set_fs_status; no test given'
	fi
	if [ -z "$status" ]; then
		die 'set_fs_status; no status given'
	fi

	if [ "$status" == "y" ]; then
		sed '/^'"$tst"'$/d' "featuresets/$featureset/failed-tests.txt"
		echo "$tst" >> "featuresets/$featureset/successful-tests.txt"
	elif [ "$status" == "n" ]; then
		sed '/^'"$tst"'$/d' "featuresets/$featureset/successful-tests.txt"
		echo "$tst" >> "featuresets/$featureset/failed-tests.txt"
	elif [ "$status" == "?" ]; then
		sed '/^'"$tst"'$/d' "featuresets/$featureset/successful-tests.txt"
		sed '/^'"$tst"'$/d' "featuresets/$featureset/failed-tests.txt"
	else
		die "set_fs_status called with invalid status: $status"
	fi
}

call_add_fs_test() { # name
	if [ -z "$1" ]; then
		die "call_add_fs_test: no test-name given"
	fi

	if [ -d "featureset-tests/$1" ]; then
		die "fs-test already exists"
	fi

	mkdir "featureset-tests/$1"
	vi "featureset-tests/$1/code.txt"
	echo "adding fs-test '$1'"
}

call_add_ls_test() { # name featureset
	if [ -z "$1" ]; then
		die "call_add_ls_test: no test-name given"
	fi

	if [ -d "lawset-tests/$1" ]; then
		die "ls-test already exists"
	fi

	featuresets=""
	echo 'Enter featuresets'
	while true
	do
		read featureset

		if [ -z "$featureset" ]; then
			break
		fi

		if [ -d "featuresets/$featureset" ]; then
			featuresets="$featureset\n$featuresets"
		else
			echo "The featureset '$featureset' does not exist"
		fi
	done

	mkdir "lawset-tests/$1"
	echo -e "$featuresets" > "lawset-tests/$1/featureset.txt"
	vi "lawset-tests/$1/code.txt"
	echo "adding ls-test '$1'"
}

call_add_ls() { # name featureset
	if [ -z "$1" ]; then
		die "call_add_ls: no lawset-name given"
	fi

	if [ -d "lawsets/$1" ]; then
		die "lawset already exists"
	fi

	if [ ! -d "featuresets/$2" ]; then
		die "featureset does not exist"
	fi

	mkdir "lawsets/$1"
	echo "$2" > "lawsets/$1/featureset.txt"
	touch "lawsets/$1/successful-tests.txt"
	touch "lawsets/$1/failed-tests.txt"
	vi "lawsets/$1/definition.txt"
	echo "adding lawset '$1'"
}

call_add_fs() { # name
	if [ -z "$1" ]; then
		die "call_add_fs: no featureset-name given"
	fi

	if [ -d "featuresets/$1" ]; then
		die "featureset already exists"
	fi

	mkdir "featuresets/$1"
	touch "featuresets/$1/successful-tests.txt"
	touch "featuresets/$1/failed-tests.txt"
	vi "featuresets/$1/definition.txt"
	echo "adding featureset '$1'"
}

call_fs_table() {
	#   L L L L 
	# T
	# T
	# T

	featuresets=($(ls featuresets))
	tests=($(ls featureset-tests))
	featuresets_len=${#featuresets[*]}
	tests_len=${#tests[*]}

	# determine lengths
	lengths[0]=0
	for tst in ${tests[*]}; do
		if [ "${#tst}" -gt "${lengths[0]}" ]; then
			lengths[0]="${#tst}"
		fi
	done

	for ((i=0;i<$featuresets_len;i++)); do
		lengths[$(($i+1))]=${#featuresets[$i]}
	done
	lengths_len=${#lengths[*]}

	# calculate pause = "==========================+=====+===+====+=====..."
	pause=""
	for ((i=0;i<$lengths_len;i++)); do
		for ((j=0;j<${lengths[$i]};j++)); do
			pause="$pause="
		done
		if [ "$(( $i + 1 ))" -lt "$lengths_len" ]; then # if this is not the last iteration
			pause="$pause=+="
		fi
	done

	# first line
	for ((i=0;i<${lengths[0]};i++)); do
		resultstring=" $resultstring"
	done

	for featureset in ${featuresets[*]}; do
		resultstring="$resultstring | $featureset"
	done
	resultstring="$resultstring\n"
	
	# all other lines
	for tst in ${tests[*]}; do
		resultstring="$resultstring$pause\n"
		resultstring="$resultstring$tst"
		buffer=$(( ${lengths[0]} - ${#tst} ))
		for ((i=0;i<$buffer;i++)); do
			resultstring="$resultstring "
		done
		featureset_i=0
		for featureset in ${featuresets[*]}; do
			stats=$(get_fs_status $featureset $tst)
			if [ $stats == "?" ]; then
				stats=" "
			fi
			resultstring="$resultstring | $stats"
			buffer=$(( ${lengths[$(( $featureset_i + 1 ))]} - 1 )) # 1 == ${#stats}
			for ((i=0;i<$buffer;i++)); do
				resultstring="$resultstring "
			done
			featureset_i="$(($featureset_i + 1))"
		done
		resultstring="$resultstring\n"
	done

	# output
	echo -e "$resultstring" | less -S
}

call_ls_table() { # featureset
	#   L L L L 
	# T
	# T
	# T

	featureset="$1"

	if [ ! -d "featuresets/$1" ]; then
		die 'call_ls_table: featureset does not exist'
	fi

	for lawset in $(ls lawsets)
	do
		if [ "$(cat "lawsets/$lawset/featureset.txt")" == "$featureset" ]; then
			lawsets=(${lawsets[*]} $lawset)
		fi
	done
	lawsets_len=${#lawsets[*]}

	for tst in $(ls lawset-tests)
	do
		if [ $(cat "lawset-tests/$tst/featureset.txt" | grep "^$featureset$") ]; then
			tests=(${tests[*]} $tst)
		fi
	done
	tests_len=${#tests[*]}

	# determine lengths
	lengths[0]=0
	for tst in ${tests[*]}; do
		if [ "${#tst}" -gt "${lengths[0]}" ]; then
			lengths[0]="${#tst}"
		fi
	done

	for ((i=0;i<$lawsets_len;i++)); do
		lengths[$(($i+1))]=${#lawsets[$i]}
	done
	lengths_len=${#lengths[*]}

	# calculate pause = "==========================+=====+===+====+=====..."
	pause=""
	for ((i=0;i<$lengths_len;i++)); do
		for ((j=0;j<${lengths[$i]};j++)); do
			pause="$pause="
		done
		if [ "$(( $i + 1 ))" -lt "$lengths_len" ]; then # if this is not the last iteration
			pause="$pause=+="
		fi
	done

	# first line
	for ((i=0;i<${lengths[0]};i++)); do
		resultstring=" $resultstring"
	done

	for lawset in ${lawsets[*]}; do
		resultstring="$resultstring | $lawset"
	done
	resultstring="$resultstring\n"
	
	# all other lines
	for tst in ${tests[*]}; do
		resultstring="$resultstring$pause\n"
		resultstring="$resultstring$tst"
		buffer=$(( ${lengths[0]} - ${#tst} ))
		for ((i=0;i<$buffer;i++)); do
			resultstring="$resultstring "
		done
		lawset_i=0
		for lawset in ${lawsets[*]}; do
			stats=$(get_ls_status $lawset $tst)
			if [ $stats == "?" ]; then
				stats=" "
			fi
			resultstring="$resultstring | $stats"
			buffer=$(( ${lengths[$(( $lawset_i + 1 ))]} - 1 )) # 1 == ${#stats}
			for ((i=0;i<$buffer;i++)); do
				resultstring="$resultstring "
			done
			lawset_i="$(($lawset_i + 1))"
		done
		resultstring="$resultstring\n"
	done

	# output
	echo -e "$resultstring" | less -S
}

if [[ $# < 1 ]]; then
	echo "not enough arguments"
	print_usage
	exit
fi

if [ "$1" == "add-fs-test" ]; then
	if [ ! $# == 2 ]; then
		die "add-fs-test needs one argument"
	fi
	call_add_fs_test "$2"
elif [ "$1" == "add-ls-test" ]; then
	if [ ! $# == 2 ]; then
		die "add-ls-test needs one argument"
	fi
	call_add_ls_test "$2"
elif [ "$1" == "add-fs" ]; then
	if [ ! $# == 2 ]; then
		die "add-fs needs one argument"
	fi
	call_add_fs "$2"
elif [ "$1" == "add-ls" ]; then
	if [ ! $# == 3 ]; then
		die "add-ls needs two arguments"
	fi
	call_add_ls "$2" "$3"
elif [ "$1" == "set-ls-status" ]; then
	if [ ! $# == 4 ]; then
		die "set-ls-status needs three arguments"
	fi
	set_ls_status "$2" "$3" "$4"
elif [ "$1" == "set-fs-status" ]; then
	if [ ! $# == 4 ]; then
		die "set-fs-status needs three arguments"
	fi
	set_fs_status "$2" "$3" "$4"
elif [ "$1" == "open-fs" ]; then
	if [ ! $# == 2 ]; then
		die "open-fs needs one argument"
	fi
	vi "featuresets/$2"
elif [ "$1" == "open-ls" ]; then
	if [ ! $# == 2 ]; then
		die "open-ls needs one argument"
	fi
	vi "lawsets/$2"
elif [ "$1" == "fs-table" ]; then
	if [ ! $# == 1 ]; then
		die "fs-table needs no arguments"
	fi
	call_fs_table
elif [ "$1" == "ls-table" ]; then
	if [ ! $# == 2 ]; then
		die "ls-table needs one argument"
	fi
	call_ls_table "$2"
else
	echo "invalid argument"
	print_usage
fi
