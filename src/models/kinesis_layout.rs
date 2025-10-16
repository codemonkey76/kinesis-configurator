use std::io::Write;
use std::{fs, io, path::Path};

#[derive(Debug, Clone, PartialEq)]
pub enum KeyAction {
    /// simple key remapping: [source]>[target]
    SimpleRemap { source: String, target: String },

    /// Macro: {trigger}>{actions}
    Macro { trigger: String, actions: String },
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct KinesisLayout {
    pub mappings: Vec<KeyAction>,
}

impl KinesisLayout {
    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }

    /// Read layout from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        match fs::read_to_string(&path) {
            Ok(content) => content
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Self::new()),
            Err(e) => Err(e),
        }
    }

    /// Write layout to a file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = fs::File::create(path)?;
        write!(file, "{}", self)
    }

    /// Add a simple key remap
    pub fn add_remap(&mut self, source: String, target: String) {
        self.mappings
            .push(KeyAction::SimpleRemap { source, target });
    }

    /// Add a macro
    pub fn add_macro(&mut self, trigger: String, actions: String) {
        self.mappings.push(KeyAction::Macro { trigger, actions });
    }

    /// Find all remaps for a specific source key
    pub fn find_by_source(&self, source: &str) -> Vec<&KeyAction> {
        self.mappings
            .iter()
            .filter(|m| match m {
                KeyAction::SimpleRemap { source: s, .. } => s == source,
                KeyAction::Macro { trigger, .. } => trigger == source,
            })
            .collect()
    }

    /// Remove all mappings for a specific source key
    pub fn remove_by_source(&mut self, source: &str) {
        self.mappings.retain(|m| match m {
            KeyAction::SimpleRemap { source: s, .. } => s != source,
            KeyAction::Macro { trigger, .. } => trigger != source,
        });
    }
}

impl std::str::FromStr for KinesisLayout {
    type Err = String;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let mut layout = Self::new();

        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if !trimmed.contains('>') {
                return Err(format!("Line {}: Missing '>' operator", line_num + 1));
            }

            let parts: Vec<&str> = trimmed.splitn(2, '>').collect();
            if parts.len() != 2 {
                return Err(format!("Line {}: Invalid format", line_num + 1));
            }

            let source = parts[0].trim();
            let target = parts[1].trim();

            if source.starts_with('[') && source.ends_with(']') {
                // Simple remap
                if !target.starts_with('[') || !target.ends_with(']') {
                    return Err(format!(
                        "Line {}: Simple remap target must be in square brackets",
                        line_num + 1
                    ));
                }
                layout.mappings.push(KeyAction::SimpleRemap {
                    source: source[1..source.len() - 1].to_string(),
                    target: target[1..target.len() - 1].to_string(),
                });
            } else if source.starts_with('{') && source.ends_with('}') {
                // Macro: {key}>{actions}
                // Target should NOT be a simple remap (no square brackets)
                if target.starts_with('[') && target.ends_with(']') {
                    return Err(format!(
                        "Line {}: Macro target cannot be a simple remap in square brackets",
                        line_num + 1
                    ));
                }
                layout.mappings.push(KeyAction::Macro {
                    trigger: source[1..source.len() - 1].to_string(),
                    actions: target.to_string(),
                });
            } else {
                return Err(format!("Line {}: Invalid key notation", line_num + 1));
            }
        }
        Ok(layout)
    }
}

impl std::fmt::Display for KinesisLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for mapping in &self.mappings {
            match mapping {
                KeyAction::SimpleRemap { source, target } => {
                    writeln!(f, "[{}]>[{}]", source, target)?;
                }
                KeyAction::Macro { trigger, actions } => {
                    writeln!(f, "{{{}}}>{}", trigger, actions)?;
                }
            }
        }
        Ok(())
    }
}
