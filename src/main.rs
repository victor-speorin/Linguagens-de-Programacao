use std::env;
use std::io;
use std::io::stdin;

fn roots(a:f32, b:f32, c:f32) -> (f32, f32) { // sintaxe= nomefunc(parametros:tipo) -> (retornos)
    let delta:f32 = b.powf(2f32) - 4f32 * a * c;
    let root1 = -b + delta.sqrt() / (2f32 * a);
    let root2 = -b - delta.sqrt() / (2f32 * a);
    (root1, root2)
}

fn main() {
    // CONSTANTES
    const X:i32 = 10; //têm que ter tipo declarado e não pode ser ocultada

    // STRINGS

    // &str
    let tx = "Olá mundo"; // String slices são o tipo default contexto texto (&[u8])
    println!("{}", tx);
    // String
    let mut tx = String::from("Olá mundo"); // Vetor(Vec<u8>) alocado na heap, pode aumentar e não é termminado por null
    println!("{}", tx);
    // Algumas funções para String
    // funciona como pilha
    tx.push('?'); 
    println!("{}", tx);
    tx.pop();
    println!("{}", tx);
    // funcoes utilitarias
    println!("tx: {}\ntamanho: {}\ncontém \"Olá\": {}", tx, tx.len(), tx.contains("Olá"));
    // String vazia
    let mut s = String::new();
    s.push_str("A song by U2"); //push de uma string inteira
    println!("s: {}", s);
    // formatando
    let s1 = "Hello, hello...".to_string(); // transformando &str para String
    let s2 = "...We're at a place called Vertigo".to_string();
    let s3 = format!("{} (Hola!) {}", s1, s2);
    println!("{}", s3);

    // TUPLAS

    // seu tamanho não pode mudar mas seus valores podem ser modificados
    // não tem impressao definida logo é utilizado o {:?}
    let mut tp = ("Aloisio", 34, "Rua Mariz e Barros", 33, 403, "(21) 9555 2345");
    println!("{:?}", tp);
    tp.0 = "Mauricio";
    println!("{:?}", tp);

    // Tuplas alinhadas
    let tpa = ((2.3, 4.5), (7.8, 0.1), "Two points?");
    println!("{}", tpa.1.0);

    //Projeção por descontrução
    let (s0, s1, s2, s3, s4, s5) = 
        ("Aloisio", 34, "Rua Mariz e Barros", 33, 403, "(21) 9555 2345");
    println!("{}", s3);

    // ARRAYS

    //inicialização com [T,n] onde T é o tipo e n o tamanho
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr[0]);
    // inicializando com o mesmo valor em todos os indices
    let arr0 = [0;10];
    println!("{:?}", arr0);

    // SLICES
    let vet = [0,1,2,3,4];
    let mut sli = &vet[0..3];
    println!("{:?}", sli);
    sli = &vet[0..=3];
    println!("{:?}", sli);

    // tamanho em bytes
    let vtam = [4,5,6,7];
    println!("{}", size_of_val(&vtam));

    // Vetores
    // o vector nao tem tamanho fixo
    let mut vect = vec![4,5,6,7];
    println!("{}", size_of_val(&vect));

    //funcões que mudam tamanho do vec
    vect.push(8);
    println!("{:?}", vect);
    vect.pop();
    println!("{:?}", vect);
    vect.remove(2);
    println!("{:?}", vect);
    // função que vê se um vec contem
    println!("{}", vect.contains(&6));

    // Funções (la em cima)
    println!("{:?}", roots(-2.5, 40.7, 4.9));

    /* 
    let args: Vec<String> = env::args().collect();
    let arg1 = &args[1];
    let arg2 = &args[2];
    println!("Primeiro arg: {arg1}");
    println!("Segundo arg: {arg2}");
    */
    
    let mut n = String::new();
    stdin().read_line(&mut n).expect("imput failed!");
    let n:f32 = n.trim().parse().expect("Invalid input!");
    println!("{}", n)
}