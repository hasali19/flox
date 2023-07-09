use bindgen::CargoCallbacks;
use which::which;

fn main() {
    let flutter_path = which("flutter").expect("flutter not found in PATH");
    let flutter_bin = flutter_path.parent().unwrap();
    let header = flutter_bin.join("cache/artifacts/engine/windows-x64/flutter_windows.h");

    let bindings = bindgen::builder()
        .header(header.to_str().unwrap())
        .allowlist_function(r"Flutter\w+")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .unwrap();

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
