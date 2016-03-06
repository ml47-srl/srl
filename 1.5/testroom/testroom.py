#!/usr/bin/env python

from curses import *
import sys
import re

# init
filename=sys.argv[1]
fh = open(filename)
rules=list(fh.readlines())
fh.close()
for x in range(len(rules)):
	rules[x] = rules[x].strip("\n").replace(" ", "")
screen = initscr()
screen.keypad(True)
noecho()
msg=""
maxY, maxX = screen.getmaxyx()
running = True

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

def findPreviousCellIndex(rule, pos):
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

def isTrueEqualsCell(cell):
	if not cell.startswith("equals("):
		return False
	argument1=getCellAt(7, cell)
	argument2=getCellAt(7+len(argument1)+1, cell)
	return argument1 == argument2 # if cell is equals(cell, cell)

def isWrongConstantEqualsCell(cell):
	"""
	return re.match(cell, r"equals\(\"{[a-zA-Z]\?}*\"\, *\"{[a-zA-Z]\?}*\"\)") # if cell is 'equals(<constant>, <other-constant>)'
	"""
	return False

def getCellAt(index, cell):
	cellname = ""
	openparens = 0
	while True:
		if cell[index] == "(":
			openparens += 1
		elif cell[index] == ")":
			openparens -= 1
		cellname += cell[index]
		index += 1
		if len(cell) < index+1:
			return cellname
		elif openparens == 0 and (cell[index] == "," or cell[index] == ")" or cell[index] == "."):
			return cellname

def substituteCellAt(index, cell, sub):
	rmcell = getCellAt(index, cell)
	cell = cell[0:index] + cell[len(rmcell)+index:]
	cell = cell[0:index] + sub + cell[index:]
	return cell

def findSubstitutions(cell):
	substitutions=list()
	if isTrueEqualsCell(cell):
		substitutions.append("\"true\"")
	elif isWrongConstantEqualsCell(cell):
		substitutions.append("\"false\"")
	substitutions.append("same-as-everything") # TODO remove
	return substitutions

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
	global running
	targetSubstitutionIndex=0
	targetCellIndex=0
	while running:
		substitutions=findSubstitutions(getCellAt(targetCellIndex, rule))
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
		elif key == ord('q'):
			running=False
		elif key == ord('\n'):
			rules.append(substituteCellAt(targetCellIndex, rule, substitutions[targetSubstitutionIndex]))
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
	global msg
	global running
	targetRuleIndex=0
	while running:
		repaintMain(targetRuleIndex)
		key = screen.getch()

		if str(key) == str(KEY_DOWN):
			if targetRuleIndex < len(rules)-1:
				targetRuleIndex += 1
		elif str(key) == str(KEY_UP):
			if targetRuleIndex > 0:
				targetRuleIndex -= 1
		elif key == ord('q'):
			running=False
		elif key == ord('\n'):
			handleRule(rules[targetRuleIndex])
		else:
			msg="wrong key"
if __name__ == "__main__":
	main()

# uninit
endwin()
