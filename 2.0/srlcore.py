#!/usr/bin/python3.4

import sys

def getSubSigns():
	return ["->"]

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
		print("loadFromFile() TODO")

	def __getRulesByFile(self, filename, pwd):
		print()	

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
		set(arg)

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
		return normalizeString("TODO")

class SubRule:
	def __init__(self, arg):
		self.set(arg)

	def set(self, arg):
		if not isinstance(arg, str):
			die("SubRule::set() arg is not a string")
		args = arg.split(getSubSigns())
		self.cellA = Cell(args[0])
		self.cellB = Cell(args[1])
		self.sign = arg[len(args[0]):len(arg)-len(args[1])] # unsure

	def allowsSubstitution(self, sub):
		return False

	def doesStringContainSubSign(string):
		if not isinstance(string, str):
			die("SubRule::doesStringContainSubSign() string is not a string")
		return getSubSigns() in string

class RelRule:
	def __init__(self, arg):
		set(arg)

	def set(self, arg):
		if isinstance(arg, Cell):
			set(arg.toString())
			return
		if not isinstance(arg, str):
			die("RelRule::set() arg is not a string")
		self.cell = Cell(arg)
