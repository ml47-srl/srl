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
			if targetCellIndex > 0:
				targetCellIndex -= 1
		elif str(key) == str(KEY_RIGHT):
			if targetCellIndex+2 < len(rule):
				targetCellIndex += 1
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
