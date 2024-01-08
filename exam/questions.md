# Principes généraux

## Q1.1 Analyseur lexical, qu'est ce que c'est ?

L'analyse lexicale lit les caractères du source, et détermine la `séquence des terminaux` le composant.

Le lexique d'un langage définit les mots qui le composent

- De manière explicite, en extension, comme: `( procedure if >= . } ;` etc...
- De manière générique, en compréhension, comme:
  - identificateur
  - chaînes de caractères
  - constantes numériques

(Les langues naturelles ont par exemple des règles génériques pour le nombre (singulier-pluriel) et le genre (masculin-féminin)

En informatique, on parle de symboles terminaux. La justification de ce terme se verra sur les arbres de dérivation.

Cela conduit souvent à des grammaires à 2 niveaux: 

- grammaire définissant les classes de terminaux
- niveau syntaxique lui-même

## Q1.2 Analyseur syntaxique, qu'est ce que c'est ?

L'analyse syntaxique vérifie que la structure de cette séquence (définie par l'analyseur lexical) est coforme à la syntaxe du langage

Principes de l'analyse syntaxique:

- reçoit de l'analyseur lexical une suite de symboles
- reconnait dans cette suite la structure d'un texte

La construction d'un arbre syntaxique permet uniquement d'avoir une représentation commode pour comprendre l'analyse syntaxique.

feuilles: symboles terminaux

noeuds: symboles non-terminaux

C'est un arbre résultant de l'analyse syntaxique.

Fait apparaître la _sturcture syntaxique_ d'un mot.

C'est une notion très importante !!!!!

On parle aussi d'_arbre de dérivation_.

Soit $ m \in V_{T}^* $.
alors $m \in L(G)$ ss'il existe un arbre syntaxique selon G dont le mot aux feuilles est m.

On dit alors que cet arbre est un arbre syntaxique pour m, ou que m admet cet arbre.

Répondre à $m \in L(G)$ = chercher un arbre syntaxique pour m.

En pratique; les analyseurs syntaxiques ne construisent pas un arbre syntaxique. L'arbre est simplement une représentation commode pour comprendre l'analyse syntaxique.
Ils construisent une dérivation particulière, appelée gauche ou droite.

À une dérivation correspond _un seul arbre_...

À un arbre correspond potentiellement plusieurs dérivations. Les dérivations équivalent correspondent au même arbre. Ce qui change est simplement l'ordre des dérivations.

À chaque arbre syntaxique correspond une unique _dérivation gauche_ et une unique _dérivation droite_

La syntaxe régit la forme des phrases

- les phrases acceptables au vu de la définition syntaxique appartiennent au langage
- les autres n'y appartiennent pas

La définition syntaxique d'un langage s'appuie sur:

- les terminaux, définis au niveau lexical
- des règles de bonne forme pour des séquences de terminaux, appelées `notions non-terminales`

La justification du terme `non-terminal` se verra sur les arbres de dérivation.

Toute description grammaticale textuelle d'un langage s'appuie sur une syntaxe spécifique, comme par exemple: (Bison)

Expression:                     non-terminal
    Expression PLUS Terme;      PLUS est un terminal

Dans cette exemple, on décrit un fragment de la syntaxe d'expressions algébriques usuelles au moyen d'une spécification écrite

## Rappel

Sémantique: signification véhiculée par les phrases du langage

analyse sémantique: contrôle la signification du code source

Sur-langage d'un langage donné: contient toutes les phrases de ce langage.

statique: à la compilation

dynamique: à l'exécution

