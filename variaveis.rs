/* 
    Variáveis & mutabilidade

    A palavras reservadas são let & const

    Por padrão, as variáveis são imutáveis.
    No entanto, você ainda tem a opção de tornar a sua variável mutável.

    Para tornar uma variavel multável adicionando mut na frente do nome da variável

    Para ignora uma variavel usamos _ex:
    let _name = 'John'

*/
// Não se pode usar mut em constantes. São sempre imutáveis e pode ser usando dentro ou fora de uma função
// O padrão de declarar uma função é de SCREAMING_SNAKE_CASE
const SECOND_IN_MINUTES: u32 = 60;


fn main() {
    // Ao iniciar uma variável multável como string ela não poderá ser atribuida outro tipo
    let mut name = "John da Silva"; 
    let mut idade = 25;
    println!("Hello {} de {} anos!!", name, idade);

    name = "Mary da Silva";
    idade = 20;
    println!("Hello {} de {} anos!!", name, idade);

    

}

