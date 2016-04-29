#!/usr/bin/python3

import sys
import os

sys.path.append(os.path.realpath(sys.path[0] + "/.."))
from srlcore import *

def pipe(srcrulestr, argstrs, vars):
	return srcrulestr, None

def sub(srcrulestr, argstrs, vars):
	if "occurence" in vars:
		occurence = int(vars["occurence"])
	else:
		occurence = -1
	return substituteCellstr(srcrulestr, occurence, argstrs[0], argstrs[1]), None
