# Rust2D

Rust2D est un moteur 2D expérimental écrit en Rust. Il utilise SDL2 pour la gestion
de la fenêtre et des événements, ainsi qu'OpenGL 3.3 Core pour le rendu.

Le projet affiche actuellement un triangle coloré tournant dans une fenêtre, avec
prise en charge du VSync et suivi des FPS dans la console.

## Fonctionnalités

- Fenêtre SDL2 redimensionnable
- Contexte OpenGL 3.3 Core
- Chargement et compilation de shaders GLSL
- Rendu d'un triangle avec couleurs par sommet
- Transformation 2D composée de translation, rotation et mise à l'échelle
- Activation et désactivation du VSync
- Affichage des FPS minimum et maximum dans la console

## Prérequis

- Rust récent, compatible avec l'édition 2024 de Cargo
- Un pilote graphique prenant en charge OpenGL 3.3
- Sur Windows, Visual Studio Build Tools avec les composants C++ peut être nécessaire
  lors de la compilation des dépendances natives

La dépendance SDL2 est regroupée avec `bundled`, aucune installation séparée de
SDL2 ne devrait donc être nécessaire.

## Installation

Cloner le dépôt puis lancer le projet depuis sa racine :

```bash
git clone https://github.com/FofeoLVR/Rust2D.git
cd Rust2D
cargo run
```

Pour compiler une version optimisée :

```bash
cargo build --release
```

L'exécutable sera alors disponible dans `target/release/`.

## Contrôles

| Touche | Action |
| --- | --- |
| `Échap` | Quitter l'application |
| `F` | Activer ou désactiver le VSync |

## Structure du projet

```text
src/
├── main.rs                      Point d'entrée et boucle principale
├── engine/
│   ├── engine.rs                Abstraction de moteur
│   └── render/
│       ├── renderer.rs          VBO, VAO et rendu OpenGL
│       ├── shader.rs            Chargement, compilation et liaison
│       └── shaders/
│           ├── basic.vert       Shader de sommet
│           └── basic.frag       Shader de fragment
└── math/
    ├── vec2.rs                  Vecteur 2D
    ├── mat3.rs                  Matrice 3 x 3
    └── transform.rs             Position, rotation et échelle
```
