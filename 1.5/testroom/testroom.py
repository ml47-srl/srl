#!/usr/bin/env python

from curses import *
import sys

# init
filename=sys.argv[1]
fh = open(filename)
rules=list(fh.readlines())
fh.close()
screen = initscr()
screen.keypad(True)
noecho()
SPACE=3
msg=""
maxY, maxX = screen.getmaxyx()

# main
def findNextCellIndex(rule, pos):
	tmp = pos
	# bis zum nxten bedeutungsvollen Zeichen
	while rule[tmp] != "," and rule[tmp] != "(" and rule[tmp] != ")":
		tmp += 1
		if len(rule)-2 < tmp:
			return pos
	# bis zur nxten Cell
	while rule[tmp] == "," or rule[tmp] == "(" or rule[tmp] == ")" or rule[tmp] == " " or rule[tmp] == ".":
		tmp += 1
		if len(rule)-2 < tmp:
			return pos
	return tmp

def findPreviousCellIndex(rule, pos): # example: next-cell(a, b).
	if pos == 0:
		return pos
	tmp = pos-1
	# bis zum Ende der vorherigen Cell
	while rule[tmp] == "," or rule[tmp] == "(" or rule[tmp] == ")" or rule[tmp] == " ":
		tmp -= 1
	# bis zum Anfang der vorherigen Cell
	while rule[tmp-1] != "," and rule[tmp-1] != "(" and rule[tmp-1] != ")" and rule[tmp-1] != " " and tmp > 0:
		tmp -= 1
	return tmp

def repaintHandleRule(rule, targetCellIndex, substitutions, targetSubstitutionIndex):
	screen.clear()
	screen.addstr(0, 2, rule)
	screen.addstr(1, targetCellIndex+2, "^")
	screen.addstr(targetSubstitutionIndex+2, 0, ">")
	for x in range(len(substitutions)):
		screen.addstr(x+2, 2, substitutions[x])
	screen.addstr(maxY-1, 0, msg)

def handleRule(rule):
	global rules
	substitutions=list()
	substitutions.append("first sub")
	targetSubstitutionIndex=0
	targetCellIndex=0
	while True:
		repaintHandleRule(rule, targetCellIndex, substitutions, targetSubstitutionIndex)
		key = screen.getch()
		
		if str(key) == str(KEY_DOWN):
			if targetSubstitutionIndex < len(substitutions)-1:
				targetSubstitutionIndex += 1
		elif str(key) == str(KEY_UP):
			if targetSubstitutionIndex > 0:
				targetSubstitutionIndex -= 1
		elif str(key) == str(KEY_LEFT):
				targetCellIndex = findPreviousCellIndex(rule, targetCellIndex)
		elif str(key) == str(KEY_RIGHT):
				targetCellIndex = findNextCellIndex(rule, targetCellIndex)
		elif key == ord('\n'):
			rules.append("ok, thats the new rule")
			break
		else:
			msg="wrong key"

def repaintMain(targetRuleIndex):
	global rules
	screen.clear()
	screen.addstr(targetRuleIndex, 0, ">")
	for x in range(len(rules)):
		screen.addstr(x, 2, rules[x])
	screen.addstr(maxY-1, 0, msg)


def main():
	global rules
	targetRuleIndex=0
	while True:
		repaintMain(targetRuleIndex)
		key = screen.getch()

		if str(key) == str(KEY_DOWN):
			if targetRuleIndex < len(rules)-1:
				targetRuleIndex += 1
		elif str(key) == str(KEY_UP):
			if targetRuleIndex > 0:
				targetRuleIndex -= 1
		elif key == ord('\n'):
			handleRule(rules[targetRuleIndex])
		else:
			msg="wrong key"
main()
screen.getch()

# uninit
endwin()
