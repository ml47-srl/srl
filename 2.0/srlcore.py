#!/usr/bin/python3.4 -B

import sys
import os

def getSubSigns():
	return ["<->", "->", "<-"] # the order is important

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
	return [",", ".", ")"]

def getCellBeginSigns():
	return [",", "("]

def die(arg):
	print(arg)
	sys.exit()

def normalizeCellstr(string):
	return string.replace("\t", "").replace(" ", "").replace("\n", "")

def spotToCellstr(spot, string):
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
			return cellname
		elif openparens == 0 and (string[spot] in getCellEndSigns()):
			return cellname

def cspotToSpot(cspot, string):
	cells=0
	for i in range(len(string)):
		if cells == cspot:
			return i
		if string[i] in getCellBeginSigns():
			cells += 1

def spotToCspot(spot, string):
	cells=0
	for i in range(len(string)):
		if i == spot:
			return cells
		if string[i] in getCellBeginSigns():
			cells += 1

def cspotToCellstr(cspot, string):
	return spotToCellstr(cspotToSpot(cspot, string), string)

class SRLSystem:
	def __init__(self):
		self.subrules=list() # rules like a -> b. (substitution rules)
		self.relrules=list() # rules like a(b). (relation rules)
		self.importedfiles=list()

	def tryToApplySubstitution(self, sub):
		for subrule in self.subrules:
			if subrule.allowsSubstitution(sub):
				self.__applySubstitution(sub)
				return

	def __applySubstitution(self, sub):
		self.__addRulestr(sub.getNewRulestr())

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
		self.cellA = None
		self.cellB = None
		self.sign = None
		self.set(arg)

	def set(self, arg):
		if not isinstance(arg, str):
			die("SubRule::set() arg is not a string")
		args = splitAtSubSigns(arg)
		self.cellA = Cell(args[0])
		self.cellB = Cell(args[1].strip("."))
		self.sign = arg[len(args[0]):len(arg)-len(args[1])] # unsure

	def allowsSubstitution(self, sub):
		return False

	def toString(self):
		return self.cellA.toString() + self.sign + self.cellB.toString() + "."

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
