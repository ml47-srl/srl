#!/usr/bin/python3 -B

import sys
import os
import srlfuncs

DEBUG=True
trace_indent=1

bodyAsCell=False

def toDebugFile(s):
	f = open("debug", "a")
	f.write(s + "\n")
	f.close()

def debug(s):
	print(s)
	toDebugFile(s)

def on(string):
	global trace_indent
	global DEBUG
	if DEBUG:
		debug("TRACE:" + ("\t" * trace_indent) + string)
		trace_indent += 1

def off(string):
	global trace_indent
	if DEBUG:
		debug("TRACE:" + ("\t" * (trace_indent-1)) + "/" + string)
		trace_indent -= 1

def getCellEndSigns():
	global bodyAsCell
	if bodyAsCell:
		return [",", ".", ")", "("]
	else:
		return [",", ".", ")"]

def getCellBeginSigns():
	return [",", "("]

def die(arg):
	debug("ERROR: " + arg)
	sys.exit()

def normalizeCellstr(string):
	return string.replace("\t", "").replace(" ", "").replace("\n", "")

def spotToCellstr(spot, string):
	on("spotToCellstr(" + str(spot) + ", '" + string + "')")
	cellname = ""
	openparens = 0
	while True:
		if string[spot] == "(":
			openparens += 1
		elif string[spot] == ")":
			openparens -= 1
		cellname += string[spot]
		spot += 1
		if len(string) < spot+1:
			off("spotToCellstr")
			return cellname
		elif openparens == 0 and (string[spot] in getCellEndSigns()):
			off("spotToCellstr")
			return cellname

def substituteCellstr(a, b, string, cspot):
	on("substituteCellstr('" + string + "', " + str(cspot) + ", '" + a + "', '" + b + "')")
	if not isinstance(string, str):
		die("substituteCellstr(): string is not str")
	if cspot == -1:
		spots = getSpotsForCellstrInCellstr(a, string)
		for spot in sorted(spots, reverse=True):
			string = string[:spot] + b + string[spot+len(a):]
	else:
		spot = cspotToSpot(cspot, string)
		string = string[:spot] + b + string[spot+len(a):]
	off("substituteCellstr")
	return string

def cspotToSpot(cspot, string):
	on("cspotToSpot(" + str(cspot) + ", '" + string + "')")
	if not isinstance(string, str):
		die("cspotToSpot(): string is not str")
	cells=0
	for i in range(len(string)):
		if cells == cspot:
			off("cspotToSpot")
			return i
		if string[i] in getCellBeginSigns():
			cells += 1
	off("cspotToSpot (shit)")

def spotToCspot(spot, string):
	on("spotToCspot(" + str(spot) + ", '" + string + "')")
	if not isinstance(string, str):
		die("spotToSpot(): string is not str")
	cells=0
	for i in range(len(string)):
		if i == spot:
			off("spotToCspot")
			return cells
		if string[i] in getCellBeginSigns():
			cells += 1
	off("spotToCspot (shit)")

def getParentCellCspot(cspot, cellstr):
	spot = cspotToSpot(cspot, cellstr)
	paren=0
	for i in range(spot, 0, -1):
		if cellstr[i] == "(":
			if paren == 0:
				return spotToCspot(i, cellstr)
			paren -= 1
		elif cellstr[i] == ")":
			paren += 1
	return -1

def cspotToCellstr(cspot, string):
	on("cspotToCellstr(" + str(cspot) + ", '" + string + "')")
	if not isinstance(string, str):
		die("cspotToCellstr(): string is not str")
	tmp = spotToCellstr(cspotToSpot(cspot, string), string)
	off("cspotToCellstr")
	return tmp

def getSpotsForCellstrInCellstr(subject, cellstr):
	on("getSpotsForCellstrInCellstr('" + subject + "', '" + cellstr + "')")
	tmp=list()
	shift=0
	while True:
		spot = cellstr.find(subject)
		if spot == -1:
			break
		thisshift = spot+len(subject)
		if (spot == 0 or (cellstr[spot-1] in getCellBeginSigns())):
			if (len(cellstr) == thisshift or cellstr[thisshift] in getCellEndSigns()):
				tmp.append(spot+shift)
		shift += thisshift
		cellstr = cellstr[thisshift:]
	off("getSpotsForCellstrInCellstr")
	return tmp

