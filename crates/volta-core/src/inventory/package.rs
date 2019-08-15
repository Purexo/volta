use std::{collections::BTreeSet, iter::Iterator};

use log::debug;
use walkdir::WalkDir;

use crate::path;
use crate::tool::PackageConfig;
use volta_fail::Fallible;

// Convenience for access as `package::Collection`
pub use PackageCollection as Collection;

#[derive(Clone)]
pub struct PackageCollection(BTreeSet<PackageConfig>);

impl PackageCollection {
    pub(crate) fn load() -> Fallible<Self> {
        let package_dir = path::user_package_dir()?;

        WalkDir::new(&package_dir)
            .max_depth(1)
            .into_iter()
            // Ignore any items which didn't resolve as `DirEntry` correctly.
            // There is no point trying to do anything with those, and no error
            // we can report to the user in any case. Log the failure in the
            // debug output, though
            .filter_map(|entry| match entry {
                Ok(dir_entry) => {
                    // Ignore directory entries.
                    if dir_entry.file_type().is_file() {
                        Some(dir_entry.into_path())
                    } else {
                        None
                    }
                }
                Err(e) => {
                    debug!("{}", e);
                    None
                }
            })
            .map(|file_path| PackageConfig::from_file(&file_path))
            .collect::<Fallible<BTreeSet<PackageConfig>>>()
            .map(PackageCollection)
    }
}

impl IntoIterator for PackageCollection {
    type Item = PackageConfig;
    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
