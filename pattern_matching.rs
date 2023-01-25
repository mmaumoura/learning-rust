fn main(){
    for i in 1..=20 {
        println!("{}: {}", i, match i{
            1 => "Muito pouco",
            2 | 3 => "Pouco",
            4..=10 => "Bastante",
            _ if i % 2 == 0 => "Número par acima de 10",
            _ => "Muito"
        });
    }
}