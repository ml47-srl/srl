#!/usr/bin/env python3

import os
import sys
import re

usage="srl.py <file>"

class SRLParser:
	@staticmethod
	def __checkConflicts(rules): # TODO
		return

	@staticmethod
	def __uncomment(code): # uncomments; strips
		result=""
		for line in code:
			if line.find("#") != -1: # if a comment exists
				line = line[:line.find("#")] # clear it
			line = line.strip() # removes whitespaces tabs and endlines from beginning and end of line
			result += line
		return result

	@staticmethod
	def __parseImportKeyword(code, rules, root):
		match = re.match(r'^@import\("([^"]|\\")+"\)\.', code)
		if match == None:
			print("Can't parse code (@import) \"" + code + "\"") # TODO
			sys.exit()
		filename = match.group()[9:-3]
		filepath = root + "/" + filename
		try:
			importfile=open(filepath)
		except:
			print("Can't import file \"" + os.path.realpath(filepath) + "\"")
			sys.exit()
		importrules = SRLParser.__parse(importfile.readlines(), os.path.dirname(filepath))
		importfile.close()

		for rule in importrules:
			rules.append(rule)

		return code[len(filename)+12:]

	@staticmethod
	def __parsePrintKeyword(code, rules): # TODO
		dotpos = code.find(".")
		if dotpos != -1:
			rules.append(code[:dotpos+1])
			return code[dotpos+1:]
		else:
			print("Can't find end of Rule (@print) \"" + code + "\"")
			sys.exit()

	@staticmethod
	def __parseRule(code, rules): # TODO
		dotpos = code.find(".")
		if dotpos != -1:
			rules.append(code[:dotpos+1])
			return code[dotpos+1:]
		else:
			print("Can't find end of Rule \"" + code + "\"")
			sys.exit()


	@staticmethod
	def __parse(code, root):
		rules=list()
		code = SRLParser.__uncomment(code)
		while code != "": # as long as code has to be converted to a rule
			if code.startswith("@"):
				if code.startswith("@import("):
					code = SRLParser.__parseImportKeyword(code, rules, root)
				elif code.startswith("@print("):
					code = SRLParser.__parsePrintKeyword(code, rules)
				else:
					print("invalid keyword \"" + code + "\"") # TODO
					sys.exit()
			else:
				code = SRLParser.__parseRule(code, rules)
		return rules

	@staticmethod
	def parse(code, root): # returns list() of rules
		rules = SRLParser.__parse(code, root)
		nonkeywordrules=list()
		for rule in rules:
			if (not rule.startswith("@")):
				nonkeywordrules.append(rule)
		SRLParser.__checkConflicts(nonkeywordrules)
		return rules

class SRLExecutor:
	@staticmethod
	def __executePrint(rules, argument):
		results=list()
		results.append(argument)
		# TODO find other results
		print("(" + ", ".join(results) + ")")

	@staticmethod
	def execute(rules): # TODO
		newrules=list()
		for rule in rules:
			if rule.startswith("@print("):
				SRLExecutor.__executePrint(newrules, rule[7:-2])
			else:
				newrules.append(rule)

if len(sys.argv) != 2:
	print(usage)
	sys.exit()
else:
	try:
		openfile = open(sys.argv[1])
	except:
		print("file \"" + sys.argv[1] + "\" not found")
		sys.exit()
	SRLExecutor.execute(SRLParser.parse(openfile.readlines(), os.path.dirname(sys.argv[1])))
	openfile.close()
