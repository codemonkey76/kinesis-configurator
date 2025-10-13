use crate::config::KeyboardConfig;
use anyhow::{Context, Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};

pub struct VDrive {
    mount_path: Option<PathBuf>,
}

impl VDrive {
    pub fn new() -> Self {
        Self { mount_path: None }
    }

    pub fn detect(&mut self) -> Result<String> {
        // Common mount points to check on Linux
        let possible_paths = vec!["/media", "/mnt", "/run/media"];

        // Look for a mounted device that looks like the Kinesis V-Drive
        for base in possible_paths {
            if let Ok(entries) = fs::read_dir(base) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        // Check if this looks like the Kinesis V-Drive
                        // Look for the layouts directory
                        let layouts_dir = path.join("layouts");
                        if layouts_dir.exists() && layouts_dir.is_dir() {
                            self.mount_path = Some(path.clone());
                            return Ok(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        // Also check under current user's runtime dir
        if let Ok(user) = std::env::var("USER") {
            let runtime_path = format!("/run/media/{}", user);
            if let Ok(entries) = fs::read_dir(&runtime_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let layouts_dir = path.join("layouts");
                    if layouts_dir.exists() && layouts_dir.is_dir() {
                        self.mount_path = Some(path.clone());
                        return Ok(path.to_string_lossy().to_string());
                    }
                }
            }
        }

        Err(anyhow!(
            "Kinesis V-Drive not found. Make sure your keyboard is in drive mode (SmartSet + Hk3)."
        ))
    }

    pub fn load_config(&self) -> Result<KeyboardConfig> {
        let mount = self
            .mount_path
            .as_ref()
            .ok_or_else(|| anyhow!("V-Drive not detected. Call detect() first."))?;

        let layouts_dir = mount.join("layouts");

        let config = KeyboardConfig::new();

        // Load each layout file
        for i in 0..9 {
            let layout_path = layouts_dir.join(format!("layout{}.txt", i + 1));
            if layout_path.exists() {
                let content = fs::read_to_string(&layout_path)
                    .context(format!("Failed to read layout{}.txt", i + 1))?;

                // Parse the layout file and update the config
                // For now, we'll just store the raw content
                // TODO: Implement proper parsing
                println!("Layout {}: {}", i + 1, content);
            }
        }

        Ok(config)
    }

    pub fn save_config(&self, config: &KeyboardConfig) -> Result<()> {
        let mount = self
            .mount_path
            .as_ref()
            .ok_or_else(|| anyhow!("V-Drive not detected. Call detect() first."))?;

        let layouts_dir = mount.join("layouts");

        // Save each layout to its own file
        for i in 0..9 {
            let layout_path = layouts_dir.join(format!("layout{}.txt", i + 1));
            let content = config.layouts[i].to_text()?;

            fs::write(&layout_path, content)
                .context(format!("Failed to write layout{}.txt", i + 1))?;
        }

        Ok(())
    }

    pub fn get_mount_path(&self) -> Option<&Path> {
        self.mount_path.as_deref()
    }
}

impl Default for VDrive {
    fn default() -> Self {
        Self::new()
    }
}
