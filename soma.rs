fn soma(a:i32, b:i32) -> i32
//retorno da função é especificado por ->
{
    println!("{} + {} = {}", a, b, a + b);
    a + b
}
//para retornar o que está dentro da função, é necessário remover o ponto e vírgula.

fn main()
{
    println!("Soma = {}", soma(2,2));
}