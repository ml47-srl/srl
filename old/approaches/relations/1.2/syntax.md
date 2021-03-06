## Syntax of SRL 1.2
#### In General
The code consists of **cells**: `cellname`<br />
Cells can be *objects* or *relations* or simply what you want them to be.<br />
Cells can also have arguments: `cellname(argument1 argument2)`<br />
Argument-cells are seperated by spaces.<br />
Integrated cells form **rules**: `equals(cell1 cell2).`<br />
Rules commonly display relations between cells.<br />
Rules have to end with a dot.<br />
A **system** is just a set of rules.<br />

#### The True-cell
The `true`-cell: for every rule applies, that it is equal to true.<br />

#### Wildcards
To insert *wildcards* into rules insert a question-mark before the argument: `is_something(?x).`<br />
This means you could substitute the wildcard-cell by any other cell and the rule stays valid.<br />
Wildcard-cells can not have arguments: `?foo(bar)`<br />

#### Comments
To define a one-line-comment use the '#' symbol.<br />

#### Keywords
Keywords have to end with a dot.<br />
- The `@import(?filename)`-keyword substitutes itself by the content of the file `?filename`.<br />
- The `@print(?str)`-keyword prints `?str` to stdout.<br />
- The `@error(?str)`-keyword stops the program and prints `?str` to stdout.<br />
It is commonly used for finding logical conflicts: eg. `implies(equals(false true) @error("ERROR: false is true")).`<br />
