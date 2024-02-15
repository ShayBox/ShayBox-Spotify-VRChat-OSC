use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

structstruck::strike! {
    #[strikethrough[derive(Debug, Serialize, Deserialize)]]
    pub struct Manifest {
        pub source: String,
        pub applications: Vec<pub struct {
            pub app_key: String,
            pub launch_type: String,
            pub binary_path_windows: String,
            pub is_dashboard_overlay: bool,
            pub strings: HashMap<String, pub struct {
                pub name: String,
                pub description: String,
            }>,
        }>,
    }
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            source:       "builtin".into(),
            applications: vec![Applications {
                app_key: "com.shaybox.vrc-osc".into(),
                launch_type: "binary".into(),
                binary_path_windows: "vrc-osc.exe".into(),
                is_dashboard_overlay: true,
                strings: HashMap::from([(
                    "en_us".into(),
                    Strings {
                        name:        "VRC-OSC".into(),
                        description: "VRChat OSC Overlay".into(),
                    },
                )]),
            }],
        }
    }
}

impl Manifest {
    pub fn get_path() -> Result<PathBuf> {
        let mut path = std::env::current_exe()?;
        path.set_file_name("vrc-osc");
        path.set_extension("vrmanifest");

        Ok(path)
    }

    pub fn load() -> Result<Self> {
        let path = Self::get_path()?;
        let file = File::options()
            .append(true)
            .create(true)
            .read(true)
            .open(path)?;

        let reader = BufReader::new(&file);
        if let Ok(config) = serde_json::from_reader(reader) {
            Ok(config)
        } else {
            let manifest = Self::default();
            let writer = BufWriter::new(&file);
            serde_json::to_writer_pretty(writer, &manifest)?;

            Ok(manifest)
        }
    }
}
