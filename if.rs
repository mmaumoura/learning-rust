fn main(){
    let idade:u8 = 17;
    let responsavel_autorizou = true;
    let maior_de_idade = idade >= 18;

    if maior_de_idade {
        println!("Permitida a entrada!");
    } else if idade > 18 || idade > 16 && responsavel_autorizou {
        println!("Permitida a entrada com assinatura do responsável!");
    } else {
        println!("Proibida a entrada!");
    }

    let condicao = if maior_de_idade { "maior" } else { "menor" }

    println!("É {} de idade!", condicao);
} 