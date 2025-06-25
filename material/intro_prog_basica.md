---
title:  Uma introdução sucinta à Rust
author: Christiano Braga
date:   Março 2025
---

# Instalação 

1. Rust site: http://rust-lang.org
2. Instalação: https://www.rust-lang.org/tools/install
3. Editor: Visual studio code - https://code.visualstudio.com/
    - Extensão ao VSCode: rust-analyzer e Rust Assist
4. Abra um terminal no Visual Code (menu Terminal > New terminal) e execute:
    - `rustc --version`
    - `cargo --version`
    - Se rust já estiver instalado: `rustup update`
5. Rust online - playground: https://play.rust-lang.org/
    ```rust
    fn main() {
        println!("Hello world!")
    }
    ```
    - Debug: Otimizações
    - Share: `gist`
    - Tools: Format, linter, análise de código e expansão de macros. (`println!` é uma macro.)
    - Help: https://play.rust-lang.org/help

- OBS: Todas os comandos utilizados neste documento foram executados num sistema Ubuntu 24.04.2 LTS.

# Programação básica 

<!-- (89 min) = 1h e 29 min -->

## Cargo: Rust build tool e package manager 

1. Execute `cargo new prog_basica` para criar o projeto `prog_basica`.
   ```shell
   cargo new prog_basica
    Creating binary (application) `prog_basica` package
   note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
   ```
2. Execute `tree prog_basica` para visualizar a estrutura de `prog_basica`.
   ```shell
   $ tree prog_basica
    prog_basica
    ├── Cargo.toml
    └── src
        └── main.rs
    
    2 directories, 2 files
    ```
    - Se não tiver `tree` instalado: `sudo apt-get install tree`.

## Comentários e formatação 

1. Comentários: como em C
2. Execute `cd prog_basica ; code .`
3. Abra o arquivo `src/main.rs` (`CTRL+O`)
1. Modifique `src/main.rs`:
    ```rust
    fn main() {
       print!("Hello, world!");
       print!("Hello again...");
    }
    ```
    - Abra um terminal e execute `cargo run`.
    ```shell
    $ cargo run
       Compiling prog_basica v0.1.0 (/home/cbraga/Dropbox/aulas/lp/2025.1/rust/prog_basica)
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
         Running `target/debug/prog_basica`
    Hello, world!Hello again...%     
    ```
    - Note a formatação da saída.
    - Agora adicione `println!(10);` como último comando da função `main`.
        - Se estiver no vscode, haverá uma marcação sob o número 10.
        - Ao passar o mouse sobre o número 10, aparecerá um texto com conteúdo igual ao que resulta da execução de `cargo run`.
            ```shell
            $ cargo run
               Compiling prog_basica v0.1.0 (/home/cbraga/Dropbox/aulas/lp/2025.1/rust/prog_basica)
            error: format argument must be a string literal
             --> src/main.rs:4:14
              |
            4 |     println!(10);
              |              ^^
              |
            help: you might be missing a string literal to format with
              |
            4 |     println!("{}", 10);
              |              +++++
    
            error: could not compile `prog_basica` (bin "prog_basica") due to 1 previous error
            ```
        - O compilador rust nos indica qual o error, onde está e como corrigí-lo.
        - Modificando a última linha da função `main` como sugerido, assim como os comandos `print!` para `println!`, o erro desaparece e a saída do programa fica um pouco mais legível.      
          ```shell
          $ cargo run
              Compiling prog_basica v0.1.0 (/home/cbraga/Dropbox/aulas/lp/2025.1/rust/prog_basica)
              Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
                  Running `target/debug/prog_basica`
          Hello, world!
          Hello again...
          10
          ```
    - As funções de formatação também podem ser parametrizadas por índices ou nomes, como a seguir:
        ```rust
        println!("Por índices: {1} é uma {0} para programação {2}.", "linguagem", "Rust", "de sistemas");
        println!("Por nomes: {nome} é uma {tipo} para programação {tipo_lang}.", tipo="linguagem", nome="Rust", tipo_lang="de sistemas");
        ```
        * Veja o resultado executando `cargo run`.
    - Para imprimir um valor em ponto flutuante é necessário especificar sua formatação no 1o. parâmetro da função de impressão.
        * Modifique `println!("{}", 10)` para `println!("{:.2}", 10.0)` e observe o resultado.

