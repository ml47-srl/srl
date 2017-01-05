# SRL 1.10

## 1. True-rule-tausch:
Man darf "true" mit jeder bewiesenen Regel austauschen und umgekehrt.
## 2. Equals-tausch:
Man darf die Zellen A und B vertauschen, wenn gilt `equals A B`.
## 3. Const-equals-definition:
Konstantengleichungen (z.B. `equals "a" "b"`), darf man mit "true" vertauschen (und umgekehrt),<br />
wenn die Konstanten string-gleich sind. Andernfalls gilt die Vertauschung mit "false".
## 4. All-Zellen:
#### Syntax:
All-zellen:<br />
&nbsp;&nbsp;`[all <id> <body>]` oder<br />
&nbsp;&nbsp;`[a <id> <body>]`<br />
All-variablen<br />
&nbsp;&nbsp;`[var <all-id> <body>]` oder<br />
&nbsp;&nbsp;`[v <all-id> <body>]`<br /><br />
ID-Gleichheit ist Zellengleichheit, und nicht String-gleichheit, deshalb werden Konstanten als IDs empfohlen.<br />
#### 4.1: Es darf in einer Regel keine zwei all-Zellen mit der gleichen ID geben
#### 4.2: Jede Variable darf nur im Body der All-Zelle vorkommen, in welcher die id der All-Zelle der all-id der All-Variable entspricht.
## 5. All-Einsetzung:
Wenn die All-Zelle die &auml;u&szlig;erste Zelle einer Regel ist, darf man Werte f&uuml;r die Variablen dieser All-Zelle einsetzen.<br />
Dabei muss jedes Vorkommen der Variable ersetzt werden.<br />
## 6. Leere All-Zellen:
Eine All-Zelle, die keine eigenen All-Variablen enth&auml;lt darf durch ihren eigenen Body ersetzt werden.<br />
Gleicherma&szlig;en darf auch jede Zelle sich selbst durch eine variablenlose All-Zelle ersetzen, die die Ursprungszelle als Body beh&auml;lt.<br />
## 7. All-ID-Wechsel:
Eine All-Zelle kann ihre ID und damit auch die All-IDs ihrer Variablen &auml;ndern, solange 4.1 dadurch nicht verletzt wird.<br />
## 8. Forone-Implikation:
Man kann aus jeder bewiesenen Regel `x` eine neue Regel `not [a y (not x)]` ableiten, und dabei  einige Zellen aus `x` zu Variablen von `y` umwandeln.<br />
Ich darf hierbei nicht aus unterschiedlichen Zellen gleiche Variablen machen.
