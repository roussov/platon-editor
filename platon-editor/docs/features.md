# 🚀 Fonctionnalités de Platon Editor

Platon est un éditeur de texte moderne et puissant, conçu pour supporter une large variété de langages de programmation avec une interface TUI (Terminal User Interface) intuitive. Voici un aperçu complet de ses fonctionnalités.

---

## 📝 Édition de texte avancée

- **Support multi-langages** : syntaxe, autocomplétion, snippets pour plus de 20 langages populaires.
- **Autocomplétion contextuelle** avec affichage d’aide intégrée.
- **Snippets** personnalisables et insertion rapide.
- **Undo / Redo** illimité.
- **Recherche et remplacement** puissants, avec support regex.
- **Commentaire / décommentaire** rapide de blocs ou lignes.
- **Formatage automatique** du code selon le langage.

---

## 🔍 Navigation et gestion de fichiers

- **Explorateur de fichiers intégré** avec arborescence et navigation.
- **Multi-onglets** pour éditer plusieurs fichiers simultanément.
- **Goto line / symbol** pour accéder rapidement au code.
- **Chargement et sauvegarde automatique**.

---

## 🧩 Support et intégration des langages

- Prise en charge de plus de 20 langages :
  - Rust, Python, JavaScript, TypeScript, C, C++, Java, Go, Ruby, Swift, Kotlin, PHP, Bash, Lua, Scala, Fortran, Basic, Assembleur, Brainfuck, Ook!, Markdown.
- **Interpréteurs intégrés** pour Brainfuck, Lua, Basic, Ook!, avec affichage TUI du pointeur et des cellules pour Brainfuck.
- **Exécution et compilation** intégrées avec retour console.
- **Optimisation automatique** du code source (pour certains langages).

---

## 🎨 Interface utilisateur

- **Interface TUI moderne et personnalisable** avec thèmes clairs et sombres.
- **Barre d’état dynamique** affichant infos sur fichier, ligne, mode, erreurs.
- **Raccourcis clavier personnalisables**.
- **Support du plein écran et redimensionnement dynamique**.
- **Affichage markdown** natif avec rendu amélioré via `pulldown-cmark`.

---

## 💡 Aide et documentation

- **Documentation intégrée** accessible via commandes ou raccourcis.
- **Aide interactive** lors de l’écriture avec descriptions des mots-clés.
- **Snippets documentés** pour une meilleure prise en main.

---

## ⚙️ Configuration et personnalisation

- Fichiers de configuration en TOML (`config.toml`, `keybindings.toml`, `themes/*.toml`).
- **Alias de commandes** personnalisables.
- Gestion des sessions utilisateur pour reprise rapide du travail.
- Support des plugins futurs (planifié).

---

## 🔧 Développement & Extension

- Code Rust propre, modulaire, facile à étendre.
- Architecture prévue pour intégrer de nouveaux langages.
- Système d’interprétation/compilation facile à enrichir.

---

## 📂 Fichiers importants du projet

- `/src/` : code source principal.
- `/src/lang/` : définitions des langages, autocomplétion, interpréteurs.
- `/docs/` : documentation utilisateur et développeur.
- `/themes/` : thèmes TUI.
- `/config/` : configuration utilisateur.

---

## 🏁 Conclusion

Platon vise à être un outil puissant et léger, adapté aux développeurs préférant travailler dans un terminal tout en bénéficiant d’un maximum d’aides modernes pour le code.

---

*Merci d’utiliser Platon ! Pour toute suggestion ou bug, contactez le mainteneur du projet.*

