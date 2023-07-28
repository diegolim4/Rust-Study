/* 
Todo valor em Rust é um tipo de dado, que informa ao Rust que tipos de dados estão sendo
especificados para que saiba como trabalhar com esses dados.
Vamos olhar para dois subconjuntos de tipos de dados: 
- escalar
- composto

Tenha em mente que Rust é uma linguagem de tipagem estática, o que significa que deve 
conhecer os tipos de todas as variáveis em tempo de compilação. O compilador geralmente 
pode inferir que tipo queremos com base no valor e como o usamos.

> Tipos Compostos - Agregamos multiplos valores
    - Tupla = (true, 15, 5.2, 'd')
    - Matriz (Array) = [1, 2, 3, 4, 5]

> Tipos escalares - Representa um valor único
    Rust tem quatro tipos escalares primários: 
    inteiros, float, booleanos e caracteres(char).

- Inteiros 
tipos inteiros com sinal(números negativos ou que usa -) começam com i, sem sinal começa com u

Tamanho	Signed	Unsigned
8-bit	i8	    u8
16-bit	i16	    u16
32-bit	i32	    u32
64-bit	i64	    u64
arch	isize	usize
*/

fn main() {
    let num:  i32 = 50 // mesmo se não definimos um tipo o rust vai assumir por default que é i32

    println!(num)
}



