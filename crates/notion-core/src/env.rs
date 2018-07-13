//! Provides utilities for modifying the environment when a shim calls out to
//! its delegated executable.

use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

use path;

/// Produces a modified version of the current `PATH` environment variable that
/// will find Node.js executables in the installation directory for the given
/// version of Node instead of in the Notion shim directory.
pub fn path_for_installed_node(version: &str) -> OsString {
    let current = env::var_os("PATH").unwrap_or(OsString::new());
    let shim_dir = &path::shim_dir().unwrap();
    let split = env::split_paths(&current).filter(|s| s != shim_dir);
    let mut path_vec: Vec<PathBuf> = Vec::new();
    path_vec.push(path::node_version_bin_dir(version).unwrap());
    path_vec.push(path::yarn_version_bin_dir(version).unwrap());
    path_vec.extend(split);
    env::join_paths(path_vec.iter()).unwrap()
}

/// Produces a modified version of the current `PATH` environment variable that
/// removes the Notion shims and binaries, to use for running system node and
/// executables.
pub fn path_for_system_node() -> OsString {
    let current = env::var_os("PATH").unwrap_or(OsString::new());
    let shim_dir = &path::shim_dir().unwrap();
    let bin_dir = &path::bin_dir().unwrap();
    // remove the shim and bin dirs from the path
    let split = env::split_paths(&current).filter(|s| s != shim_dir && s != bin_dir);
    env::join_paths(split).unwrap()
}
