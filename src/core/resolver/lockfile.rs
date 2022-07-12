use std::{str::FromStr, path::{PathBuf}};
use toml_edit::Document;
use crate::{util::{sha256::Sha256Hash, anyerror::{AnyError, Fault}}, core::{pkgid::PkgId, version::Version, config::FromToml, manifest::IpManifest}};

#[derive(Debug)]
pub struct LockFile(Vec<LockEntry>);

impl FromToml for LockFile {
    type Err = Fault;
    
    fn from_toml(table: &toml_edit::Table) -> Result<Self, Self::Err> where Self: Sized {
        let mut inner = Vec::new();
        // take array as as tables
        if let Some(item) = table.get("ip") {
            match item.as_array_of_tables() {
                // parse each table entry into a `LockEntry` struct
                Some(arr) => {
                    for tbl in arr {
                        inner.push(LockEntry::from_toml(tbl)?);
                    }
                }
                None => {
                    return Err(AnyError(format!("expects 'ip' to be an array of tables")))?
                }
            }
        }
        Ok(Self(inner))
    }
}

impl LockFile {
    /// Loads a lockfile from the `root` path.
    /// 
    /// If the file does not exist, then an empty lock entry list is returned.
    pub fn from_path(root: &PathBuf) -> Result<Self, Fault> {
        let lock_file = root.join(IP_LOCK_FILE);
        if lock_file.exists() == true {
            // open file
            let contents = std::fs::read_to_string(&lock_file)?;
            // parse toml syntax
            Ok(Self::from_toml(contents.parse::<Document>()?.as_table())?)
        } else {
            Ok(Self(Vec::new()))
        }
    }

    pub fn get(&self, target: &PkgId, version: &Version) -> Option<&LockEntry> {
        self.0.iter().find(|&f| &f.name == target && &f.version == version )
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, PartialEq)]
struct Source(String);

#[derive(Debug, PartialEq)]
pub struct LockEntry {
    name: PkgId,
    version: Version,
    sum: Option<Sha256Hash>,
    source: Source,
}

impl From<&IpManifest> for LockEntry {
    fn from(ip: &IpManifest) -> Self {
        Self {
            name: ip.get_pkgid().clone(), 
            version: ip.get_version().clone(), 
            sum: Some(ip.get_checksum_proof(0).unwrap_or(ip.compute_checksum())), 
            source: Source(ip.get_repository().unwrap_or(&String::new()).to_string()),
        }
    }
}

impl LockEntry {
    pub fn get_sum(&self) -> Option<&Sha256Hash> {
        self.sum.as_ref()
    }

    pub fn to_toml(&self, table: &mut toml_edit::Table) -> () {
        table["name"] = toml_edit::value(&self.name.to_string());
        table["version"] = toml_edit::value(&self.version.to_string());
        if let Some(sum) = self.get_sum() {
            table["sum"] = toml_edit::value(&sum.to_string());
        }
        table["source"] = toml_edit::value(&self.source.0);
    }
}

impl FromToml for LockEntry {
    type Err = Fault; 

    fn from_toml(table: &toml_edit::Table) -> Result<Self, Self::Err> where Self: Sized {
        Ok(Self {
            name: PkgId::from_str(table.get("name").unwrap().as_str().unwrap())?,
            version: Version::from_str(table.get("version").unwrap().as_str().unwrap())?,
            sum: match table.get("sum") {
                Some(item) => Some(Sha256Hash::from_str(item.as_str().unwrap())?),
                None => None,
            },
            source: Source(table.get("source").unwrap().as_str().unwrap().to_owned()),
        })
    }
}

#[cfg(test)]
mod test {
    use toml_edit::Document;
    use super::*;

    #[test]
    fn from_toml() {
        let toml = r#"
# This file is automatically generated by Orbit.
# It is not intended for manual editing.

[[ip]]
name = "ks-tech.rary.gates"
version = "0.1.0"
sum = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
source = "git.url1"

[[ip]]
name = "ks-tech.util.toolbox"
version = "1.2.3"
sum = "f3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
source = "git.url2"

"#;
        let lock = LockFile::from_toml(toml.parse::<Document>().unwrap().as_table()).unwrap();
        assert_eq!(lock.len(), 2);
    }

    #[test]
    fn check_lock() {
        let ip = IpManifest::from_path(&PathBuf::from("./test/data/projects/project-a/")).unwrap();
        assert_eq!(ip.is_locked(), true);
    }
}

pub const IP_LOCK_FILE: &str = "Orbit.lock";