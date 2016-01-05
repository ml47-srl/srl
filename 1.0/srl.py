#!/usr/bin/env python3

import sys
import os

def usage():
	print("Usage:\t" + os.path.basename(sys.argv[0]) + " <code>\n\t" + os.path.basename(sys.argv[0]) + " -f <filename>")
	sys.exit()

def clearline(code):
	code = code.strip("\t").strip(" ")
	if code.find("#") != -1:
		code = code[:code.find("#")]
	code = code.strip("\n")
	return code

def console(code):
	question=input(">> ")

if len(sys.argv) == 3 and sys.argv[1] == "-f":
	try:
		myfile = open(sys.argv[2])
	except:
		print("can't find file " + sys.argv[2])
		sys.exit()
	code=""
	for line in myfile.readlines():
		code += clearline(line)
	console(code)
	myfile.close()
elif len(sys.argv) == 2:
	console(clearline(sys.argv[1]))
else:
	usage()
