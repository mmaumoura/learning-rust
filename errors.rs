fn main(){
    match resultado() {
        Ok(mensagem) => println!("Mensagem de sucesso = {}", mensagem),
        Err(numero) => println!("Código de erro = {}", numero)
    }
}

fn resultado() -> Result<String, u8>{
    Err(42)
}