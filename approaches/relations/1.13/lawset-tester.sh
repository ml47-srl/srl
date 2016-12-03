#!/bin/bash

usage="Usage:\tlawset-tester add-test [test-name]\n\tlawset-tester add-lawset [lawset-name]\n\tlawset-tester test [lawset] [test]\n\t"

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
	mkdir "tests/$1"
	vi "tests/$1/code.txt"
	echo "adding test '$1'"
}

call_add_lawset() {
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
			printf ">> "
			read answer
			if [ "$answer" == "y" ]; then
				set_status "$lawset" "$tst" "y"
			elif [ "$answer" == "n" ]; then
				set_status "$lawset" "$tst" "n"
			elif [ "$answer" == "?" ]; then
				set_status "$lawset" "$tst" "?"
			else
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
else
	echo "invalid argument"
	print_usage
fi
