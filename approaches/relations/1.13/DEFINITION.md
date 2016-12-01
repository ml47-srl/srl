# SRL 1.13

### Anmerkungen
Alles, was in [[]]-Klammern steht ist eine generische Angabe<br />
und gilt f&uuml;r alles, was man dort einsetzen kann.<br />
Dies ist kein Bestandteil von SRL sonder lediglich ein Erkl&auml;rsyntax.

1. Wenn es eine Regel "`= [[A]] [[B]].`" gibt,<br />
   wobei A und B Zellen sind,<br />
   darf man aus jeder bewiesenen Regel,<br />
   ein Vorkommnis von `[[A]]` mit `[[B]]` ersetzen und die daraus resultierende Regel gilt.<br />
   Man darf auch ein Vorkommnis von `[[B]]` mit `[[A]]` ersetzen und die daraus resultierende Regel gilt auch.

2. Wenn die Regel `'false'.` gilt, ist die Datenbank paradox.<br />
   Alle Informationen, die man aus der Datenbank gewonnen hat sind hinf&auml;llig.

3. Die erste bewiesene Regel jeder Datenbank ist immer: "`{a (= <a> <a>)}."`

4. Es gilt f&uuml;r alle nicht string-gleiche Konstanten A und B: "`= 'false' (= [[A]] [[B]]).`"

5. Aus jeder bewiesenen Regel R kann man die Regel "`= 'true' [[R]].`" ableiten.

6. Ungleiches Verhalten => Ungleiche Zelle :/

7. Wenn eine Zelle inkorrekt ist, darf sie nicht verwendet werden.<br />
   Inkorrekte Zellen:
   - Eine komplexe Zelle, welche inkorrekte Zellen enth&auml;lt
   - Eine Scope-zelle, die eine Scope-zelle mit der gleichen ID enth&auml;lt
   - Eine Scope-zelle, die eine eine inkorrekte Zelle enth&auml;lt

8. Man alle Vorkommnisse einer Var-zelle x,
   aus der Regel R, mit einer beliebigen,
   aber festen Zelle austauschen,
   wenn die Scope-zelle von x ganz außen in R steht.
   Die äußere Scope-zelle wird hierbei mit ihrem Body ersetzt.
   Für jede Var-Zelle die hiernach keine Scope-zelle hat,
   muss eine neue Scope-zelle ganz außen um die Regel gelegt werden.

9. For-one Implikation
10. Man kann aliegende Scopes austauschen
11. Man kann Scope-vars umbenennen, falls Regel 7 damit nicht kollidiert.

Algemeine Informationen:<br />
- Zellen sind eins von den folgenden: scope, complex, simple, constant, var<br />
- Scope = "{[[simple]] [[cell]]}"<br />
- complex = "([[cell]] [[cell]] ...)"<br />
- simple = "([A-Za-z0-9]_)+"<br />
- constant = "'[[simple]]'"<br />
- var = "<[[simple]]>"<br />
- Vars/Constants sind keine simplen Zellen<br />
- Scopes sind auch keine komplexen Zellen<br />
- Die ID einer Scope-zelle ist ihr erstes Argument<br />
- Die ID einer Scope-zelle dient zur Zuordnung der Vars zu dem Scopes<br />
