## Syntax of SRL 1.1
#### In General
The code consists of **cells**: `cellname`<br />
Cells can be *objects* or *relations* or simply what you want them to be.<br />
Cells can also have arguments: `cellname(argument1 argument2)`<br />
Argument-cells are seperated by spaces.<br />
Integrated cells form **rules**: `equals(cell1 cell2).`<br />
Rules commonly display relations between cells.<br />
Rules have to end with a dot.<br />
A **system** is just a set of rules.<br />

#### The means-cell
The `means(?a ?b)`-cell gives the information to the interpreter, that ?a is subsitutable by ?b.

#### Wildcards
To insert *wildcards* into rules insert a question-mark before the argument: `is_something(?x).`<br />
This means you could substitute the wildcard-cell by any other cell and the rule stays valid.<br />
Wildcard-cells can not have arguments: `?foo(bar)`<br />

#### Comments
To define a one-line-comment use the '#' symbol.<br />

#### Keywords
Keywords have to end with a dot.<br />
- The `@import(?filename)`-keyword substitutes itself by the content of the file `?filename`.<br />
- The `@print(?str)`-keyword prints `?str` and anything equal to it to stdout.<br />

#### Examples
Examples can be seen in lib/.<br />
