#!/bin/bash

die() {
	echo $1
	exit 1
}

[[ ! -d bots ]] && die 'missing bots folder'
[[ ! -d botpairs ]] && mkdir botpairs

# eg. botpair = "linbot-23d4c"

# $1 = bot
get_revs() {
	cd bots/$1;
	git rev-list master
}

create_botpair() {
	botpair="$1"
	bot=$(bot_from_botpair $botpair)
	local botpair_path="botpairs/$botpair"
	mkdir $botpair_path
	cp -r botwrapper $botpair_path    # created botpairs/linbot-23d4c/botwrapper
	cp -r bots/$bot $botpair_path/bot # created botpairs/linbot-23d4c/bot
	cd $botpair_path
	(cd bot;
		mv git .git;
		git checkout $(rev_from_botpair $1);
		rm -rf .git;
	)
	(cd botwrapper;
		cargo build;
		mv target/debug/botwrapper ../bot;
	)
	ls | grep -v bot | xargs rm -rf
}

create_missing_botpairs() {
	for bot in $(get_bots)
	do
		for rev in $(get_revs $bot)
		do
			local botpair="$bot-$rev"
			local botpair_path="botpairs/$botpair"
			[[ -f $botpair_path ]] && die 'Here is a file? -- snh'
			if [ ! -d $botpair_path ]; then
				create_botpair $botpair
			fi
		done
	done
}

bot_from_botpair() {
	echo ${1%-*}
}

rev_from_botpair() {
	echo ${1#*-}
}

# $1 = botpair
exec_botpair() {
	echo "executing botpair '$1'"
}

get_bots() {
	cd bots
	ls
}

get_botpairs() {
	cd botpairs
	ls
}

get_botpair_with_highest_prio() {
	local botpairs=$(get_botpairs)
	local max_botpair=${botpairs[0]}
	for botpair in $botpairs
	do
		if [ $(get_prio $botpair) -gt $(get_prio $max_botpair) ]; then
			max_botpair=$botpair
		fi
	done
	echo $max_botpair
}

# $1 = botpair
get_prio() { # 1/(1+number_of_execs) * 2^niceness * 1/(1+number_of_commits_behind)
	echo '1' # TODO
	# $(()) is only int!
}

exec_correct_botpair() {
	local botpair=$(get_botpair_with_highest_prio)
	local prio=$(get_prio $botpair)
	if [ ! $prio == 0 ]; then
		exec_botpair $botpair
	fi
}

create_missing_botpairs
exec_correct_botpair
git pull origin master

# TODO if time is right => commit

exec ./botroom.sh
