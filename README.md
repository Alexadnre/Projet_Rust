Projet RUST 2025 

Groupe : Alexandre VIDELAINE / Alexandre GROSDIDIER / Théo LYONNET

Thème : Ville générée procéduralement 

Crates : Bevy-engine
REdemander par mail : master de génération de ville 



site : https://bevyengine.org/

Partir des routes ( plan des routes ) et générer les batiments à partir des routes / bibliotèque de bruit
Quel type de grill ou floatant ? 
L'utilisateur doit devoir intéragir avant la génération


# Projet Rust - Ville Isométrique avec Bevy

## 1. Initialisation du Projet

### Installer Rust et Bevy
Assurez-vous d'avoir Rust installé sur votre machine et ajoutez Bevy à votre projet.

## 2. Création de la Grille Hexagonale

### Représentation des Hexagones
- Chaque hexagone sera divisé en 6 triangles.
- Les coordonnées des sommets et les relations entre les triangles devront être stockées.

### Génération de la Grille
- Une fonction sera définie pour générer une grille d'hexagones.
- La position de chaque hexagone devra être calculée en respectant une disposition en nid d'abeille.

## 3. Rendu 2D Isométrique

### Transformation des Coordonnées
- Il faudra convertir les coordonnées hexagonales en coordonnées isométriques.
- L'affichage devra être ajusté pour une perspective correcte.

### Affichage avec Bevy
- Un système devra être mis en place pour dessiner les hexagones et triangles.
- Des entités et des composants Bevy seront utilisés pour structurer la scène.

## 4. Génération Procédurale de Routes

### Utilisation du Bruit (Noise)
- Un algorithme de bruit sera utilisé pour générer une carte de bruit afin d'influencer la disposition des routes.
- Un algorithme sera appliqué pour relier certains triangles ou hexagones en chemins naturels.

### Placement des Routes
- Des critères devront être définis pour déterminer où une route traverse un triangle ou un hexagone.
- Le bruit devra être utilisé pour rendre l'aspect des routes plus naturel.

### Sliders pour les Routes
- Un slider sera ajouté pour ajuster l'intensité du bruit dans la génération des routes.
- Un autre slider pourra ajuster la fréquence des chemins générés (nombre de routes par zone).

## 5. Génération Procédurale de Bâtiments

### Placement Basé sur les Routes
- Les zones proches des routes devront être détectées pour y placer des bâtiments.
- Les hauteurs et types de bâtiments devront être ajustés en fonction du terrain.

### Rendu des Bâtiments
- Des sprites ou des primitives devront être définis pour représenter les bâtiments.
- Des variations devront être appliquées pour un effet plus réaliste.

### Sliders pour les Bâtiments
- Un slider permettra de contrôler la densité des bâtiments autour des routes.
- Un autre slider ajustera la taille des bâtiments (hauteur et largeur).

## 6. Améliorations Futures

- Ajouter des éléments de décor comme des arbres, des rivières, etc.
- Optimiser les performances pour pouvoir gérer de grandes cartes.
- Implémenter des interactions avec l'environnement telles que le zoom et le déplacement.

### Sliders pour les Décors
- Un slider permettra de définir la fréquence des arbres et autres éléments de décor.
- Un autre slider ajustera la taille des éléments décoratifs pour plus de diversité.

## 7. Exécution du Projet

Le projet sera lancé en utilisant la commande appropriée.

---

