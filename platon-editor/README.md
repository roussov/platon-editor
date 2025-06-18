# 📘 Commandes de l'Éditeur Platon

Platon est un éditeur de texte programmable en Rust prenant en charge de nombreux langages. Voici la documentation exhaustive des commandes disponibles en session interactive.

---

## 🔰 Commandes Globales

| Commande               | Alias(s)      | Description                                               |
|------------------------|---------------|-----------------------------------------------------------|
| `:quit`                | `:q`          | Quitter l'éditeur                                         |
| `:write`               | `:w`          | Sauvegarder le fichier en cours                           |
| `:writeas <fichier>`   | `:wa`         | Sauvegarder sous un autre nom                             |
| `:open <fichier>`      | `:o`          | Ouvrir un fichier dans l'éditeur                          |
| `:new`                 | `:n`          | Créer un nouveau fichier                                  |
| `:reload`              |               | Recharger le fichier depuis le disque                     |
| `:clear`               |               | Effacer le terminal                                       |

---

## 🧠 Navigation

| Commande               | Description                                 |
|------------------------|---------------------------------------------|
| `:goto <ligne>`        | Aller à une ligne spécifique                |
| `:top`                 | Aller au début du fichier                   |
| `:bottom`              | Aller à la fin du fichier                   |
| `:scroll <+n/-n>`      | Faire défiler de n lignes                   |
| `:center`              | Centrer la vue sur la ligne actuelle        |

---

## 🛠️ Edition

| Commande               | Description                                             |
|------------------------|---------------------------------------------------------|
| `:insert`              | Passer en mode insertion                                |
| `:delete <ligne>`      | Supprimer une ligne spécifique                          |
| `:replace <l1>-<l2>`   | Remplacer les lignes entre l1 et l2                     |
| `:cut <ligne>`         | Couper une ligne                                        |
| `:copy <ligne>`        | Copier une ligne                                        |
| `:paste`               | Coller la dernière ligne copiée                         |
| `:undo`                | Annuler la dernière modification                        |
| `:redo`                | Rétablir la modification annulée                        |

---

## 🔍 Recherche et remplacement

| Commande                      | Description                                          |
|-------------------------------|------------------------------------------------------|
| `:find <mot>`                 | Rechercher un mot ou une expression                 |
| `:replaceall <mot> <remp>`    | Remplacer toutes les occurrences                    |
| `:highlight <mot>`            | Mettre en surbrillance un mot                      |
| `:regex <expr>`               | Rechercher avec une expression régulière           |

---

## ✨ Langages et complétion

| Commande                    | Description                                                     |
|-----------------------------|-----------------------------------------------------------------|
| `:lang <langage>`           | Définir le langage de syntaxe                                  |
| `:autocomplete`             | Suggère des complétions à la position du curseur              |
| `:doc <mot>`                | Affiche la documentation du mot-clé ou symbole                |
| `:snippet <lang>`           | Insère un snippet de démarrage pour le langage                |
| `:run`                      | Exécute le code si un interpréteur est disponible             |
| `:interpret <lang>`         | Interprète le code dans la langue indiquée                    |

---

## 🎨 Apparence et thème

| Commande              | Description                                      |
|-----------------------|--------------------------------------------------|
| `:theme <nom>`        | Appliquer un thème (dark, light, solarized, etc)|
| `:fontsize <taille>`  | Modifier la taille des caractères                |
| `:style debug`        | Activer le mode style debug                      |
| `:fullscreen`         | Basculer en mode plein écran                     |

---

## ⚙️ Système & configuration

| Commande                | Description                                          |
|-------------------------|------------------------------------------------------|
| `:config`               | Ouvrir le fichier de configuration utilisateur       |
| `:alias <nom> <cmd>`    | Définir un alias personnel pour une commande         |
| `:reset`                | Réinitialiser tous les paramètres                    |
| `:session save`         | Sauvegarder la session actuelle                      |
| `:session load`         | Recharger une session précédente                     |
| `:platon`               | Affiche le logo ASCII de Platon                      |

---

## 🧪 Développement & Interpréteurs

| Commande                  | Langages compatibles         | Description                                      |
|---------------------------|------------------------------|--------------------------------------------------|
| `:interpret`              | brainfuck, ruby, basic...    | Interprète le code courant                      |
| `:visualize`              | brainfuck                    | Affiche les cellules et le pointeur en TUI      |
| `:compile`                | rust, c, cpp...              | Compile si un compilateur est disponible        |

---

## 🧬 Langages supportés

Actuellement pris en charge :

- Rust, Python, JavaScript, TypeScript, C, C++, Java, Go
- Ruby, Swift, Kotlin, PHP, Bash, Lua, Scala, Fortran
- BASIC, Assembler, Markdown, Brainfuck, Ook!

---

## 📎 Raccourcis Clavier

| Touche                    | Action                               |
|---------------------------|---------------------------------------|
| Ctrl+S                   | Sauvegarder                          |
| Ctrl+Q                   | Quitter                              |
| Ctrl+F                   | Rechercher                           |
| Ctrl+E                   | Mode insertion                       |
| Ctrl+L                   | Nettoyer l’écran                     |
| Alt+← / Alt+→            | Navigation rapide entre onglets      |

---

ℹ️ Pour plus de détails ou pour personnaliser vos commandes :
- Modifier le fichier ~/.config/platon/config.toml
- Voir aussi : `:help` dans l’éditeur