## Variáveis, tipos escalares e _shadowing_ 

### Variáveis 

- Declare uma variável utilizando o comando `let`:
    ```rust
    let x = 10;
    ```
- Vscode completará sua declaração como o tipo _inferido_ para a variável `x`. Por _default_, inteiros são do tipo `i32` (inteiros com 32 bits).
- Modifique o valor sendo atribuído à `x` para 10.0 e observe o resultado.
- O tipo _default_ para valores de ponto flutuante é `f64`. Modifique-o para `f32` e verifique se há alguma diferença. (Quando esta mudança fará diferença?)
- Modifique o programa `main.rs` no projeto `prog_basica` para que `x` fique associado à um valor em ponto flutuante e que o programa o imprima como tal, utilizando tantas casas decimais quanto seu valor necessitar.
- A declaração `let x = 10;` faz com que `x` seja uma variável _imutável_. 
    - Adicione uma atribuição à `x`, logo a seguir à sua declaração, e observe o comportamento do vscode. 
- A saída a seguir é impressa ao executarmos `cargo run`:
    ```shell
    $ cargo run
       Compiling prog_basica v0.1.0 (/home/cbraga/Dropbox/aulas/lp/2025.1/rust/prog_basica)
    warning: value assigned to `x` is never read
      --> src/main.rs:12:9
       |
    12 |     let x = 10.0 ;
       |         ^
       |
       = help: maybe it is overwritten before being read?
       = note: `#[warn(unused_assignments)]` on by default

    error[E0384]: cannot assign twice to immutable variable `x`
      --> src/main.rs:13:5
       |
    12 |     let x = 10.0 ;
       |         - first assignment to `x`
    13 |     x = 15.0;
       |     ^^^^^^^^ cannot assign twice to immutable variable
       |
    help: consider making this binding mutable
       |
    12 |     let mut x = 10.0 ;
       |         +++

    For more information about this error, try `rustc --explain E0384`.
    warning: `prog_basica` (bin "prog_basica") generated 1 warning
    error: could not compile `prog_basica` (bin "prog_basica") due to 1 previous error; 1 warning emitted    
    ```
- O compilador sugere que a declaração de `x` seja modificada, de forma a permitir que `x` seja considerada _mutável_.
- Após a modificação acima, o código passa ser o seguinte:
    ```rust
    fn main() {

        // Printing

        println!("Hello, world!");
        println!("Hello again...");
        println!("{}", 10);
        println!("Por índices: {1} é uma {0} para programação {2}.", "linguagem", "Rust", "de sistemas");
        println!("Por nomes: {nome} é uma {tipo} para programação {tipo_lang}.", tipo="linguagem", nome="Rust", tipo_lang="de sistemas");

        // Variaveis
        let mut x = 10.0 ;
        x = 15.0;
        println!("{:.2}", x);
    }
    ```
- Note que o compilador (através do vscode) ainda emite um _warning_: o valor 10 nunca é lido. Essa informação é mostrada ao passarmos o mouse sobre a declaração de `x` ou se executarmos `cargo run`.
    ```shell
    $ cargo run
       Compiling prog_basica v0.1.0 (/home/cbraga/Dropbox/aulas/lp/2025.1/rust/prog_basica)
    warning: value assigned to `x` is never read
      --> src/main.rs:12:13
       |
    12 |     let mut x = 10.0 ;
       |             ^
       |
       = help: maybe it is overwritten before being read?
       = note: `#[warn(unused_assignments)]` on by default

    warning: `prog_basica` (bin "prog_basica") generated 1 warning
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
         Running `target/debug/prog_basica`
    Hello, world!
    Hello again...
    10
    Por índices: Rust é uma linguagem para programação de sistemas.
    Por nomes: Rust é uma linguagem para programação de sistemas.
    15.00    
    ```
    * Por que este _warning_ é emitido? Como corrigi-lo?

- As regras para definição de identificadores de variáveis (e símbolos em geral) são as usuais das linguagens de programação modernas. Rust utiliza _snake case_ para variáveis.

### Tipos escalares 

- Inteiros 
    * com sinal: i8, i16, i32, e i64 
    * sem sinal: u8, u16, u32, e u64
- Ponto flutuante: f32 e f64. 
    * O tipo f64 é o _default_ quando uma variável é inicializada com um float.
    * Declare `let x = 10.0` e observe a resposta do vscode.
    * As constantes `MAX`e `MIN` estão definidas para cada um destes tipos no módulo `std` de Rust.
        * Inclua as sentenças abaixo no seu programa:
        ```rust
        println!("f32 MAX {}", std::f32::MAX);
        println!("f32 MAX {}", std::f32::MAX);
        ```
        * Faça o mesmo para os diferentes tipos de inteiro.
- O operador `as` é utilizado para fazer coerção de tipos (_type casting_).
    ```rust
    let k1 = 10;
    let k2 = 15.0;
    println!("k1 + k2 = {}", k1 + k2);
    ```
    - O código acima produz um erro pois i32 e f64 não podem ser somados.
    - Se convertermos `k2` para i32 antes da soma, utilizando o operador `as`, como em `k2 as f32`, então o erro de tipagem é resolvido. (Que outra solução existe ao erro de tipagem nesta expressão?)

- Rust possui o tipo `bool` com os operadores lógicos`tradicionais.
- Valores do tipo `char` são definidos com notação usual como `'a'` ou `'\u{U+2211}'`.
    * Modifique seu programa para imprimir estes caracteres.
