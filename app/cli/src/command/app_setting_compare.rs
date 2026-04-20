use regex::Regex;
use serde_yaml::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_FILE_REGEX: &str = r"appsettings[.]?[A-z]*\.yml";

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(
        long,
        default_value = DEFAULT_FILE_REGEX,
        help = "Regex pattern used to select files while walking directories"
    )]
    regex: String,
}

impl Args {
    pub async fn run(&self) -> anyhow::Result<()> {
        let root = std::env::current_dir()?;
        let file_regex = Regex::new(&self.regex)
            .map_err(|e| anyhow::anyhow!("invalid regex '{}': {e}", self.regex))?;
        let files = find_files(&root, &file_regex, 2)?;

        if files.is_empty() {
            anyhow::bail!("no files matching /{}/ found under {:?}", self.regex, root);
        }

        let mut per_file: BTreeMap<PathBuf, BTreeSet<String>> = BTreeMap::new();
        let mut files_by_dir: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
        for p in &files {
            let content = fs::read_to_string(p)?;
            let yaml_input = content.strip_prefix('\u{FEFF}').unwrap_or(&content);
            let yaml: Value = serde_yaml::from_str(yaml_input)
                .map_err(|e| anyhow::anyhow!("yaml parse error in {:?}: {e}", p))?;

            let mut keys = BTreeSet::<String>::new();
            collect_key_paths(&yaml, "", &mut keys);
            per_file.insert(p.clone(), keys);

            let dir = p
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_else(|| root.clone());
            files_by_dir.entry(dir).or_default().push(p.clone());
        }

        let mut all_diffs: Vec<(PathBuf, String, Vec<PathBuf>, Vec<PathBuf>)> = Vec::new();
        for (dir, dir_files) in &files_by_dir {
            let mut union = BTreeSet::<String>::new();
            for p in dir_files {
                if let Some(keys) = per_file.get(p) {
                    union.extend(keys.iter().cloned());
                }
            }

            for k in &union {
                let mut present = Vec::<PathBuf>::new();
                let mut missing = Vec::<PathBuf>::new();
                for p in dir_files {
                    if per_file.get(p).map(|s| s.contains(k)).unwrap_or(false) {
                        present.push(p.clone());
                    } else {
                        missing.push(p.clone());
                    }
                }
                if !missing.is_empty() && !present.is_empty() {
                    all_diffs.push((dir.clone(), k.clone(), present, missing));
                }
            }
        }

        if all_diffs.is_empty() {
            println!(
                "OK: all matching files have consistent keys within each directory ({} file(s), {} directories).",
                files.len(),
                files_by_dir.len()
            );
            for p in &files {
                println!("- {:?}", p);
            }
            return Ok(());
        }

        println!(
            "DIFF: found {} key mismatch(es) across {} file(s) in {} directories.",
            all_diffs.len(),
            files.len(),
            files_by_dir.len()
        );
        for (dir, key, present, missing) in all_diffs {
            println!();
            println!("directory: {:?}", dir);
            println!("key: {}", key);
            println!("  present in:");
            for p in present {
                println!("  - {:?}", p);
            }
            println!("  missing in:");
            for p in missing {
                println!("  - {:?}", p);
            }
        }

        anyhow::bail!("appsettings key mismatch found");
    }
}

fn find_files(root: &Path, regex: &Regex, depth_ttl: usize) -> anyhow::Result<Vec<PathBuf>> {
    fn walk(
        dir: &Path,
        out: &mut Vec<PathBuf>,
        regex: &Regex,
        depth_ttl: usize,
    ) -> anyhow::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let ft = entry.file_type()?;
            if ft.is_dir() {
                if depth_ttl > 0 {
                    walk(&path, out, regex, depth_ttl - 1)?;
                }
                continue;
            }
            if !ft.is_file() {
                continue;
            }
            let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            if regex.is_match(name) {
                out.push(path);
            }
        }
        Ok(())
    }

    let mut out = Vec::<PathBuf>::new();
    walk(root, &mut out, regex, depth_ttl)?;
    out.sort();
    Ok(out)
}

fn collect_key_paths(v: &Value, prefix: &str, out: &mut BTreeSet<String>) {
    match v {
        Value::Mapping(map) => {
            for (k, child) in map {
                let key = yaml_key_to_string(k);
                let path = if prefix.is_empty() {
                    key
                } else {
                    format!("{}.{}", prefix, key)
                };
                out.insert(path.clone());
                collect_key_paths(child, &path, out);
            }
        }
        Value::Sequence(seq) => {
            for (i, child) in seq.iter().enumerate() {
                let path = if prefix == "dbconfig" {
                    if let Some(id) = mapping_get_field_as_string(child, "id") {
                        format!("{prefix}[id={id}]")
                    } else if prefix.is_empty() {
                        format!("[{}]", i)
                    } else {
                        format!("{}[{}]", prefix, i)
                    }
                } else if prefix.is_empty() {
                    format!("[{}]", i)
                } else {
                    format!("{}[{}]", prefix, i)
                };
                out.insert(path.clone());
                collect_key_paths(child, &path, out);
            }
        }
        _ => {}
    }
}

fn mapping_get_field_as_string(v: &Value, field: &str) -> Option<String> {
    let Value::Mapping(map) = v else {
        return None;
    };

    let key = Value::String(field.to_string());
    let value = map.get(&key)?;
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Null => Some("null".to_string()),
        _ => None,
    }
}

fn yaml_key_to_string(k: &Value) -> String {
    match k {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        // YAML allows complex keys; fall back to debug string.
        _ => format!("{k:?}"),
    }
}
