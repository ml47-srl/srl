#!/usr/bin/env python3

import os
import sys
import re

usage="srl.py <file>"

class SRLParser:
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
			print("Can't import file \"" + filepath + "\"")
			sys.exit()
		importrules = SRLParser.parse(importfile.readlines(), os.path.dirname(filepath))
		importfile.close()

		for rule in importrules:
			rules.append(rule)

		return code[len(filename)+12:]

	@staticmethod
	def __parsePrintKeyword(code, rules):
		return code

	@staticmethod
	def parse(code, root): # returns list() of rules
		rules=list()
		code = SRLParser.__uncomment(code)
		while code != "": # as long as code has to be converted to a rule
			if code.startswith("@"):
				if code.startswith("@import("):
					return SRLParser.__parseImportKeyword(code, rules, root)
				elif code.startswith("@print("):
					return SRLParser.__parsePrintKeyword(code, rules)
				else:
					print("invalid keyword \"" + code + "\"") # TODO
					sys.exit()
			else:
				print("Can't parse code \"" + code + "\"") # TODO
				sys.exit()
		return rules

if len(sys.argv) != 2:
	print(usage)
	sys.exit()
else:
	try:
		openfile = open(sys.argv[1])
	except:
		print("file \"" + sys.argv[1] + "\" not found")
		sys.exit()
	rules=SRLParser.parse(openfile.readlines(), os.path.dirname(sys.argv[1]))
	print(rules) # TODO remove
	openfile.close()
