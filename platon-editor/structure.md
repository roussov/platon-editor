```text
platon/
├── Cargo.toml                # Virtual manifest (workspace root)
├── README.md
├── .gitignore
├── config/
│   ├── default.toml          # Configuration par défaut
│   └── themes/
│       ├── dark.toml
│       └── light.toml
├── docs/
│   ├── commands.md           # Documentation utilisateur des commandes
│   ├── usage.md              # Guide d'utilisation
│   └── features.md
├── scripts/
│   ├── build.sh              # Script de build global
│   └── run.sh                # Script pour lancer l'éditeur
├── platon_editor/            # Crate principale
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── buffer.rs
│       ├── commands.rs
│       ├── config.rs
│       ├── editor.rs
│       ├── errors.rs
│       ├── file.rs
│       ├── help.rs
│       ├── input.rs
│       ├── markdown_render.rs
│       ├── optimize.rs
│       ├── session.rs
│       ├── style.rs
│       ├── terminal.rs
│       └── tui.rs
├── lang/                     # Crates par langage
│   ├── mod.rs                # Modèle commun d'autocomplétion/interpréteur
│   ├── rust/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── python/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── javascript/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── typescript/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── c/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── cpp/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── java/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── go/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── ruby/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── swift/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── kotlin/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── php/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── bash/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── lua/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── scala/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── fortran/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── basic/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── asm/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── brainfuck/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── ook/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── markdown/
│       ├── Cargo.toml
│       └── src/lib.rs
