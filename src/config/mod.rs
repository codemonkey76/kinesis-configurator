mod generator;
mod parser;

pub use generator::Generator;
pub use parser::Parser;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardConfig {
    pub layouts: [Layout; 9],
    pub active_layout: usize,
    pub macros: Vec<Macro>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub id: usize,
    pub name: String,
    pub remaps: HashMap<String, String>,
}
impl Layout {
    pub fn to_text(&self) -> anyhow::Result<String> {
        let mut output = String::new();

        for (from, to) in &self.remaps {
            output.push_str(&format!("{}={}\n", from, to));
        }

        Ok(output)
    }

    pub fn from_text(id: usize, name: String, content: &str) -> anyhow::Result<Self> {
        let mut remaps = HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((from, to)) = line.split_once('=') {
                remaps.insert(from.trim().to_string(), to.trim().to_string());
            }
        }

        Ok(Layout { id, name, remaps })
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    pub name: String,
    pub trigger: String,
    pub sequence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: u8,
    pub name: String,
    pub remaps: HashMap<String, String>,
}

impl KeyboardConfig {
    pub fn new() -> Self {
        // Initialize with 9 empty layouts
        let layouts = std::array::from_fn(|i| Layout {
            id: i,
            name: format!("Layout {}", i + 1),
            remaps: HashMap::new(),
        });

        Self {
            layouts,
            active_layout: 0,
            macros: Vec::new(),
        }
    }

    pub fn copy_layout(&mut self, from: usize, to: usize) -> Result<(), String> {
        if from >= 9 || to >= 9 {
            return Err("Invalid layout index".to_string());
        }
        self.layouts[to].remaps = self.layouts[from].remaps.clone();
        Ok(())
    }

    pub fn from_text(content: &str) -> anyhow::Result<Self> {
        Parser::parse(content)
    }

    pub fn to_text(&self) -> anyhow::Result<String> {
        Generator::generate(self)
    }
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        Self::new()
    }
}
