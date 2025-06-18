# 📘 Platon Editor – Commandes et Raccourcis

Bienvenue dans la documentation des commandes de l'éditeur de texte Platon. Ce document référence toutes les commandes accessibles via les raccourcis clavier ou via la ligne de commande intégrée (`:`).

---

## ✨ Commandes Générales

| Commande           | Description                                 | Raccourci     |
|--------------------|---------------------------------------------|----------------|
| :open <fichier>    | Ouvre un fichier                            | Ctrl+O         |
| :save              | Sauvegarde le fichier courant               | Ctrl+S         |
| :quit              | Ferme l'éditeur                             | Ctrl+Q         |
| :write             | Alias de `:save`                            |                |
| :close             | Ferme le fichier actif                      |                |
| :reload            | Recharge le fichier à partir du disque     |                |
| :new <nom>         | Crée un nouveau fichier                     |                |

---

## 🧠 Édition

| Commande           | Description                                   | Raccourci         |
|--------------------|-----------------------------------------------|--------------------|
| :autocomplete      | Affiche les suggestions contextuelles         | Tab                |
| :format            | Formate le code selon le langage              | Ctrl+Shift+F       |
| :snippet <lang>    | Insère un snippet d’exemple                   |                    |
| :comment           | Commente/décommente la ligne                  | Ctrl+/             |
| :undo              | Annule la dernière modification               | Ctrl+Z             |
| :redo              | Rétablit une annulation                       | Ctrl+Shift+Z       |

---

## 📜 Navigation

| Commande           | Description                                 | Raccourci     |
|--------------------|---------------------------------------------|----------------|
| :goto <ligne>      | Aller à une ligne                           |                |
| :find <mot>        | Recherche une chaîne dans le fichier        | Ctrl+F         |
| :replace <a> <b>   | Remplace <a> par <b>                         | Ctrl+R         |
| :symbols           | Liste les symboles/fonctions                |                |

---

## 🧩 Aide et Documentation

| Commande           | Description                                       | Raccourci     |
|--------------------|---------------------------------------------------|----------------|
| :doc               | Affiche cette documentation                      | Ctrl+D         |
| :doc <mot>         | Affiche la doc pour un mot-clé/langage           |                |
| :about             | Affiche des infos sur l'éditeur                  |                |

---

## 🚀 Exécution & Interprétation

| Commande             | Description                                     | Raccourci     |
|----------------------|-------------------------------------------------|----------------|
| :run                 | Exécute/interprète le code courant              | F5             |
| :build               | Compile le fichier courant (si applicable)      | Ctrl+B         |
| :repl <lang>         | Lance un REPL interactif pour le langage donné |                |

---

## 🎨 Thèmes et Interface

| Commande           | Description                             | Raccourci       |
|--------------------|-----------------------------------------|------------------|
| :theme <nom>       | Change de thème                         | Ctrl+T           |
| :fullscreen        | Active/désactive le plein écran         | F11              |
| :status            | Affiche la barre d’état                 |                  |
| :resize <l> <h>    | Redimensionne la fenêtre terminal       |                  |

---

## 🧪 Langages supportés

Actuellement pris en charge :

rust, python, javascript, typescript, c, cpp, java, go, ruby, swift, kotlin, php, bash, lua, scala, fortran, basic, asm, brainfuck, ook, markdown

---

## 🔧 Avancé

| Commande               | Description                                     |
|------------------------|-------------------------------------------------|
| :config                | Ouvre le fichier de configuration               |
| :set <clé>=<valeur>    | Modifie une valeur de config temporairement     |
| :alias <nom>=<cmd>     | Crée un alias pour une commande personnalisée   |
| :optimize              | Optimise le code courant (si supporté)          |
| :theme reload          | Recharge les thèmes                            |

---

## 📁 Fichiers

- ~/.config/platon/config.toml → Configuration principale
- ~/.config/platon/keybindings.toml → Raccourcis clavier personnalisés
- ~/.config/platon/theme/ → Thèmes
- ./docs/commands.md → Cette documentation

---

Pour voir cette documentation dans Platon : utilisez la commande :doc ou appuyez sur Ctrl+D.

