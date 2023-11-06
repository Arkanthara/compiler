# Caractérisation d'une grammaire LL(1)

Une grammaire est LL(1) si chaque case contient exactement une production ou erreur.
Une grammaire LL(1) n'est pas ambigüe

Il peut y avoir possibilité de soigner une grammaire qui n'est pas LL(1) pour la transformer en grammaire LL(1).

Tant qu'on peut factoriser, on doit factoriser pour que la grammaire soit LL(1).

On peut transformer toutes grammaires LL(k) en grammaire LL(1) pour k fixé.
Quand c'est pas e-productif, on n'a pas besoin d'aller chercher chez le suivant...

LL(*) ne fixe pas une taille de look-ahead a priori
Il existe aussi les grammaires eBNF...

# Analyseurs LR

L'analyseur LR(1) est le plus puissant.

## Analyseur ascendant

Ici, on part du mot à reconnaitre, puis on va effectuer des lectures et des réductions.
LR(k): from Left to right, Right derivation

Analyseur LR(0) basé sur une variante de l'automate des items
