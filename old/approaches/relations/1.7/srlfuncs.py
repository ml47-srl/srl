#!/usr/bin/python3 -B

import srlcore

def sub(args): # sub(a, b, str, cspot)
	return srlcore.substituteCellstr(args[0].toString(), args[1].toString(), args[2].toString(), int(args[3].toString())), None
