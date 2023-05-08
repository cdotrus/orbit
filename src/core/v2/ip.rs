use std::path::PathBuf;
use crate::core::v2::manifest::Manifest;
use crate::util::anyerror::AnyError;
use crate::util::anyerror::Fault;
use crate::core::v2::manifest;

use super::lockfile::IP_LOCK_FILE;
use super::lockfile::LockFile;
use super::manifest::FromFile;
use crate::core::v2::manifest::ORBIT_METADATA_FILE;
use crate::core::v2::manifest::IP_MANIFEST_FILE;
use crate::core::v2::manifest::ORBIT_SUM_FILE;
use toml_edit::Document;
use crate::util::sha256::Sha256Hash;
use crate::core::lang::vhdl::primaryunit::PrimaryUnit;
use crate::core::lang::vhdl::token::Identifier;
use std::str::FromStr;
use std::collections::HashMap;
use std::error::Error;
use crate::core::v2::lockfile::LockEntry;

#[derive(Debug, PartialEq)]
pub struct Ip {
    /// The base directory for the entire [Ip] structure.
    root: PathBuf,
    /// The metadata for the [Ip].
    data: Manifest,
    /// The lockfile for the [Ip].
    lock: LockFile,
}

impl Ip {

    pub fn get_root(&self) -> &PathBuf {
        &self.root
    }

    pub fn get_man(&self) -> &Manifest {
        &self.data
    }

    pub fn get_lock(&self) -> &LockFile {
        &self.lock
    }

    pub fn load(root: PathBuf) -> Result<Self, Box<dyn Error>> {
        let man_path = root.join(IP_MANIFEST_FILE);
        if man_path.exists() == false || man_path.is_file() == false {
            return Err(AnyError(format!("A manifest file does not exist")))?
        }
        let lock_path = root.join(IP_LOCK_FILE);
        Ok(Self {
            root: root,
            data: Manifest::from_file(&man_path)?,
            lock: LockFile::from_file(&lock_path)?,
        })
    }

    /// Checks if the given path hosts a valid manifest file.
    pub fn is_valid(path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let man_path = path.join(IP_MANIFEST_FILE);
        if man_path.exists() == false || man_path.is_file() == false {
            return Err(AnyError(format!("A manifest file does not exist")))?
        }
        // attempt to load the manifest file
        let _ = Manifest::from_file(&man_path)?;
        return Ok(())
    }

    /// Finds all Manifest files available in the provided path `path`.
    /// 
    /// Errors if on filesystem problems.
    fn detect_all_sub(path: &PathBuf, name: &str, is_exclusive: bool) -> Result<Vec<Self>, Fault> {
        let mut result = Vec::new();
        // walk the ORBIT_PATH directory @TODO recursively walk inner directories until hitting first 'Orbit.toml' file
        for mut entry in manifest::find_file(&path, &name, is_exclusive)? {
            // read ip_spec from each manifest
            let man = Manifest::from_file(&entry)?;
            entry.pop();
            result.push( Self {
                root: entry, 
                data: man,
                lock: LockFile::new(),
            });
        }
        Ok(result)
    }