- Valores inteiros decimais , por exemplo, podem ser facilmente impressos em outras bases comom octal, hexadecimal ou binária.
    ```rust
    let z = 10;
    println!("z in decimal is {}, in octal is {:o}, in hexadecimal is {:X} and in binary is {:b}.", z, z, z, z);
    ```
- Valores inteiros também podem ser expressos utilizando `_` como em Python para melhorar a legibilidade, com por ex. `1_000_000`.

### _Shadowing_ e constantes 

- Uma declaração de variável pode ser ocultada (_shadowed_) por outra declaração de variável com o mesmo nome.
    ```rust
    let x = 0;
    let x = 'A';
    println!("This is x = {}", x);
    ```
    * O último valor de `x` será impresso.
- Escopos podem ocultar variáveis do ambiente, como em C por exemplo.
- Uma variável imutável que oculta uma mutável faz com que o símbolo sendo declarado seja imutável. Vale a última declaração.

- Constantes são declaradas com a palavra reservada `const` ao invés de `let`. 
    * O identificador da constante deve utilizar snake case maiúsculo. (Por exemplo `X` ao invés de `x`.) 
    * Seu tipo não é inferido, cabe ao programador declará-lo. 
    * Diferentemente de variáveis, constantes não podem ser ocultadas.

## Tipos compostos: _string_, tuplas e vetores

### Strings

- `&str`: String slices
    * São o tipo _default_ de uma variável contendo texto (`&[u8]`), assim como `i32` para números sem ponto flutuante e `f64` para ponto flutuante.
    ```rust
    fn main() {
        let tx = "Olá mundo";
        println!("{}", tx)
    }
    ```
    * É uma referência à um valor do tipo `str`.
    * Tempo de vida (_lifetime_) estático. (Mais sobre isso a frente.)

- `String`
    * Vetor (`Vec<u8>`) de bytes, alocado na _heap_, pode aumentar, e não é terminado por _null_.
    ```rust
    fn main() {
        let mut tx = String::from("Olá mundo");
        tx.push('?');
        println!("{}", tx);
        tx.pop();
        println!("{} Ops...", tx);
        tx.push('!');
        println!("{}", tx);
    }
    ```
    * Algumas funções sobre strings:
    ```rust
    fn main() {
        let mut tx = String::from("Olá ");
        println!("tx: {}\ntamanho: {}\ncontém \"Olá\": {}", 
            tx,
            tx.len(),
            tx.contains("Olá")
        );
        tx.push_str("mundo!");
        println!("tx: {}", tx)
    }
    ```
    * String vazia com `String::new()`:
    ```rust
    fn main() {
        let mut s = String::new();
        s.push_str("A song by U2");
        println!("{}", s);
    }    
    ```
    * Formating:
    ```rust
    fn main() {
        let s1 = "Hello, hello...".to_string();
        let s2 = "...We're at a place called Vertigo (¿dónde está?)".to_string();
        let s3 = format!("{} (Hola!) {}", s1, s2);
        println!("{}", s3);
    }    
    ```
