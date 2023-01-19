//constantes e variáveis globais precisam ter os tipos definidos;
const PI:f32 = 3.14;
static VARIAVEL_GLOBAL:u8 = 1; //static é usado para a criação de uma variável global.
fn main()  {
	println!("PI = {}", PI);
	println!("VARIAVEL_GLOBAL = {}", VARIAVEL_GLOBAL);

	// unsafe{} - define que algo não é seguro;

	let variavel = 300;
	println!("variavel = {}, tamanho {} bytes", variavel, std::mem::size_of_val(&variavel));

	let decimal:f32 = 2.5;
	println!("decimal = {}", decimal);

	let boolean = false;
	println!("boolean = {}", std::mem::size_of_val(&boolean));

	//let mut variavel - para variáveis mutáveis

	let letra:char = 'C';
	println!("char = {}", std::mem::size_of_val(&letra));
}