use std::io;
use std::path::PathBuf;

use embed_manifest::embed_manifest_file;

fn main() {
    let sdk = get_winsdk_include().unwrap();

    windres::Build::new()
        .include(sdk.join("um"))
        .include(sdk.join("shared"))
        .compile("windows/runner/Runner.rc")
        .unwrap();

    embed_manifest_file("windows/runner/runner.exe.manifest").unwrap();
}

fn get_winsdk_include() -> io::Result<PathBuf> {
    let mut paths = std::fs::read_dir(r"C:\Program Files (x86)\Windows Kits\10\Include")?
        .flatten()
        .map(|e| e.path())
        .collect::<Vec<_>>();
    paths.sort();
    Ok(paths.into_iter().next_back().unwrap())
}
