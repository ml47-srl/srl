# SRL 1.13

### Anmerkungen
Alles, was in `[[...]]`-Klammern steht ist eine generische Angabe<br />
und gilt f&uuml;r alles, was man dort einsetzen kann.<br />
Dies ist kein Bestandteil von SRL sondern lediglich ein Erkl&auml;rsyntax.

### Gesetze
1. Wenn es eine Regel "`= [[A]] [[B]].`" gibt,<br />
   wobei A und B Zellen sind,<br />
   darf man aus jeder Regel,<br />
   ein Vorkommnis von `[[A]]` mit `[[B]]` ersetzen und die daraus resultierende Regel gilt.<br />
   Man darf auch ein Vorkommnis von `[[B]]` mit `[[A]]` ersetzen und die daraus resultierende Regel gilt auch.

2. Wenn die Regel "`'false'.`" gilt, ist die Datenbank paradox.<br />
   Alle Informationen, die man aus der Datenbank gewonnen hat sind ung&uuml;ltig.

3. Die erste Regel jeder Datenbank ist immer: "`{a (= <a> <a>)}."`

4. Es gilt f&uuml;r alle nicht string-gleiche Konstanten A und B: "`= 'false' (= [[A]] [[B]]).`"

5. Aus jeder Regel R kann man die Regel "`= 'true' [[R]].`" ableiten.

6. Ungleiches Verhalten => Ungleiche Zelle :/

7. Wenn eine Zelle inkorrekt ist, darf sie nicht verwendet werden.<br />
   Inkorrekte Zellen:
   - Eine komplexe Zelle, welche inkorrekte Zellen enth&auml;lt
   - Eine Scope-zelle, die eine Scope-zelle mit der gleichen ID enth&auml;lt
   - Eine Scope-zelle, die eine eine inkorrekte Zelle enth&auml;lt

8. Man alle Vorkommnisse einer Var-zelle x,
   aus der Regel R, mit einer beliebigen,
   aber festen Zelle austauschen,
   wenn die Scope-zelle von x ganz au&szlig;en in R steht.
   Die &auml;u&szlig;ere Scope-zelle wird hierbei mit ihrem Body ersetzt.
   F&uuml;r jede Var-Zelle die hiernach keine Scope-zelle hat,
   muss eine neue Scope-zelle ganz au&szlig;en um die Regel gelegt werden.

9. For-one Implikation
10. Man kann anliegende Scopes austauschen
11. Man kann die ID und alle Vars einer Scope-zelle umbenennen,
    falls dadurch nicht zwei verschaltelte Scope-zellen mit gleicher ID auftreten

### Algemeine Informationen
- Vars/Constants sind keine simplen Zellen<br />
- Scopes sind auch keine komplexen Zellen<br />
- Die ID einer Scope-zelle ist ihr erstes Argument<br />
- Die ID einer Scope-zelle dient zur Zuordnung der Vars zu dem Scopes<br />

### Zellendifferenzierung
- Zellen sind eins von den folgenden: scope, complex, simple, constant, var<br />
- Scope = "`{[[simple]] [[cell]]}`"<br />
- complex = "`([[cell]] [[cell]] ...)`"<br />
- simple = Zeichenkette nur aus Buchstaben von A-Z, a-z, 0-9 und _
- constant = "`'[[simple]]'`"<br />
- var = "`<[[simple]]>`"<br />
- Alle Zellen die keinen oder mehreren dieser Definitionen entsprechen, sind inkorrekt
