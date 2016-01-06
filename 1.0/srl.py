#!/usr/bin/env python3

import os
import sys

usage="srl.py <file>"

def uncommentSRL(lines):
	result=""
	for line in lines.split("\n"):
		if line.find("#") != -1:
			line = line[:line.find("#")]
		else:
			line = line.strip("\n")
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
	lines=""
	for line in openfile.readlines():
		lines += line
	openfile.close()
	parseSRL(lines)
