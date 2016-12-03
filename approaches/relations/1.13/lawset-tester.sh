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
	_test="$2"

	if [ "$(awk '/^''$/{print "1"}' "lawsets/$lawset/successful-tests.txt")" == 1 ]; then
		echo "y"
	elif [ "$(awk '/^''$/{print "1"}' "lawsets/$lawset/failed-tests.txt")" == 1 ]; then
		echo "n"
	fi
	echo "?"
}

set_status() { # lawset test status
	lawset="$1"
	_test="$2"
	status="$3"

	if [ "$status" == "y" ]; then
		sed '/^'"$_test"'$/d' "lawsets/$lawset/failed-tests.txt"
		echo "$_test" >> "lawsets/$lawset/successful-tests.txt"
	elif [ "$status" == "n" ]; then
		sed '/^'"$_test"'$/d' "lawsets/$lawset/successful-tests.txt"
		echo "$_test" >> "lawsets/$lawset/failed-tests.txt"
	elif [ "$status" == "?" ]; then
		sed '/^'"$_test"'$/d' "lawsets/$lawset/successful-tests.txt"
		sed '/^'"$_test"'$/d' "lawsets/$lawset/failed-tests.txt"
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
	_test="$3"

	if [ "$_test" == "all" ]; then
		for __test in $(ls tests); do
			call_test_with_lawset_definition "$_lawset" "$lawset_definition" "$__test"
		done
	elif [ "$_test" == "new" ]; then
		for __test in $(ls tests); do
			if [ "$(get_status "$lawset" "$_test")" == "?" ]; then
				call_test_with_lawset_definition "$_lawset" "$lawset_definition" "$__test"
			fi
		done
	else
		if [ ! -f "tests/$_test/code.txt" ]; then
			die "could not find 'tests/$_test/code.txt'"
		fi
		test_code="$(cat tests/$_test/code.txt)"
		reset
		while true; do
			echo -e "Lawset: $lawset\n"
			echo "$lawset_definition"
			echo "_______________________________________________________________________"
			echo -e "Test: $_test\n"
			echo "$test_code"
			echo "_______________________________________________________________________"
			printf ">> "
			read answer
			if [ "$answer" == "y" ]; then
				set_status "$lawset" "$_test" "y"
			elif [ "$answer" == "n" ]; then
				set_status "$lawset" "$_test" "n"
			elif [ "$answer" == "?" ]; then
				set_status "$lawset" "$_test" "?"
			else
				reset
				echo -e 'What?\n'
				continue
			fi
			break
		done
	fi

}

call_test() { # lawset test
	lawset="$1"
	_test="$2"
	if [ "$lawset" == "all" ]; then
		for _lawset in $(ls lawsets); do
			call_test "$_lawset" "$_test"
		done
	else
		if [ ! -f "lawsets/$lawset/definition.txt" ]; then
			die "could not find 'lawsets/$lawset/definition.txt'"
		fi
		lawset_definition="$(cat lawsets/$lawset/definition.txt)"
		call_test_with_lawset_definition "$lawset" "$lawset_definition" "$_test"
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