def getCspotsForCellstrInCellstr(subject, cellstr):
	on("getCspotsForCellstrInCellstr('" + subject + "', '" + cellstr + "')")
	if not isinstance(subject, str):
		die("getCspotsForCellstrInCellstr(): subject is not str")
	if not isinstance(cellstr, str):
		die("getCspotsForCellstrInCellstr(): cellstr is not str")
	tmp = getSpotsForCellstrInCellstr(subject, cellstr)
	for i in range(len(tmp)):
		tmp[i] = spotToCspot(tmp[i], cellstr)
	off("getCspotsForCellstrInCellstr")
	return tmp

class SRLSystem:
	def __init__(self):
		self.__rules=list()
		self.__importedfiles=list()

		self.__nosubcellstrs=list()
		self.__novarcellstrs=list()
		self.__nofunccellstrs=list()

	def applySubstitution(self, ruleID, cspot, action=None):
		on("applySubstitution")

		rulestr = self.__rules[ruleID].toString()
		cellstr = cspotToCellstr(cspot, rulestr)
		msg=""

		if cellstr.startswith("{"): # insert the variable
			if not self.novar(ruleID, cspot):
				if isinstance(action, str):
					self.__addRulestr(substituteCellstr(cellstr, action, rulestr, -1))
				else:
					debug("applySubstitution: {var}: action not str")
					msg = "SubInfo: ERROR: action not str"
			else:
				debug("applySubstitution: @novar prohibits")
				msg = "SubInfo: ERROR: @novar prohibits"

		elif cellstr.startswith("<") and action == "x": # call the function
			if not self.nofunc(ruleID, cspot):
				funcstr = cellstr[1:cellstr.find("(")-1]
				args = Cell(cellstr).getArgs()
				ret, msg = eval("srlfuncs." + funcstr + "(args)") # TODO make less horrible
				self.__addRulestr(substituteCellstr(cellstr, ret, rulestr, cspot))
			else:
				debug("applySubstitution: @nofunc prohibits")
				msg = "SubInfo: ERROR: @nofunc prohibits"

		elif not self.nosub(ruleID, cspot):
			if cellstr == "true": # substitute with rule
				self.__addRulestr(substituteCellstr(cellstr, self.__rules[int(action)].toString(), rulestr, cspot))
			else: # is it true?
				for rule in self.__rules:
					if rule.toString().strip(".") == cellstr: # yes!
						self.__addRulestr(substituteCellstr(cellstr, "true", rulestr, cspot))
						break
		else:
			debug("applySubstitution: @nosub prohibits")
			msg = "SubInfo: ERROR: @nosub prohibits"

		off("applySubstitution")
		return msg

	def novar(self, ruleID, cspot):
		rulestr = self.__rules[ruleID].toString()
		parentCspot = getParentCellCspot(cspot, rulestr)
		if parentCspot == -1:
			return False
		else:
			parentStr = cspotToCellstr(parentCspot, rulestr)
			parentBodyStr = parentStr[:parentStr.find("(")]
			return (parentBodyStr in self.__novarcellstrs) or self.novar(ruleID, parentCspot)
		
	def nofunc(self, ruleID, cspot):
		rulestr = self.__rules[ruleID].toString()
		parentCspot = getParentCellCspot(cspot, rulestr)
		if parentCspot == -1:
			return False
		else:
			parentStr = cspotToCellstr(parentCspot, rulestr)
			parentBodyStr = parentStr[:parentStr.find("(")]
			return (parentBodyStr in self.__nofunccellstrs) or self.nofunc(ruleID, parentCspot)

	def nosub(self, ruleID, cspot):
		rulestr = self.__rules[ruleID].toString()
		parentCspot = getParentCellCspot(cspot, rulestr)
		if parentCspot == -1:
			return False
		else:
			parentStr = cspotToCellstr(parentCspot, rulestr)
			parentBodyStr = parentStr[:parentStr.find("(")]
			return (parentBodyStr in self.__nosubcellstrs) or self.nosub(ruleID, parentCspot)

	def loadFromFile(self, filename):
		self.__addRulestrsByFile(filename, sys.path[0])

	def __addRulestr(self, rulestr):
		self.__rules.append(Rule(rulestr))

	def __addRulestrsByFile(self, filename, pwd):
		abspath = os.path.realpath(pwd + "/" + filename)
		if abspath in self.__importedfiles:
			debug("ignoring multiple import: \"" + abspath + "\"")
			return

		# load file
		try:
			fh = open(abspath)
		except:
			die("Couldn't load \"" + abspath + "\"")
		self.__importedfiles.append(abspath)
		lines = fh.readlines()
		fh.close()

		# concat all lines
		alllines=""
		for line in lines:
			if line.find("#") != -1:
				line = line[0:line.find("#")]
			alllines += normalizeCellstr(line)

		# check for @import
		while "@import(" in alllines:
			start = alllines.find("@import(\"")
			for i in range(start+9, len(alllines)):
				if alllines[i] == "\"" and alllines[i-1] != "\\":
					end=i
					break
			newfile = alllines[start+9:end]
			self.__addRulestrsByFile(newfile, os.path.dirname(abspath))
			alllines = alllines[:start] + alllines[end+3:]
		# check for @nosub
		while "@nosub(" in alllines:
			start = alllines.find("@nosub(")
			end = alllines[start:].find(")") + start

			self.__nosubcellstrs.append(alllines[start+7:end])
			alllines = alllines[:start] + alllines[end+2:]
		# check for @novar
		while "@novar(" in alllines:
			start = alllines.find("@novar(")
			end = alllines[start:].find(")") + start

			self.__novarcellstrs.append(alllines[start+7:end])
			alllines = alllines[:start] + alllines[end+2:]
		# check for @nofunc
		while "@nofunc(" in alllines:
			start = alllines.find("@nofunc(")
			end = alllines[start:].find(")") + start

			self.__nofunccellstrs.append(alllines[start+8:end])
			alllines = alllines[:start] + alllines[end+2:]
		# check for rules
		while "." in alllines:
			self.__addRulestr(alllines[:alllines.find(".")+1])
			alllines = alllines[alllines.find(".")+1:]

	def getRules(self):
		return self.__rules

	def toString(self):
		return str([x.toString() for x in self.__rules]) + " @nosub=" + str(self.__nosubcellstrs) + " @novar=" + str(self.__novarcellstrs) + " @nofunc=" + str(self.__nofunccellstrs) 