### Tuplas
- Seus componentes podem ser modificados mas novos componentes não podem ser adicionados nem removidos.
- Para acessar um componente de uma tupla escrevemos `t.n` onde `t` é a variavel do tipo tupla e `n` é o componente que desejamos projetar.
- Note o uso da sintaxe `{:?}`. A macro `println!` não sabe imprimir dados complexos (não escalares). Esta notação é necessária nestes casos. 
    ```rust
    fn main(){
        let mut s = 
            ("Aloísio", 34, "Rua Mariz e Barros", 33, 403, "(21) 9555 2345");
        println!("{:?}", s);
        println!("{}{}{}{}{}", s.0, s.4, s.1, s.3, s.2);
        s.0 = "Maurício";
        println!("{:?}", s);
    }
    ```
- Projeção de tuplas aninhadas:
    ```rust
    fn main(){
        let s = ((2.3, 4.5), (7.8, 0.1), "Two points?");
        println!("{}", s.1.0);
    }    
    ```
- Projeção por descontrução (_destructuring_):
    ```rust
    fn main(){
        let (s0, s1, s2, s3, s4, s5) = 
            ("Aloísio", 34, "Rua Mariz e Barros", 33, 403, "(21) 9555 2345");
        println!("{}", s3);
    }
    ```
### Cadeias (_Arrays_)
- Como em outras linguagens (não Python) cadeias são estruturas ordenadas e homogêneas.
- Seu tipo é informado com sintaxe `[T,n]` onde `T` é o tipo de seus elementos e `n` o tamanho da cadeia. 
- Têm a sintaxe usual de linguagens de programação para inicialização, acesso indexado e atribuição.
    ```rust
    fn main(){
        let v = [1,2,3,4,5];
        println!("{:?}",v[0]);
    }    
    ```
- Inicialização com o mesmo valor em todos os índices:
    * Declaração:
    ```rust
    fn main(){
        let v = [0; 10];
        println!("{:?}", v);
    }
    ```
    * Execução:
    ```shell
    $ cargo run
    Compiling rust-aula1 v0.1.0 (/home/cbraga/Dropbox/aulas/lp/2025.1/rust/rust-aula1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
    Running `target/debug/rust-aula1`
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]    
    ```
- Slices:
    * É possível representar intervalos com sintaxe similar a de Python.
    * Através do operador `=` podemos incluir o último elemento do intervalo no slice, que não é incluído, caso contrário.
    ```rust
    fn main() {
        let v = [0,1,2,3,4];
        let mut s = &v[0..3];
        println!("{:?}", s);
        s = &v[0..=3];
        println!("{:?}", s);
    }
    ```
    * Execução:
    ```shell
    $ cargo run
    Compiling rust-aula1 v0.1.0 (/home/cbraga/Dropbox/aulas/lp/2025.1/rust/rust-aula1)
     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
      Running `target/debug/rust-aula1`
    [0, 1, 2]
    [0, 1, 2, 3]
    ```
- Tamanho em bytes:
    - Declaração:
    ```rust
    fn main() {
        let v = [4,5,6,7];
        println!("{}", std::mem::size_of_val(&v));
    }
    ```
    - Execução:
    ```shell
    $ cargo run
        Compiling rust-aula1 v0.1.0 (/home/cbraga/Dropbox/aulas/lp/2025.1/rust/rust-aula1)
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
            Running `target/debug/rust-aula1`
    16
    ```
    - O tipo de cada elemento é $32$ bits $=$ $4$ bytes. Como o vetor tem $4$ posições, o tamanho do vetor em bytes é $4 \times 4 = 16$.

### Vetores (_Vectors_)
- Assim como um _array_, um _vector_ é uma estrutura homogênia, armazanada contiguamente na memória.
- Diferentemente dos _arrays_, um _vector_ não tem tamanho fixo.
- O tipo `Vec` é *genérico* (com sintaxe similar a Java).
    ```rust
    fn main() {
        let v = vec![4,5,6,7];
        println!("{}", std::mem::size_of_val(&v));
    }
    ```