    /// Finds all IP manifest files along the provided path `path`.
    /// 
    /// Wraps Manifest::detect_all.
    pub fn detect_all(path: &PathBuf) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        Self::detect_all_sub(path, IP_MANIFEST_FILE, true)
    }

    /// Checks the metadata file for a entry for `dynamic`.
    pub fn is_dynamic(&self) -> bool {
        self.get_root().join(".orbit-dynamic").exists() == true
    }

    pub fn generate_dst_lut(&self) -> HashMap<Identifier, String> {
        // @todo: read units from metadata to speed up results
        let units = Self::collect_units(true, self.get_root()).unwrap();
        let checksum = Ip::read_checksum_proof(self.get_root()).unwrap();
        // compose the lut for symbol transformation
        let mut lut = HashMap::new();
        units.into_iter().for_each(|(key, _)| {
            lut.insert(
                key.clone(), 
                "_".to_string() + checksum.to_string().get(0..10).unwrap()
            );
        });
        lut
    }

    pub fn set_as_dynamic(&self) -> () {
        let _ = std::fs::write(self.get_root().join(".orbit-dynamic"), "").unwrap();
    }

    /// Checks if needing to read off the lock file.
    /// 
    /// This determines if the lock file's data matches the Orbit.toml manifest data,
    /// indicating it is safe to pull data from the lock file and no changes would be
    /// made to the lock file.
    pub fn can_use_lock(&self) -> bool {
        let target = self.get_lock().get(self.get_man().get_ip().get_name(), self.get_man().get_ip().get_version());
        match target {
            Some(entry) => entry.matches_target(&LockEntry::from(self)),
            None => false,
        }
    }

    /// Computes the checksum on the root of the IP.
    /// 
    /// Changes the current working directory to the root for consistent computation.
    pub fn compute_checksum(dir: &PathBuf) -> Sha256Hash {
        let ip_files = crate::util::filesystem::gather_current_files(&dir, true);
        let checksum = crate::util::checksum::checksum(&ip_files, &dir);
        checksum
    }

    /// Gets the already calculated checksum from an installed IP from [ORBIT_SUM_FILE].
    /// 
    /// Returns `None` if the file does not exist, is unable to read into a string, or
    /// if the sha cannot be parsed.
    pub fn read_checksum_proof(dir: &PathBuf) -> Option<Sha256Hash> {
        let sum_file = dir.join(ORBIT_SUM_FILE);
        if sum_file.exists() == false {
            None
        } else {
            match std::fs::read_to_string(&sum_file) {
                Ok(text) => {
                    match Sha256Hash::from_str(&text.trim()) {
                        Ok(sha) => Some(sha),
                        Err(_) => None,
                    }
                }
                Err(_) => None,
            }
        }
    }

    /// Caches the result of collecting all the primary design units for the given package.
    /// 
    /// Writes the data to the toml data structure. Note, this function does not save the manifest data to file.
    // pub fn stash_units(&mut self) -> () {
    //     // collect the units
    //     let units = Self::collect_units(true).unwrap();
    //     let tbl = self.get_manifest_mut().get_mut_doc()["ip"].as_table_mut().unwrap();
    //     tbl.insert("units", toml_edit::Item::Value(toml_edit::Value::Array(Array::new())));
    //     let arr = tbl["units"].as_array_mut().unwrap();
    //     // map the units into a serialized data format
    //     for (_, unit) in &units {
    //         arr.push(unit.to_toml());
    //     }
    //     tbl["units"].as_array_mut().unwrap().iter_mut().for_each(|f| {
    //         f.decor_mut().set_prefix("\n    ");
    //         f.decor_mut().set_suffix("");
    //     });
    //     tbl["units"].as_array_mut().unwrap().set_trailing("\n");
    // }

    /// Gathers the list of primary design units for the current ip.
    /// 
    /// If the manifest has an toml entry for `units` and `force` is set to `false`, 
    /// then it will return that list rather than go through files.
    pub fn collect_units(force: bool, dir: &PathBuf) -> Result<HashMap<Identifier, PrimaryUnit>, Fault> {
        // try to read from metadata file
        match (force == false) && Self::read_units_from_metadata(&dir).is_some() {
            // use precomputed result
            true => Ok(Self::read_units_from_metadata(&dir).unwrap()),
            false => {
                // collect all files
                let files = crate::util::filesystem::gather_current_files(&dir, false);
                Ok(crate::core::lang::vhdl::primaryunit::collect_units(&files)?)
            }
        }
    }

    pub fn read_units_from_metadata(dir: &PathBuf) -> Option<HashMap<Identifier, PrimaryUnit>> {
        let meta_file = dir.join(ORBIT_METADATA_FILE);
        if std::path::Path::exists(&meta_file) == true {
            if let Ok(contents) = std::fs::read_to_string(&meta_file) {
                if let Ok(toml) = contents.parse::<Document>() {
                    let entry = toml.get("ip")?.as_table()?.get("units")?.as_array()?;
                    let mut map = HashMap::new();
                    for unit in entry {
                        let pdu = PrimaryUnit::from_toml(unit.as_inline_table()?)?;
                        map.insert(pdu.get_iden().clone(), pdu);
                    }
                    Some(map)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    // /// Adds to manifest file to set as dynamic.
    // pub fn set_as_dynamic(&mut self) -> () {
    //     self.data.get_mut_doc().as_table_mut()["dynamic"] = value(true);
    // }
}

use crate::core::pkgid::PkgPart;
use crate::core::version::Version;

const SPEC_DELIM: &str = "=";

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct IpSpec(PkgPart, Version);

impl IpSpec {
    pub fn new(id: PkgPart, version: Version) -> Self {
        Self(id, version)
    }

    pub fn get_name(&self) -> &PkgPart {
        &self.0
    }

    pub fn get_version(&self) -> &Version {
        &self.1
    }
}

impl FromStr for IpSpec {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // split by delimiter
        match s.split_once("=") {
            Some((n, v)) => {
                Ok(Self::new(PkgPart::from_str(n)?, Version::from_str(v)?))
            },
            None => {
                Err(Box::new(AnyError(format!("missing specification delimiter {}", SPEC_DELIM))))
            }
        }
    }
}

impl std::fmt::Display for IpSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} v{}", self.0, self.1)
    }
}

impl From<(PkgPart, Version)> for IpSpec {
    fn from(value: (PkgPart, Version)) -> Self {
        Self(value.0, value.1)
    }
}

use serde::{Deserialize, Serialize};
use serde::Serializer;
use serde::de::{self};
use std::fmt;

impl<'de> Deserialize<'de> for IpSpec {
    fn deserialize<D>(deserializer: D) -> Result<IpSpec, D::Error>
        where D: de::Deserializer<'de>
    {
        struct LayerVisitor;

        impl<'de> de::Visitor<'de> for LayerVisitor {
            type Value = IpSpec;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a 256-character checksum")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: de::Error, {
                
                match IpSpec::from_str(v) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(de::Error::custom(e))
                }
            }
        }

        deserializer.deserialize_map(LayerVisitor)
    }
}

impl Serialize for IpSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}{}{}", self.get_name(), SPEC_DELIM, self.get_version()))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compute_checksum() {
        let sum = Ip::compute_checksum(&PathBuf::from("./tests/env/project1/"));
        assert_eq!(sum, Sha256Hash::from_u32s([
            2472527351, 1678808787, 3321465315, 1927515725, 
            108238780, 2368649324, 2487325306, 4053483655]))
    }

    #[test]
    fn from_str_ip_spec() {
        let ip = format!("name{}1.0.0", SPEC_DELIM);

        assert_eq!(IpSpec::new(PkgPart::from_str("name").unwrap(), Version::from_str("1.0.0").unwrap()), IpSpec::from_str(&ip).unwrap());
    }
}