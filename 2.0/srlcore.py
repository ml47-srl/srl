#!/usr/bin/python3.4

import sys
import os

def getSubSigns():
	return ["->", "<->"]

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

def die(arg):
	print(arg)
	sys.exit()

def normalizeString(string):
	return string.replace("\t", "").replace(" ", "").replace("\n", "")

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
		print()

	def loadFromFile(self, filename):
		rules = self.__getRulesByFile(filename, sys.path[0])
		for rule in rules:
			if isinstance(rule, SubRule):
				self.subrules.append(rule)
			elif isinstance(rule, RelRule):
				self.relrules.append(rule)
			else:
				die("loadFromFile: NOPE NOPE NOPE")

	def __getRulesByFile(self, filename, pwd):
		abspath = os.path.realpath(pwd + "/" + filename)
		if abspath in self.importedfiles:
			return
		rules=list()
		# load file
		try:
			fh = open(abspath)
		except:
			die("Couldn't load \"" + abspath + "\"")
		self.importedfiles.append(abspath)
		lines=fh.readlines()
		fh.close()
		# 

		# concat all lines
		alllines=""
		for line in lines:
			if line.find("#") != -1:
				line = line[0:line.find("#")]
			alllines += normalizeString(line)
		# check for imports
		while "@import(" in alllines:
			start = alllines.find("@import(\"")
			for i in range(start+9, len(alllines)):
				if alllines[i] == "\"" and alllines[i-1] != "\\":
					end=i
					break
			newfile = alllines[start+9:end]
			for rule in self.__getRulesByFile(newfile, os.path.dirname(abspath)):
				rules.append(rule)
			alllines = alllines[:start] + alllines[end+3:]
		# check for rules
		while "." in alllines:
			x = alllines[:alllines.find(".")+1]
			if containsSubSigns(x):
				self.subrules.append(SubRule(x))
			else:
				self.relrules.append(RelRule(x))
			alllines = alllines[alllines.find(".")+1:]
		return rules

	def toString(self):
		return str([x.toString() for x in self.subrules]) + " : " + str([x.toString() for x in self.relrules])

class Substitution:
	def __init__(self, subject, replacement, rule):
		if not isinstance(subject, Cell):
			die("Substitution::init() wrong subject data-type")
		if not isinstance(replacement, Cell):
			die("Substitution::init() wrong replacement data-type")
		if not isinstance(rule, str):
			die("Substitution::init() wrong rule data-type")
		self.subject = subject
		self.replacement = replacement
		self.rule = rule

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
		string = normalizeString(arg)

		if not "(" in string:
			self.body = string
			return
		self.body = string[:string.find("(")]
		while string != self.body + "()":
			# getCellAt
			index = string.find("(")+1
			cellname = ""
			openparens = 0
			while True:
				if string[index] == "(":
					openparens += 1
				elif string[index] == ")":
					openparens -= 1
				cellname += string[index]
				index += 1
				if len(string) < index+1:
					argstr = cellname
					break
				elif openparens == 0 and (string[index] in getCellEndSigns()):
					argstr = cellname
					break
			# /getCellAt
			string = (string[:string.find("(")+1] + string[string.find("(")+1+len(argstr):]).replace("(,", "(")
			self.args.append(Cell(argstr))

	def toString(self):
		if len(self.args) == 0:
			return normalizeString(self.body)
		else:
			tmp = self.body + "("
			for arg in self.args:
				tmp += arg.toString() + ","
			tmp = tmp.strip(",") + ")"
			return normalizeString(tmp)

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
		self.cellB = Cell(args[1])
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
		self.cell = Cell(arg)

	def toString(self):
		return self.cell.toString() + "."
