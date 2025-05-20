use rand::seq::IndexedRandom;

pub fn string_by_length(length: i32) -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let mut rng = rand::rng();
    let random_string = (0..length)
        .map(|_| *chars.choose(&mut rng).unwrap())
        .collect::<String>();
    format!("start-{}-end", random_string)
}
