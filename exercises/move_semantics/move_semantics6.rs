

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data.clone());

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    // 返回此字符串切片的大写等效项，
    data = data.to_uppercase();

    println!("{}", data);
}
