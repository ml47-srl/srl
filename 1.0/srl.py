#!/usr/bin/env python3

import os
import sys

usage="srl.py <file>"

def uncommentSRL(lines):
	result=""
	for line in lines:
		if line.find("#") != -1: # if a comment exists
			line = line[:line.find("#")] # clear it
		line = line.strip() # removes whitespaces tabs and endlines from beginning and end of line
		result += line
	return result

def parseSRL(lines):
	print(uncommentSRL(lines))

if len(sys.argv) != 2:
	print(usage)
	sys.exit()
else:
	try:
		openfile = open(sys.argv[1])
	except:
		print("file " + sys.argv[1] + " not found")
		sys.exit()
	parseSRL(openfile.readlines())
	openfile.close()
