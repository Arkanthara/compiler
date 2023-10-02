### Limites des langages réguliers

Les langages réguliers ne suffisent pas. ex: {$a^nb^n$|$n\geq 0}: pas régulier (Pourrait servir pour vérifier si le nombre de parenthèses est correct dans une expression...)

Au fait on a besoin d'une forme de mémoire infini pour conter les _a_ et les _b_ car c'est non borné -> langage non régulier

Si la mémorisation est bornée, c'est un langage régulier

Lemme de pompage

Soit L un langage régulier. Alors il existe un entier k tq pour tout mot m de L de longueur $\geq k$, il existe des mots $x, u, y \in \Sigma^{*}$*
avec $m = xuv$, $u \neq \epsilon$ et pour $\forall n$, $xu^ny \in L$

### Langages et grammaires algébriques

#### Définition

Une grammaire algébrique ou hors-contexte est un quadruplet $G=(V_T, V_N, P, S)$ tel que
- $V_T$ et $V_N$ sont des vocabulaires disjoints: $V_T$ est l'ensemble des terminaux, $V_N$ l'ensemble des non-terminaux
- $S\inV_N$ est l'axiome, ou symbole de départ
- $P \subseteq V_N \times (V_N \cop V_T)^*)$ est l'ensemble des (règles de) productions.

La grammaire engendre des mots de $V_T^*$

Problématique: cas où on a l'embarras du choix pour appliquer les règles d'inférence -> on aura $(nombre de choix par règles)^{nombre de règles} = complexité$

#### Définition

Une dérivation est une suite de dérivations directes (Suite d'application des règles d'inférence définies pour arriver à une preuve...)

#### Définition

Soit G une grammaire algébrique d'axiome S. Le _langage engendré_ par G est défini par:
$L(G)={m \in V_T^*|S \Rightarrow^*m}$
Déterminer si un mot m appartient au langage engendré, c'est déterminer si: $S\Rightarrow_{G} m ?$

#### Définition

Dérivation directe: Soient $\beta, \beta^' \in (V_N \cop V_T)^*. $\beta$ se dérive directement en $\beta^'$ selon G, noté $\beta \Rightarrow_{G} \beta^'$, s'il existe des mots
$\gamma, \gamma^' \in (V_N \cop V_T)^* et une production $X \rightarrow \alpha$ tels que:
- $\beta = \gamma X \gamma^'$
- $\beta^' = \gamma \alpha \gamma^'$

(En gros, un symbole est remplacé par un ou plusieurs symboles...)

#### Définition

Soient $\beta, \beta^' \in (V_N \cop V_T)$.
Une suite de mots $\beta_0, \beta_1, \dots, \beta_n$  $(n \geq 0)$ est une dérivation de $\beta$ en $\beta^'$ selon G si $\beta_0 = \beta, \beta_n = \beta^'$, et
$$ \beta_0 \Rightarrow \beta_1 \Rightarrow \dots \Rightarrow \beta_n$$

#### Définition

La relation de dérivation $\Rightarrow^*_{G} est la fermeture réflexive et transitive de la relation de dérivation directe $\Rightarrow_{G}$

#### Définition

On peut trouver 2 grammaires algébriques qui engendrent le même langage.

Deux grammaires sont _équivalentes_ si elles engendrent le même langage

#### Définition

Les langages engendrés par les grammaires algébriques sont appelés _langages algébriques_

Tout langage régulier est algébrique

- possible d'utiliser une grammaire algébrique au lieu d'expressions régulières
- Mais AF plus efficaces

Grammaire régulière: peut pas avoir plus d'un symbole non terminal. Ex: A -> a régulière; A -> aB régulière; A -> aBC non régulière

### Arbres syntaxiques

Principes de l'analyse syntaxique:
- reçoit de l'analyseur lexical une suite de symboles
- reconnait dans cette suite la structure d'un texte

feuilles: symboles terminaux

noeuds: symboles non-terminaux

C'est un arbre résultant de l'analyse syntaxique.

Fait apparaître la _sturcture syntaxique_ d'un mot.

C'est une notion très importante !!!!!

On parle aussi d'_arbre de dérivation_.

#### Théorème

Soit $m \in V_T^*$. _$m \in L(G)$ ss'il existe un arbre syntaxique_ selon G dont le _mot aux feuilles est m_.

On dit alors que cet arbre est un arbre syntaxique pour m, ou que m admet cet arbre.

Répondre à $m \in L(G)$ = chercher un arbre syntaxique pour m.

En pratique; les analyseurs syntaxiques ne construisent pas un arbre syntaxique. L'arbre est simplement une représentation commode pour comprendre l'analyse syntaxique.
Ils construisent une dérivation particulière, appelée gauche ou droite.

À une dérivation correspond _un seul arbre_...

À un arbre correspond potentiellement plusieurs dérivations. Les dérivations équivalent correspondent au même arbre. Ce qui change est simplement l'ordre des dérivations.

À chaque arbre syntaxique correspond une unique _dérivation gauche_ et une unique _dérivation droite_

#### Définition

Soient $\beta, \beta^' \in ()$ et $\beta_0 \Rightarrow \beta_1 $ une dérivation de $\beta$ en $\beta^'$.Si à chaque étape on choisit de remplacer dans $\beta_i$ le non-terminal
- le plus à gauche: dérivation gauche (leftmost derivation) notée $\beta lm\Rightarrow ^* \beta^'$
- le plus à droite: dérivation droite (rightmost derivation) notée $\beta rm\Rightarrow^* \beta^'$

#### Récapitulatif

- 1 dérivation -> 1 seul arbre
- 1 arbre -> potentiellement plusieurs dérivations
- 1 arbre -> unique dérivation gauche et unique dérivation droite
- 1 mot -> potentiellement plusieurs arbres/interprétations

### Grammaires 'pathologiques'

### Chomsky -> les algébriques... et les autres


Langage algebrique: garantie d'efficacité !!!
