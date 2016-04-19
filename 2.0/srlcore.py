#!/usr/bin/python3.4 -B

import sys
import os

DEBUG=False
debug_amount=0

bodyAsCell=False

def on(string):
	global debug_amount
	if DEBUG:
		print("DEB:" + "\t" * debug_amount + string)
		debug_amount += 1

def off(string):
	global debug_amount
	if DEBUG:
		print("DEB:" + "\t" * (debug_amount-1) + "/" + string)
		debug_amount -= 1

def getSubSigns():
	return ["->", "<-"] # the order is important

def containsSubSigns(string):
	for sign in getSubSigns():
		if sign in string:
			return True
	return False

def splitAtSubSigns(string):
	for sign in getSubSigns():
		if sign in string:
			return string.split(sign)
	die("splitAtSubSigns: NOPE NOPE NOPE")
	return None

def getCellEndSigns():
	global bodyAsCell
	if bodyAsCell:
		return [",", ".", ")", "("]
	else:
		return [",", ".", ")"]

def getCellBeginSigns():
	return [",", "("]

def die(arg):
	print(arg)
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
		if len(cspots)-1 < occurence:
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
			if (cellstr[thisshift] in getCellEndSigns()):
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
		self.subrules=list() # rules like a -> b. (substitution rules)
		self.relrules=list() # rules like a(b). (relation rules)
		self.importedfiles=list()

	def applySubstitution(self, relruleID, subruleID, string):
		on("applySubstitution")
		relrulestr = self.relrules[relruleID].toString()

		subrule = self.subrules[subruleID]

		args = dict()
		if string != "":
			argsstrs = string.split("?")
			for argstr in argsstrs:
				argsOfArgs = argstr.split("=")
				if len(argsOfArgs) != 2:
					die("SRLSystem::applySubstitution(): wrong argsOfArgs size (" + str(len(argsOfArgs)) + ") @ " + str(argsOfArgs))
				args[argsOfArgs[0]] = argsOfArgs[1]
		
		self.__addRulestr(subrule.substitute(relrulestr, args))
		off("applySubstitution")

	def loadFromFile(self, filename):
		self.__addRulestrsByFile(filename, sys.path[0])

	def __addRulestr(self, rulestr):
		if containsSubSigns(rulestr):
			self.subrules.append(SubRule(rulestr))
		else:
			self.relrules.append(RelRule(rulestr))

	def __addRulestrsByFile(self, filename, pwd):
		abspath = os.path.realpath(pwd + "/" + filename)
		if abspath in self.importedfiles:
			print("ignoring multiple import: \"" + abspath + "\"")
			return

		# load file
		try:
			fh = open(abspath)
		except:
			die("Couldn't load \"" + abspath + "\"")
		self.importedfiles.append(abspath)
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
		return str([x.toString() for x in self.subrules]) + " : " + str([x.toString() for x in self.relrules])

class Substitution:
	def __init__(self, cspot, newstr, rulestr):
		self.set(cspot, newstr, rulestr)

	def set(self, cspot, newstr, rulestr):
		if not isinstance(cspot, int):
			die("Substitution::init() wrong cspot data-type")
		if not isinstance(newstr, str):
			if isinstance(newstr, Cell):
				self.set(cspot, newstr.toString(), rulestr)
				return
			die("Substitution::init() wrong newstr data-type")
		if not isinstance(rulestr, str):
			if isinstance(rulestr, SubRule) or isinstance(rulestr, RelRule):
				self.set(cspot, newstr, rulestr.toString())
				return
			die("Substitution::init() wrong rulestr data-type")
		self.cspot = cspot
		self.newstr = newstr
		self.rulestr = rulestr

	def getNewRulestr(self):
		print("Substitution::getNewRulestr() TODO")
		return self.rulestr # TODO

class Cell:
	def __init__(self, arg):
		self.args=list()
		self.body=""
		self.set(arg)

	def set(self, arg):
		if isinstance(arg, Cell):
			self.set(arg.toString())
			return
		if not isinstance(arg, str):
			die("Cell::set() arg is not a string")
		string = normalizeCellstr(arg.strip("."))

		if not "(" in string:
			self.body = string
			return
		self.body = string[:string.find("(")]
		while string != self.body + "()":
			argstr = spotToCellstr(string.find("(")+1, string)
			string = (string[:string.find("(")+1] + string[string.find("(")+1+len(argstr):]).replace("(,", "(")
			self.args.append(Cell(argstr))

	def toString(self):
		if len(self.args) == 0:
			return normalizeCellstr(self.body)
		else:
			tmp = self.body + "("
			for arg in self.args:
				tmp += arg.toString() + ","
			tmp = tmp.strip(",") + ")"
			return normalizeCellstr(tmp)

class SubRule:
	def __init__(self, arg):
		self.cellstrA = None
		self.cellstrB = None
		self.sign = None
		self.set(arg)

	def set(self, arg):
		if not isinstance(arg, str):
			die("SubRule::set() arg is not a string")
		args = splitAtSubSigns(arg)
		self.cellstrA = args[0]
		self.cellstrB = args[1].strip(".")
		self.sign = arg[len(args[0]):len(arg)-len(args[1])]

	def insertArgs(self, string, args):
		for arg in args:
			string = substituteCellstr(string, -1, "{" + arg + "}", args[arg])
		return string

	def substitute(self, relrulestr, args):
		on("substitute")
		if "occur" in args:
			occurence = int(args["occur"])
		else:
			occurence = -1

		if self.sign == "<-":
			tmp = substituteCellstr(relrulestr, occurence, self.insertArgs(self.cellstrB, args), self.insertArgs(self.cellstrA, args))
		elif self.sign == "->":
			tmp = substituteCellstr(relrulestr, occurence, self.insertArgs(self.cellstrA, args), self.insertArgs(self.cellstrB, args))
		off("substitute")
		return tmp

	def toString(self):
		return self.cellstrA + self.sign + self.cellstrB + "."

class RelRule:
	def __init__(self, arg):
		self.cell = None
		self.set(arg)

	def set(self, arg):
		if isinstance(arg, Cell):
			set(arg.toString())
			return
		if not isinstance(arg, str):
			die("RelRule::set() arg is not a string")
		self.cell = Cell(arg.strip("."))

	def toString(self):
		return self.cell.toString() + "."
