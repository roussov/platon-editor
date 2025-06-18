pub fn from_file(path: PathBuf) -> std::io::Result<Self> {
    let contents = fs::read_to_string(&path)?;
    let lines = contents.lines().map(String::from).collect::<Vec<String>>();
    Ok(Self {
        name: path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "untitled".to_string()),
        path: Some(path),
        lines: if lines.is_empty() {
            vec![String::new()]
        } else {
            lines
        },
        cursor: Cursor { x: 0, y: 0 },
        dirty: false,
    })
}

pub fn insert_char(&mut self, c: char) {
    if self.cursor.y >= self.lines.len() {
        self.lines.push(String::new());
    }
    let line = &mut self.lines[self.cursor.y];
    if self.cursor.x > line.len() {
        self.cursor.x = line.len();
    }
    line.insert(self.cursor.x, c);
    self.cursor.x += 1;
    self.dirty = true;
}

pub fn backspace(&mut self) {
    if self.cursor.y >= self.lines.len() {
        return;
    }

    if self.cursor.x > 0 {
        let line = &mut self.lines[self.cursor.y];
        line.remove(self.cursor.x - 1);
        self.cursor.x -= 1;
    } else if self.cursor.y > 0 {
        let prev_line = self.lines.remove(self.cursor.y);
        self.cursor.y -= 1;
        self.cursor.x = self.lines[self.cursor.y].len();
        self.lines[self.cursor.y].push_str(&prev_line);
    }

    self.dirty = true;
}

pub fn insert_newline(&mut self) {
    if self.cursor.y >= self.lines.len() {
        self.lines.push(String::new());
        self.cursor.y = self.lines.len() - 1;
        self.cursor.x = 0;
        return;
    }

    let current_line = &mut self.lines[self.cursor.y];
    let new_line = current_line.split_off(self.cursor.x);
    self.lines.insert(self.cursor.y + 1, new_line);
    self.cursor.y += 1;
    self.cursor.x = 0;
    self.dirty = true;
}

pub fn move_cursor_up(&mut self) {
    if self.cursor.y > 0 {
        self.cursor.y -= 1;
        let line_len = self.lines[self.cursor.y].len();
        self.cursor.x = self.cursor.x.min(line_len);
    }
}

pub fn move_cursor_down(&mut self) {
    if self.cursor.y + 1 < self.lines.len() {
        self.cursor.y += 1;
        let line_len = self.lines[self.cursor.y].len();
        self.cursor.x = self.cursor.x.min(line_len);
    }
}

pub fn move_cursor_left(&mut self) {
    if self.cursor.x > 0 {
        self.cursor.x -= 1;
    } else if self.cursor.y > 0 {
        self.cursor.y -= 1;
        self.cursor.x = self.lines[self.cursor.y].len();
    }
}

pub fn move_cursor_right(&mut self) {
    if self.cursor.y < self.lines.len() {
        let line_len = self.lines[self.cursor.y].len();
        if self.cursor.x < line_len {
            self.cursor.x += 1;
        } else if self.cursor.y + 1 < self.lines.len() {
            self.cursor.y += 1;
            self.cursor.x = 0;
        }
    }
}

pub fn get_line(&self, index: usize) -> Option<&String> {
    self.lines.get(index)
}

pub fn line_count(&self) -> usize {
    self.lines.len()
}

pub fn get_contents(&self) -> String {
    self.lines.join("\n")
}

pub fn save(&mut self) -> std::io::Result<()> {
    if let Some(ref path) = self.path {
        fs::write(path, self.get_contents())?;
        self.dirty = false;
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Aucun chemin de fichier associé.",
        ))
    }
}

pub fn set_path(&mut self, path: PathBuf) {
    self.path = Some(path);
}

pub fn mark_clean(&mut self) {
    self.dirty = false;
}
