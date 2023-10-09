#### Grammaires ambiguës

On peut avoir 2 interprétations (sémantique) différentes d'un mot ambigu.

#### Définition

Un mot ambigu possède plusieurs arbres syntaxiques

Une grammaire est ambiguë si elle permet de dériver au moins un mot ambigu.

Un langage est ambigu si toutes les grammaires engendrées par ce langage sont ambiguës

On ne travaille que avec des grammaires non ambiguës !!!

Certaines grammaires sont clairement ambiguës, et d'autres sont mal conçues

$G_1$: on ne peut pas ajouter $\epsilon$ où on veut !!!!

$G_2$: on peut ajouter $\epsilon$ où on veut !!!

Décider de l'ambiguité d'une grammaire est indécidable !!!!

- non terminal par niveau

- moins prioritaires en haut

- plus prioritaires en bas

- axiomes et terminaux tout en bas

- associativité gauche\droite $\Rightarrow$ récursivité gauche\droite



Tout symbole non terminal doit se transformer en symbole terminal. Si c'est pas le cas, on dit que c'est improductif....




