use std::fs::{self, File, OpenOptions, read_dir};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct PlatonFile {
    pub path: PathBuf,
    pub content: String,
}

impl PlatonFile {
    /// Ouvre un fichier et lit son contenu.
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        let mut file = File::open(&path_buf)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(Self {
            path: path_buf,
            content,
        })
    }

    /// Crée un nouveau fichier vide ou existant (sans écraser).
    pub fn create<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        File::create(&path_buf)?; // Crée le fichier vide
        Ok(Self {
            path: path_buf,
            content: String::new(),
        })
    }

    /// Enregistre le contenu courant dans le fichier.
    pub fn save(&self) -> io::Result<()> {
        let mut file = File::create(&self.path)?;
        file.write_all(self.content.as_bytes())
    }

    /// Remplace le contenu du fichier et sauvegarde.
    pub fn write_content(&mut self, new_content: &str) -> io::Result<()> {
        self.content = new_content.to_string();
        self.save()
    }

    /// Supprime le fichier associé.
    pub fn delete(&self) -> io::Result<()> {
        fs::remove_file(&self.path)
    }

    /// Renomme le fichier.
    pub fn rename<P: AsRef<Path>>(&mut self, new_path: P) -> io::Result<()> {
        fs::rename(&self.path, &new_path)?;
        self.path = new_path.as_ref().to_path_buf();
        Ok(())
    }

    /// Vérifie si un fichier existe.
    pub fn exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists()
    }
}

/// Liste les fichiers dans un dossier donné.
pub fn list_files_in_dir<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in read_dir(dir)? {
        let entry = entry?;
        files.push(entry.path());
    }
    Ok(files)
}

/// Lit un fichier texte brut et retourne son contenu.
pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Écrit une chaîne dans un fichier (en écrasant).
pub fn write_file<P: AsRef<Path>>(path: P, data: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(path)?;
    file.write_all(data.as_bytes())
}