- As operações sobre _arrays_ também valem para _vectors_.
- Instâncias do tipo `Vec` podem crescer e diminuir. As funções `push`, `pop` e `remove`, por exemplo, permitem este tipo de manipulação.
    ```rust
    fn main() {
        let mut v = vec![4,5,6,7];
        println!("{}", std::mem::size_of_val(&v));
        v.push(8);
        println!("{:?}", v);
        v.pop();
        println!("{:?}", v);
        v.remove(2);
        println!("{:?}", v);
    }
    ```
- A função `contains` verifica se um determinado elemento (**passado por referência**) pertence ou não ao _vector_.
    ```rust
    fn main() {
        let mut v = vec![4,5,6,7];
        println!("{}", std::mem::size_of_val(&v));
        v.push(8);
        println!("{:?}", v);
        v.pop();
        println!("{:?}", v);
        v.remove(2);
        println!("{:?}", v);
        println!("{}", v.contains(&6));
    }
    ```

## Funções
- Funções em Rust têm uma sintaxe ligeiramente diferente de outras linguagens para desenvolvimento de sistemas (_system development languages_) como `C` e `C++`.
- O programa a seguir computa as raizes de um polinômio do 2o. grau pela fórmula de báscara:
    ```rust
    fn roots(a:f32, b:f32, c:f32) -> (f32, f32) {
        let delta:f32 = 
            b.powf(2f32) - 4f32 * a * c;
        let root1 = 
            (-b + delta.sqrt()) / (4f32 * a);
        let root2 = 
            (-b - delta.sqrt()) / (4f32 * a);
        (root1, root2)
    }

    fn main() {
        println!("{:?}", roots(-2.5,40.7,4.9))
    }
    ```
- Algumas coisas dignas de nota:
    * Como já havíamos visto na função `main`, funções são declaradas com a palavra-chave `fn`.
    * A declaração da função deve ser tipada, como fazemos em `C` ou `C++`, porém a forma de fazê-la lembra linguagens funcionais tipadas como Haskell.
    * A representação matemática da função `roots` é:
    $roots : \mathcal{R}^3 \to \mathcal{R}^2$, isto é, $roots$ recebe três números reais e retorna dois números reais.
    * Não é necessário o uso da palavra `return`. A função retorna o valor na última linha do bloco que codifica seu corpo. Seria necessário somente se em algum ponto no corpo da função (resultado de um condicional, por exemplo) fosse necessário retornar da função.
    * Se colocássemos um `;` após o par `(root1, root2)` um erro seria produzido. 
        - Lembre-se que `;`não um marcador de fim de linha ou fim de comando. É uma operação de sequenciamento de comandos.
        - Considerando, somente para efeito didático, que existe um comando `nop`, que não faz nada, a sentença `(root1, root2);` significaria `(root1, root2) ; nop`, e o retorno da função seria `nop`, que não tem o tipo `(f32, f32)`.

- Rust permite que uma variável seja inicializada com o retorno de um bloco, como por exemplo:
    ```rust
    fn main() {
        // println!("{:?}", roots(-2.5,40.7,4.9))

        let x = {
            let pi = 3.1415;
            let r  = 2.0;
            pi * r * r 
        } ;
        println!("{}", x)
    }    
    ```
    * A forma mais fácil de entender o código acima é perceber que um bloco é uma função anônima sem parâmetros.

## Entrada de dados
- Pela linha de comando
    ```rust
    use std::env;

    fn main() {
        let args: Vec<String> = env::args().collect();

        let arg1 = &args[1];
        let arg2 = &args[2];

        println!("First argument: {arg1}");
        println!("Second argument: {arg2}");
    }
    ```
- Via `std::io::stdin`:
    * Declaration:
    ```rust
    fn main() {
        let mut n = String::new();
        std::io::stdin()
            .read_line(&mut n)
            .expect("Input failed!");

        let n:f32 = n.trim().parse().expect("Invalid input!");
        println!("{}", n)
    } 
    ```
    * Execução correta:
    ```shell
    $ cargo run
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
         Running `target/debug/rust-aula1`
    34.6
    34.6    
    ```
    * Execução com erro:    
    ```shell    
    $ cargo run   
        Finished `dev` profile [unoptimized     + debuginfo] target(s) in 0.00s
         Running `target/debug/rust-aula1`  
    Ola 

    thread 'main' panicked at src/main.rs:7:    34:
    Invalid input!: ParseFloatError { kind: Invalid }
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace    
    ```
