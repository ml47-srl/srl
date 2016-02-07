## Syntax of SRL 1.3
#### In General
The code consists of **cells**: `cellname`<br />
Cells can have arguments: `cellname(argument1, argument2)`<br />
Argument-cells are seperated by commas.<br />
A cell without arguments is a **simple cell**.<br />
A cell with arguments is a **complex cell**.<br />
Integrated cells form **rules**: `equals(cell1, cell2).`<br />
Rules commonly display relations between cells.<br />
Rules have to end with a dot.<br />
A **system** is just a set of rules.<br />

#### The True-cell
The `true`-cell: for every rule applies, that it is equal to true.<br />

#### For all / For one-cells
To insert For-all-cells into rules: insert a question mark before the for-all-cell: `is_something(?x).`<br />
This means you could substitute the For-all-cell by any other cell and the rule stays valid.<br />
To insert For-one-cells into rules: insert an exclamation mark before the for-one-cell: `is_a_dog(!dog).`<br />
This means that there is at least one cell you could substitute the for-one-cell with, so that the rule stays valid.<br />

For-all / For-one-cells can not have arguments: `?foo(bar).` would be invalid.<br />

#### Comments
To define a one-line-comment use the '#' symbol.<br />

#### Keywords
Keywords have to end with a dot.<br />
- The `@import(?filename)`-keyword substitutes itself by the content of the file `?filename`.<br />
- The `@print(?str)`-keyword prints `?str` to stdout.<br />
- The `@error(?str)`-keyword stops the program and prints `?str` to stdout.<br />
It is commonly used for finding logical conflicts: eg. `implies(equals(false true) @error("ERROR: false is true")).`<br />