(S'applique au typage)

Selon le langage, on mène toutes les tâches de compilation

- de front en une passe: on ne fait qu'un passage sur le texte source
- successivement en plusieurs passes: on fait des passes successives sur des formes intermédiaires représentant le code source.
  Certaines créent des structures de données utilisées par des passes ultérieures.
  On peut ainsi faire une analyse sémantique plus fine et du meilleur code objet.

`Tendance moderne` de compilation: contrôler le plus de choses statiquement, à la compilation,

Un `auto-interprète` ou `interprète méta-circulaire` est écrit dans le langage dont il peut interpréter les programmes.

Un `auto-compilateur` est écrit dans le langage qu'il peut compiler. => problème du bootstrap ou amorçage de la pompe...
Si on veut écrire le compilateur dans son propre langage S

- écrire d'abord un compilateur d'un sous-ensemble S' de S en E, et le compiler
- écrire ensuite en S' le compilateur de S
- c'est ce qu'on appelle un bootstrap du compilateur

Générateur de compilateurs: C'est une grammaire que l'on compile pour obtenir un compilateur

## Notation postfixée

Incontournable !

C'est une écriture linéaire d'un graphe qui décrit l'ordre d'évaluation des cupérandes et cupérateurs.

### Fonction stricte

Évalue toujours tous ses arguments d'appel

Ne peut donc être évaluée si l'un des arguments d'appel ne peut pas l'être

Un exemple typique de fonction non stricte est la conditionnelle "Si":

- elle évalue toujours son premier argument d'appel, la condition
- elle n'évalue que l'un ou (exclusif) l'autre de ses deuxièmes et troisième arguments, selon la valeur de la condition

# Grammaires algébriques

### Limites des langages réguliers

Les langages réguliers ne suffisent pas.
ex: {$a^n b^n $| $n \geq 0$}: pas régulier (Pourrait servir pour vérifier si le nombre de parenthèses est correct dans une expression...)

Au fait on a besoin d'une forme de mémoire infini pour conter les _a_ et les _b_ car c'est non borné -> langage non régulier

Si la mémorisation est bornée, c'est un langage régulier

Lemme de pompage

Soit L un langage régulier. Alors il existe un entier k tq pour tout mot m de L de longueur $\geq k$, il existe des mots $x, u, y \in \Sigma^* $
avec $m = xuv$, $u \neq \epsilon$ et pour $\forall n$, $xu^ny \in L$

## Q3.1: Définir formellement ce qu'est une grammaire

Une grammaire algébrique ou hors-contexte est un quadruplet
$G=(V_T, V_N, P, S)$

- $V_T$ et $V_N$ sont des vocabulaires disjoints: $V_T$ est l'ensemble des terminaux, $V_N$ l'ensemble des non-terminaux
- $S \in V_N$ est l'axiome, ou symbole de départ
- $P \subseteq V_N \times (V_N \cup V_T)^* $ est l'ensemble des (règles de) productions.

La grammaire engendre des mots de $V_T$

## Définitions

#### Dérivation

Une dérivation est une suite de dérivations directes (Suite d'application des règles d'inférence définies pour arriver à une preuve...).
On peut avoir plusieurs dérivations pour un même mot.

#### Langage engendré par une grammaire

Soit G une grammaire algébrique d'axiome S. Le _langage engendré_ par G est défini par:
$L(G)={m \in V_T^* |S \Rightarrow^* m}$
Déterminer si un mot m appartient au langage engendré, c'est déterminer si: $S\Rightarrow_{G}^* m ?$

#### Dérivation directe (formellement)

Dérivation directe: Soient $\beta, \beta' \in (V_N \cup V_T)^*$ . $\beta$ se dérive directement en $\beta'$ selon G, noté $\beta \Rightarrow_{G} \beta'$, s'il existe des mots
$\gamma, \gamma' \in (V_N \cup V_T)^* $ et une production $X \rightarrow \alpha$ tels que:

- $\beta = \gamma X \gamma'$
- $\beta' = \gamma \alpha \gamma'$

(En gros, un symbole est remplacé par un ou plusieurs symboles...)

#### Dérivation (formellement)

Soient $\beta, \beta' \in (V_N \cup V_T)$.
Une suite de mots $\beta_0, \beta_1, \dots, \beta_n$  $(n \geq 0)$ est une dérivation de $\beta$ en $\beta'$ selon G si $\beta_0 = \beta, \beta_n = \beta'$, et
$$ \beta_0 \Rightarrow \beta_1 \Rightarrow \dots \Rightarrow \beta_n$$

#### Relation de dérivation

La relation de dérivation $\Rightarrow_{G}^* $ est la fermeture réflexive et transitive de la relation de dérivation directe $\Rightarrow_{G}$

#### Langages algébriques

Les langages engendrés par les grammaires algébriques sont appelés _langages algébriques_

#### Grammaires équivalentes

On peut trouver 2 grammaires algébriques qui engendrent le même langage.

Deux grammaires sont _équivalentes_ si elles engendrent le même langage

#### Remarques

Tout langage régulier est algébrique

- possible d'utiliser une grammaire algébrique au lieu d'expressions régulières
- Mais AF plus efficaces

Grammaire régulière: peut pas avoir plus d'un symbole non terminal. Ex: A -> a régulière; A -> aB régulière; A -> aBC non régulière

#### Dérivation gauche / droite (formellement)

Soient $\beta, \beta' \in (V_N \cup V_T)^* $ et $\beta_0 \Rightarrow \beta_1 $ une dérivation de $\beta$ en $\beta'$.Si à chaque étape on choisit de remplacer dans $\beta_i$ le non-terminal

- le plus à gauche: dérivation gauche (leftmost derivation) notée $\beta lm\Rightarrow ^* \beta'$
- le plus à droite: dérivation droite (rightmost derivation) notée $\beta rm\Rightarrow^* \beta'$

#### Récapitulatif

- 1 dérivation -> 1 seul arbre
- 1 arbre -> potentiellement plusieurs dérivations
- 1 arbre -> unique dérivation gauche et unique dérivation droite
- 1 mot -> potentiellement plusieurs arbres/interprétations

## Q3.2: Classification des grammaires

Classification de Chomsky

$\text{régulier} \subset \text{algébrique} \subset \text{contextuel} \subset \text{arbitraire}$

Les grammaires algébriques expriment:

- les prcupriétés structurelles des langages
- mais pas leurs prcupriétés contextuelles
- on les appelle aussi grammaires hors contexte

Par exemple, on ne peut pas exprimer par une grammaire algébrique

- que toute variable utilisée a été déclarée
- les vérifications de typage etc...
  Ces prcupriétés relèvent de l'analyse sémantique

Pour être sensible au contexte, il faut utiliser une grammaire contextuelle (Productions de la forme $\alpha \rightarrow \beta$ avec $|\alpha| \leq |\beta|$ et $\alpha, \beta \in (V_N \cup V_T)^* $

Ex: AB $\rightarrow$ BA

Les grammaires contextuelles

- engendrent les langages contextuels
- ne sont pas utilisées lors de l'analyse syntaxique
- pas d'algorithme polynomial connu qui, pour tout mot, détermine si ce mot est engendré par une grammaire contextuelle donnée.

Il existe des grammaires arbitraires.
Productions de la forme $\alpha \rightarrow \beta$ avec $\alpha, \beta \in (V_N \cup V_T)^* $

Ex: AB $\rightarrow \epsilon$

Ces grammaires engendrent l'ensemble de tous les langages.

## Q3.3: Grammaires ambigües

Un mot est ambigü s'il admet plusieurs arbres syntaxiques.

Une grammaire est ambiguë si elle permet de dériver au moins un mot ambigu.

Un langage est ambigu si toutes les grammaires qui l'engendrent sont ambiguës.

On ne travaille que avec des grammaires non ambiguës !!!

Certaines grammaires sont clairement ambiguës, et d'autres sont mal conçues

$G_1$

$\text{listInst} \rightarrow \epsilon | \text{inst listInst}$
$\text{inst} \rightarrow \text{affect PV} | \text{lecture PV}$

$
\text{lecture} \rightarrow \text{READ IDENT}$

$G_2$
$\text{listInst} \rightarrow \text{inst listInst} | \text{inst}$
$inst \rightarrow \epsilon | \text{affect PV} | \text{lecture PV}$

$G_1$: on ne peut pas ajouter $\epsilon$ où on veut !!!! (non ambiguë)

$G_2$: on peut ajouter $\epsilon$ où on veut !!! (ambiguë)

Prouver qu'une grammaire est ambiguë: trouver un mot qui admet au moins 2 arbres syntaxiques.

Prouver qu'une grammaire n'est pas ambiguë: faire une preuve... Très difficile...

$\Rightarrow$ Décider de l'ambiguité d'une grammaire est indécidable !!!!

Il y a possibilité de transformer une grammaire ambiguë en grammaire non-ambiguë.

Grammaire à cupérateurs: grammaire faisant intervenir des cupérateurs avec associativité et priorité.

Pour rendre une grammaire à cupérateur non ambiguë:

- on ajoute un non terminal par niveau de priorité (S pour +, P pour * , etc)

- les moins prioritaires en haut de l'arbre, proches de l'axiome

- les plus prioritaires en bas de l'arbre, proches des feuilles

- les axiomes et terminaux tout en bas (les atomes)

- associativité gauche\droite $\Rightarrow$ récursivité gauche\droite

Grammaires pathologiques: certains non-terminaux ne servent à rien.
Cela est souvent dû à une erreur de conception.
les règles improductives ne sont pas détectées...

#### Définition

Une grammaire est dite réduite si elle ne contient pas de non-terminal improductif ou inaccessible.

#### Définition

Un non-terminal $X \in V_N$ est improductif s'il n'existe pas de mot $u \in V_T^* $ tel que $X \Rightarrow^* u$ (le langage engendré par $X$ est vide). Il est productif sinon.

#### Calcul des productifs: idée

X est productif:

- s'il existe une production $X \rightarrow u$ avec $u \in V_T^* $
- ou s'il existe une production $X \rightarrow \alpha$ avec $\alpha \in (V_N \cup V_T)^* $ tel que tous les non-terminaux apparaissant dans $\alpha$ sont productifs.

#### Définition d'un inaccessible

Soit $G$ une grammaire algébrique d'axiome $S$. Un non-terminal $X \in V_N$ est inaccessible s'il n'existe pas de mots $\alpha, \beta \in (V_N \cup V_T)^* $ tels que $S \Rightarrow^* \alpha X \beta$. Il est accessible sinon.

#### Calcul des accessibles: idée

$X$ est accessible si:

- c'est l'axiome
- ou il existe une production $Y \rightarrow \alpha X \beta$ telle que $Y$ est accessible.

Même principe d'itérations de point fixe que pour les accessibles mais on cherche les candidats en partie __droite__ de production

Si on souhaite calculer une grammaire réduite, il faut supprimer d'abord les improductifs, puis les inaccessibles.
