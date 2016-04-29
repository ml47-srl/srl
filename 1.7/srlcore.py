#!/usr/bin/python3 -B

import sys
import os

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

def substituteCellstr(string, occurence, a, b):
	on("substituteCellstr('" + string + "', " + str(occurence) + ", '" + a + "', '" + b + "')")
	if not isinstance(string, str):
		die("substituteCellstr(): string is not str")
	if occurence == -1:
		spots = getSpotsForCellstrInCellstr(a, string)
		for spot in sorted(spots, reverse=True):
			string = string[:spot] + b + string[spot+len(a):]
	else:
		spots = getSpotsForCellstrInCellstr(a, string)
		if len(spots)-1 < occurence:
			die("substituteCellstr(): Can't do it! its too far")
		spot = spots[occurence]
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
		self.__subrules=list() # rules like a -> b. (substitution rules)
		self.__relrules=list() # rules like a(b). (relation rules)
		self.__importedfiles=list()

	def applySubstitution(self, srcruleID, subruleID, string):
		on("applySubstitution")
		srcrulestr = self.getSrcRuleByID(srcruleID).toString()

		subrule = self.__subrules[subruleID]

		vars = dict()
		if string != "":
			varstrs = string.split(" ")
			for varstr in varstrs:
				tmp = varstr.split("=")
				if len(tmp) != 2:
					die("SRLSystem::applySubstitution(): wrong tmp size (" + str(len(tmp)) + ") @ " + str(tmp))
				vars[tmp[0]] = tmp[1]

		newrulestr, msg = subrule.substitute(srcrulestr, vars)

		if newrulestr != None:
			self.__addRulestr(newrulestr)

		off("applySubstitution")
		return msg

	def validateCondition(self, conditionstr):
		if conditionstr == "true":
			return True

		for srcrule in self.getSrcRules():
			if srcrule.toString().strip(".") == conditionstr:
				return True
		return False

	def loadFromFile(self, filename):
		self.__addRulestrsByFile(filename, sys.path[0])

	def getSrcRules(self):
		return [x for x in self.__relrules + self.__subrules]

	def getSrcRuleByID(self, id):
		if id >= 0:
			return self.__relrules[id]
		else:
			return self.__subrules[-id-1]

	def getSubRules(self):
		return [x for x in self.__subrules]

	def __addRulestr(self, rulestr):
		if rulestr.startswith("<"):
			self.__subrules.append(SubRule(rulestr))
		else:
			self.__relrules.append(RelRule(rulestr))

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

		# check for imports
		while "@import(" in alllines:
			start = alllines.find("@import(\"")
			for i in range(start+9, len(alllines)):
				if alllines[i] == "\"" and alllines[i-1] != "\\":
					end=i
					break
			newfile = alllines[start+9:end]
			self.__addRulestrsByFile(newfile, os.path.dirname(abspath))
			alllines = alllines[:start] + alllines[end+3:]
		# check for rules
		while "." in alllines:
			self.__addRulestr(alllines[:alllines.find(".")+1])
			alllines = alllines[alllines.find(".")+1:]

	def toString(self):
		return str([x.toString() for x in self.__subrules]) + " : " + str([x.toString() for x in self.__relrules])

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

	def toString(self):
		if len(self.__args) == 0:
			return normalizeCellstr(self.__body)
		else:
			tmp = self.__body + "("
			for arg in self.__args:
				tmp += arg.toString() + ","
			tmp = tmp.strip(",") + ")"
			return normalizeCellstr(tmp)

	def getArgsStrs(self):
		return [x.toString() for x in self.__args]

class SubRule:
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

	def __insertArgs(self, string, vars):
		for var in vars:
			if var.startswith("{") and var.endswith("}"):
				string = substituteCellstr(string, -1, var, vars[var])
		return string

	def substitute(self, srcrulestr, vars):
		on("substitute")

		sys.path.append(sys.path[0] + "/subcells")
		import subcells

		srcrulestr = self.__insertArgs(srcrulestr, vars)

		argstrs=self.__cell.getArgsStrs()
		newrulestr, msg = eval("subcells." + self.getSubcellStr() + "(srcrulestr, argstrs, vars)") # TODO make less horrible

		off("substitute")
		return newrulestr, msg

	def getSubcellStr(self):
		string = self.toString()
		string = string[1:string.find("(")-1]
		debug("getSubcellStr: " + string)
		return string

	def toString(self):
		return self.__cell.toString() + "."

class RelRule:
	def __init__(self, arg):
		self.__cell = None
		self.__set(arg)

	def __set(self, arg):
		if isinstance(arg, Cell):
			self.__set(arg.toString())
			return
		if not isinstance(arg, str):
			die("RelRule::set() arg is not a string")
		self.__cell = Cell(arg.strip("."))

	def toString(self):
		return self.__cell.toString() + "."
