#!/bin/bash

usage="Usage:\tlawset-tester add-test [test]
	lawset-tester add-lawset [lawset]
	lawset-tester test [lawset] [test]
	lawset-tester dump-failed [lawset]
	lawset-tester dump-tests
	lawset-tester dump-lawsets
	lawset-tester ls-tests
	lawset-tester ls-lawsets
	lawset-tester print-test [testname]
	lawset-tester print-lawset [lawset]"

# complete -W "$(ls tests) $(ls lawsets) add-test add-lawset test dump-failed dump-tests dump-lawsets ls-tests ls-lawsets print-test print-lawset" "$0"

print_usage() {
	echo -e "$usage"
}

die() {
	echo "$1"
	exit
}

get_status() { # lawset test
	lawset="$1"
	tst="$2"

	if [ -z "$lawset" ]; then
		die 'get_status; no lawset given'
	fi
	if [ -z "$tst" ]; then
		die 'get_status; no test given'
	fi

	if [ "a$(awk '/^'"$tst"'$/{print "1"}' "lawsets/$lawset/successful-tests.txt")" == a1 ]; then
		echo "y"
	elif [ "a$(awk '/^'"$tst"'$/{print "1"}' "lawsets/$lawset/failed-tests.txt")" == a1 ]; then
		echo "n"
	else
		echo "?"
	fi
}

set_status() { # lawset test status
	lawset="$1"
	tst="$2"
	status="$3"

	if [ -z "$lawset" ]; then
		die 'set_status; no lawset given'
	fi
	if [ -z "$tst" ]; then
		die 'set_status; no test given'
	fi
	if [ -z "$status" ]; then
		die 'set_status; no status given'
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
		die "set_status called with invalid status: $status"
	fi
}

call_add_test() {
	if [ -z "$1" ]; then
		die "call_add_test: no test-name given"
	fi

	if [ -d "tests/$1" ]; then
		die "test already exists"
	fi

	mkdir "tests/$1"
	vi "tests/$1/code.txt"
	echo "adding test '$1'"
}

call_add_lawset() {
	if [ -z "$1" ]; then
		die "call_add_lawset: no lawset-name given"
	fi

	if [ -d "lawsets/$1" ]; then
		die "lawset already exists"
	fi

	mkdir "lawsets/$1"
	touch "lawsets/$1/successful-tests.txt"
	touch "lawsets/$1/failed-tests.txt"
	vi "lawsets/$1/definition.txt"
	echo "adding lawset '$1'"
}

call_test_with_lawset_definition() { # lawset lawsetdefinition test
	lawset="$1"
	lawset_definition="$2"
	tst="$3"

	if [ "$tst" == "all" ]; then
		for for_tst in $(ls tests); do
			call_test_with_lawset_definition "$lawset" "$lawset_definition" "$for_tst"
		done
	elif [ "$tst" == "new" ]; then
		for for_tst in $(ls tests); do
			if [ "$(get_status "$lawset" "$for_tst")" == "?" ]; then
				call_test_with_lawset_definition "$lawset" "$lawset_definition" "$for_tst"
			fi
		done
	else
		if [ ! -f "tests/$tst/code.txt" ]; then
			die "could not find 'tests/$tst/code.txt'"
		fi
		test_code="$(cat tests/$tst/code.txt)"
		tput reset
		while true; do
			echo -e "Lawset: $lawset\n"
			echo "$lawset_definition"
			echo "_______________________________________________________________________"
			echo -e "Test: $tst\n"
			echo "$test_code"
			echo "_______________________________________________________________________"
			echo "[y]: Yes, test passed	[n]: No, test failed	[?]: Answer not clear	[s]: Skip test"
			printf ">> "
			read answer
			if [ "$answer" == "y" ]; then
				set_status "$lawset" "$tst" "y"
			elif [ "$answer" == "n" ]; then
				set_status "$lawset" "$tst" "n"
			elif [ "$answer" == "?" ]; then
				set_status "$lawset" "$tst" "?"
			elif [ ! "$answer" == "s" ]; then
				tput reset
				echo -e 'What?\n'
				continue
			fi
			break
		done
	fi

}

call_test() { # lawset test
	lawset="$1"
	tst="$2"
	if [ "$lawset" == "all" ]; then
		for for_lawset in $(ls lawsets); do
			call_test "$for_lawset" "$tst"
		done
	else
		if [ ! -f "lawsets/$lawset/definition.txt" ]; then
			die "could not find 'lawsets/$lawset/definition.txt'"
		fi
		lawset_definition="$(cat lawsets/$lawset/definition.txt)"
		call_test_with_lawset_definition "$lawset" "$lawset_definition" "$tst"
	fi
}

call_dump_failed() { # lawset
	lawset="$1"
	for for_tst in $(ls tests); do
		if [ "$(get_status "$lawset" "$for_tst")" == "n" ]; then
			echo "Test: $for_tst:"
			cat tests/$for_tst/code.txt
			echo
		fi
	done
}

call_dump_tests() {
	for for_tst in $(ls tests); do
		echo "Test: $for_tst:"
		cat tests/$for_tst/code.txt
		echo
	done
}

call_dump_lawsets() {
	for for_tst in $(ls lawsets); do
		echo "Lawset: $for_tst:"
		cat tests/$for_tst/definition.txt
		echo
	done
}

call_ls_tests() {
	ls tests
}

call_ls_lawsets() {
	ls lawsets
}

call_print_test() {
	cat "tests/$1/code.txt"
}

call_print_lawset() {
	cat "lawsets/$1/definition.txt"
}

if [[ $# < 1 ]]; then
	echo "not enough arguments"
	print_usage
	exit
fi

if [ "$1" == "add-test" ]; then
	if [ ! $# == 2 ]; then
		die "add-test needs one argument"
	fi
	call_add_test "$2"
elif [ "$1" == "add-lawset" ]; then
	if [ ! $# == 2 ]; then
		die "add-lawset needs one argument"
	fi
	call_add_lawset "$2"
elif [ "$1" == "test" ]; then	
	if [ ! $# == 3 ]; then
		die "test needs one argument"
	fi
	call_test "$2" "$3"
elif [ "$1" == "dump-failed" ]; then
	if [ ! $# == 2 ]; then
		die "dump-failed needs one argument"
	fi
	call_dump_failed "$2"
elif [ "$1" == "dump-tests" ]; then
	if [ ! $# == 1 ]; then
		die "dump-tests needs no arguments"
	fi
	call_dump_tests
elif [ "$1" == "dump-lawsets" ]; then
	if [ ! $# == 1 ]; then
		die "dump-lawsets needs no arguments"
	fi
	call_dump_lawsets
elif [ "$1" == "ls-tests" ]; then
	if [ ! $# == 1 ]; then
		die "ls-tests needs no arguments"
	fi
	call_ls_tests
elif [ "$1" == "ls-lawsets" ]; then
	if [ ! $# == 1 ]; then
		die "ls-lawsets needs no arguments"
	fi
	call_ls_lawsets
elif [ "$1" == "print-test" ]; then
	if [ ! $# == 2 ]; then
		die "print-test needs one argument"
	fi
	call_print_test "$2"
elif [ "$1" == "print-lawset" ]; then
	if [ ! $# == 2 ]; then
		die "print-lawset needs one argument"
	fi
	call_print_lawset "$2"
else
	echo "invalid argument"
	print_usage
fi
