
fn main() {
    embuild::espidf::sysenv::output();
    let dir = std::fs::read_dir("../raw_records").unwrap();
    let mut paths: Vec<_> = dir
        .filter_map(|e| e.ok())
        .map(|e| e.path().canonicalize().unwrap())
        .filter(|p| p.extension().map(|e| e == "raw").unwrap_or(false)) 
        .collect();
    paths.sort();

    for (i, path) in paths.iter().enumerate() {
        println!("cargo:rustc-env=SOUND_{}={}", i, path.display());
    }
    println!("cargo:rustc-env=SOUND_COUNT={}", paths.len());
}