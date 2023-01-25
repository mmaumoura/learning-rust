fn main () {
    let multiplicador:u8 = 5;
    let mut contador:u8 = 0;

    while contador < 10 {
        contador += 1;
        if contador == 5 {
            continue;
        }
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }

    contador = 0;
    loop {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
        if contador == 10 {
            break;
        }
    }
    
    //for i in 1..=10 - sinônimo      
    for i in 1..11 { //realiza um intervalo entre 1 e 10, não chega até o último número.
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }

}