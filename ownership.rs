fn main(){
    let mut string = String::from("Maurício");
    rouba(&mut string); //faz uma referência mutável
    println!("{}", string);
}

fn rouba(string: &mut String){
    string.push_str(" Sanches");
    println!("{}", string);
}