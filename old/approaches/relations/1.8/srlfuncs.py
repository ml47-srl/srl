#!/usr/bin/python3 -B

import srlcore

def sub(args): # sub(a, b, str, cspot)
	return srlcore.substituteCellstr(args[0].toString(), args[1].toString(), args[2].toString(), int(args[3].toString())), None

def streq(args):
	if args[0].toString() == args[1].toString():
		return "true", None
	elif ("{" in args[0].toString()) or ("{" in args[1].toString()):
		return "dunno", None
	else:
		return "false", None
