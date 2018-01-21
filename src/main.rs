use std::collections::HashSet;

fn normalize_path(path_string: &String) -> String {
    let path_iter = path_string.split(':');
    let mut list: Vec<String> = Vec::new();
    let mut set: HashSet<String> = HashSet::new();
    for path in path_iter {
        if !set.contains(path) {
            set.insert(path.to_string());
            list.push(path.to_string())
        }
    }
    list.join(":")
}

fn main() {
    let mut path_string = "".to_string();

    if let Some(paths) = std::env::var_os("PATH") {
        path_string = paths.to_str().unwrap().to_string();
    }

    let result = normalize_path(&path_string);
    println!("shorten the length of environment variable PATH from {} bytes to {} bytes.", path_string.len(), result.len());
    println!("copy and paste below line to your .bashrc or etc.\n");
    println!("export PATH=\"{}\"", result);
}
