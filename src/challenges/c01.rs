use axum::extract::Path;

pub async fn nested_xor(Path((first, second)): Path<(u32, u32)>) -> String {
    (u32::pow(first ^ second, 3)).to_string()
}

pub async fn calculate_sled_id(Path(path): Path<String>) -> String {
    let numbers = path
        .split('/')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let xored = numbers.into_iter().fold(0, |acc, num| acc ^ num);
    let powed = xored.pow(3);
    powed.to_string()
}
