Construction de automate LR-AFD

On construit l'automate caractéristique

On sature les états par expansion...

Attention, cela reste un automate à pile et il faut se fier à la pile !!!!

L'automate LR(0) peut ne pas être déterministe ! conflit reduce / reduce (on ne sait pas où aller après la réduction) et conflit shift reduce (on ne sait pas si on doit lire ou réduire)...
Il faut qu'il y ait un seul choix possible... Quelqu'un doit décider du choix !!!!

## Table des actions....
d: décalage (p42)
e: erreur
a: accept
red: on réduit

Rem: réduction effectuée quelque soit la tête de lecture

LR(0): une seule action possible ou erreur... Il est par construction déterministe...

On peut essayer une analyse LR(1), et pour cela, on va commencer par faire une analyse SLR(1) (Simple LR(1))....

SLR(1) prend en compte le symbole sous la tête de lecture pour décider d'une réduction...
Sait pas choisir entre décalage et réduction.... Conflit  S/R au sens SLR(1)....
Conflit R/R: si on a 2 réductions dans un même état et que l'intersection des 2 suivants n'est pas vide. alors conflit....
SLR(1): une seule action ou erreur par case....
Une grammaire ambig!ue n'est ni LR(0), ni SLR(1)

Tables des successeurs entre LR(0) et SLR(1) identique, mais table des actions est différente
