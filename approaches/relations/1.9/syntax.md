## Syntax of SRL 1.9
#### In General
The code consists of **cells**.<br />
Cells can be simple cells: "`cellname`"<br />
or complex cells: "`(a b c d)`"<br />
Integrated cells form **rules**: "`a (b c) d.`"<br />
Rules commonly display relations between cells.<br />
Rules have to end with a dot.<br />
A **system** is just a set of rules.<br />
<br /><br />
- The "`equals(<a>, <b>)`"-keyword displays that it is possible to substitute `a` by `b` or `b` by `a` and the rule stays valid.<br />
- Every stated rule is equal to "`[const true]`", that means that<br />"`equals a b.`" implies "`equals (equals a b) [const true].`"

#### Datatypes
- Syntax: "`[<datatype> <id>]`" <br />
- abstract datatypes:
 - Every time you change a rule so, that you add new abstract cells, you have to check, whether the new datatype and id combination already exists in that rule, if yes, you have to change the ids of the newly added abstract cells.
- There are 4 datatypes:
- Const: "`[const x]`"<br />
 - Inside of a *const* block no normal rules apply.<br />
 - So you can not substitute equal cells or anything like that.<br />
 - The equality of constants is similar to the equality of strings, you simply see if they are<br />
 - "`equals [const x] [const x]`" is "`[const true]`", but<br/>
 - "`equals [const x] [const y]`" is "`[const false]`" even if "`equals x y`".<br />
- All: "`[all x]`"<br />
 - *all* is an abstract datatype
 - You can insert any cell into a *all*-cell, if you insert it into every occurence, so<br/>"`equals [all x] y.`" implies "`equals wow y.`" and "`equals [one [const wow]] y`" and `...`.
- One: "`[one x]`"<br />
 - *one* is an abstract datatype
 - You can substitute any cell with any *one*-cell, so<br />"`equals wow y.`" implies "`equals [one x] y`."
- Hyp: "`[hyp x]`"<br />
 - *hyp* is an abstract datatype
 - *hyp* is short for hypothetical

#### Comments
To define a one-line-comment use the '#' symbol.<br />
