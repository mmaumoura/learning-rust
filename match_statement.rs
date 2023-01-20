fn main() {
    let linguagem = "";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "desconhecido"
    };

    println!("O propósito de {} é {}", linguagem, proposito);
}