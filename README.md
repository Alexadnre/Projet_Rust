# 🏙️ Projet RUST 2025 – Ville Générée Procéduralement

**Groupe** : Alexandre VIDELAINE / Alexandre GROSDIDIER / Théo LYONNET  
**Technologie** : [Bevy Engine](https://bevyengine.org/) (Rust, ECS, Rendering, UI)  
**Thème** : Simulation d’une ville 3D générée à partir de bruit procédural.

---

## 📚 Sommaire

- [1. Objectif du projet](#1-objectif-du-projet)
- [2. Présentation technique](#2-présentation-technique)
- [3. Fonctionnalités interactives](#3-fonctionnalités-interactives)
- [4. Architecture du code](#4-architecture-du-code)
- [5. Gestion de la résolution (Upscaling)](#5-gestion-de-la-résolution-upscaling)
- [6. Problèmes rencontrés](#6-problèmes-rencontrés)
- [7. Améliorations possibles](#7-améliorations-possibles)
- [8. Captures d’écran](#8-captures-décran)
- [9. Lancer le projet](#9-lancer-le-projet)

---

## 1. 🎯 Objectif du projet

Créer une **ville 3D procédurale** avec génération dynamique de routes, bâtiments et arbres, en combinant **bruit de Perlin** et logique ECS (Entity Component System) via le moteur **Bevy**.

---

## 2. 🧠 Présentation technique

- 📦 Langage : **Rust**
- 🧱 Moteur : **Bevy (0.13)**
- 🌐 Génération procédurale : **Noise (Perlin) + Hash-seeding**
- 📐 Interface : **EGUI** intégrée à Bevy
- 🎨 Matériaux : `StandardMaterial` avec textures pixelisées dynamiques

---

## 3. 🕹️ Fonctionnalités interactives

| Élément           | Description |
|------------------|-------------|
| 🎚️ Slider `chaos` | Génère plus ou moins de variabilité dans la ville |
| 🔍 Zoom           | Contrôle la distance de la caméra |
| 🧊 Résolution     | Diminue la résolution de rendu sans changer la taille de la fenêtre |
| ⬅️➡️ Caméra       | Navigation avec les flèches directionnelles |
| 🗑️ Reset ville    | Efface les entités de la scène |

---

## 4. 📂 Architecture du code

```
/src
├── main.rs              # Point d’entrée
├── camera.rs            # Contrôle et zoom caméra
├── city.rs              # Génération des bâtiments et routes
├── chunk.rs             # Gestion dynamique de chunks visibles
├── components.rs        # Tous les composants ECS (tags, ressources)
├── lighting.rs          # Setup de l’éclairage
├── trees.rs             # Ajout d’arbres
├── ui.rs                # Interface utilisateur EGUI
└── graphics/
    ├── mod.rs
    ├── pixel_target.rs  # Création du render target basse résolution
    └── pixel_view.rs    # Projection sur écran via un sprite plein écran
```

---

## 5. 🧩 Gestion de la résolution (Upscaling / Pixelisation)

- Le rendu 3D est fait sur une **texture offscreen** plus petite.
- Cette texture est ensuite **étirée (upscaled)** à la taille de la fenêtre avec un `SpriteBundle`.
- Cela simule un **mode pixel art** sans dégrader les performances.

```rust
// Résolution dynamique
let lowres_image = Image::new_fill(...);
camera.viewport = Some(Viewport {
    physical_size: UVec2::new(w, h),
    ..
});
```

---

## 6. 🐛 Problèmes rencontrés

| Problème | Résolution |
|---------|------------|
| `Camera3d` en double | Tag `Main3dCamera` pour ne jamais confondre les caméras |
| `Viewport` mal dimensionné | Ajout d’un système de clamp pour éviter un viewport vide |
| Panique `unwrap` sur chunks | Remplacé par `get_single_mut().ok()?` sécurisés |
| `ImageSampler` obsolète | Remplacé par `image.sampler = ImageSampler::nearest()` |
| Texture trop floue | Génération de textures `1x1` manuellement via RGBA |

---

## 7. 🚀 Améliorations possibles

- 🌳 Ajouter du LOD (Level of Detail) pour les arbres
- 🏗️ Créer une vraie bibliothèque de bâtiments
- 📷 Intégrer une option de capture automatique (screenshot)
- 📏 Activer une **minimap** ou une vue top-down
- 🌍 Génération par seed avec input de l’utilisateur

---

## 8. 🖼️ Captures d’écran

Place tes images dans `/assets/screenshots/` et insère-les ici :

| Vue Générale | Zoom Proche | Mode Pixelisé |
|--------------|-------------|----------------|
| ![1](assets/screenshots/vue_generale.png) | ![2](assets/screenshots/zoom.png) | ![3](assets/screenshots/pixel.png) |

---

## 9. ▶️ Lancer le projet

```bash
# Étapes à suivre :
cargo build
cargo run
```

### 📦 Dépendances principales

```toml
[dependencies]
bevy = "0.13"
bevy_egui = "0.24"
noise = "0.8"
rand = "0.8"
```

---