class Cell:
	def __init__(self, arg):
		self.__args=list()
		self.__body=""
		self.__set(arg)

	def __set(self, arg):
		if isinstance(arg, Cell):
			self.__set(arg.toString())
			return
		if not isinstance(arg, str):
			die("Cell::set() arg is not a string")
		string = normalizeCellstr(arg.strip("."))

		if not "(" in string:
			self.__body = string
			return
		self.__body = string[:string.find("(")]
		while string != self.__body + "()":
			argstr = spotToCellstr(string.find("(")+1, string)
			string = (string[:string.find("(")+1] + string[string.find("(")+1+len(argstr):]).replace("(,", "(")
			self.__args.append(Cell(argstr))

	def getArgs(self):
		return self.__args

	def toString(self):
		if len(self.__args) == 0:
			return normalizeCellstr(self.__body)
		else:
			tmp = self.__body + "("
			for arg in self.__args:
				tmp += arg.toString() + ","
			tmp = tmp.strip(",") + ")"
			return normalizeCellstr(tmp)

class Rule:
	def __init__(self, arg):
		self.__cell = None
		self.__set(arg)

	def __set(self, arg):
		if isinstance(arg, Cell):
			self.__set(arg.toString())
			return
		if not isinstance(arg, str):
			die("SubRule::set() arg is not a string")
		self.__cell = Cell(arg.strip("."))

	def toString(self):
		return self.__cell.toString() + "."
