## Syntax of SRL 1.5
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

#### Constants
- **Constants** are simple cells in double-quotes: `"fairy"`<br />
- Every constant is inequal to each other constant<br />
- Every cell that is not a constant is called a **Symbolic**<br />

#### Predefined cells
- The `"true"`-constant: for every rule applies, that it is equal to `"true"`.<br />

#### For all
To insert For-all-cells into rules: insert a question mark before the for-all-cell: `is_something(?x).`<br />
This means you could substitute the For-all-cell by any other cell and the rule stays valid.<br />

Remember: If you do any of these substitutions you have to substitute every occurence of the substituted cell in a rule.<br />
Remember: For-all / For-one-cells can not have arguments: `?foo(bar).` would be invalid.<br />

#### Comments
To define a one-line-comment use the '#' symbol.<br />
