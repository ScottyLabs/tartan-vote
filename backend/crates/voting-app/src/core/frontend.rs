use std::path::{Path, PathBuf};

/// Resolve the built frontend directory from env, a Nix-bundled path, or dev `frontend/dist`.
pub fn resolve_frontend_dist() -> Option<PathBuf> {
    if let Ok(raw) = std::env::var("FRONTEND_DIST") {
        let path = PathBuf::from(raw);
        if index_exists(&path) {
            return Some(path);
        }
    }

    if let Ok(exe) = std::env::current_exe()
        && let Some(exe_dir) = exe.parent()
    {
        let bundled = exe_dir.join("../share/tartan-vote/static");
        if index_exists(&bundled) {
            return bundled.canonicalize().ok();
        }
    }

    let dev_path = PathBuf::from("frontend/dist");
    if index_exists(&dev_path) {
        return Some(dev_path);
    }

    None
}

fn index_exists(dir: &Path) -> bool {
    dir.join("index.html").is_file()
}
