## Syntax of SRL 1.4
#### In General
The code consists of **cells**: `cellname`<br />
Cells can have arguments: `cellname(argument1, argument2)`<br />
Argument-cells are seperated by commas.<br />
A cell without arguments is a **simple cell**.<br />
A cell with arguments is a **complex cell**.<br />
Integrated cells form **rules**: `a(b, c).`<br />
Rules commonly display relations between cells.<br />
Rules have to end with a dot.<br />
A **system** is just a set of rules.<br />

#### Keywords
- The `@import(<filename>)`-keyword substitutes itself by the content of the file `<filename>`.<br />
- The `equals(<a>, <b>)`-keyword displays that it is possible to substitute a by b or b by a and the rule stays valid.<br />

#### Predefined cells
- The `true`-cell: for every rule applies, that it is equal to `true`.<br />
- The `false`-cell: if `false` equals `true` (equality per `equals`) you know that a conflict exists.<br />

#### For all / For one-cells
To insert For-all-cells into rules: insert a question mark before the for-all-cell: `is_something(?x).`<br />
This means you could substitute the For-all-cell by any other cell and the rule stays valid.<br />
To insert For-one-cells into rules: insert an exclamation mark before the for-one-cell: `is_a_dog(!dog).`<br />
This means that there is at least one cell you could substitute the for-one-cell with, so that the rule stays valid<br />
and you could substitute any cell with a for-one-cell.<br />

Remember: If you do any of these substitutions you have to substitute every occurence of the substituted cell in a rule.<br />
Remember: For-all / For-one-cells can not have arguments: `?foo(bar).` would be invalid.<br />

#### Comments
To define a one-line-comment use the '#' symbol.<br />
