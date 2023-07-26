O primeiro passo é instalar Rust. Vamos fazer o download de Rust através do rustup, uma ferramenta de linha de comando para gerenciar versões Rust e ferramentas associadas

# Verificar as dependências: 
Certifique-se de ter as dependências corretas instaladas no seu sistema. No caso do Rust, é necessário ter as bibliotecas de desenvolvimento C e C++ instaladas. No Ubuntu e outras distribuições baseadas em Debian, você pode instalá-las usando o seguinte comando:

- sudo apt-get install build-essential

# Instalação no linux e Mac
- curl https://sh.rustup.rs -sSf | sh

# Atualização
- rustup update

# Desinstalar
- rustup self uninstall

# Conferir versão
- rustc --version
ou
- rustup -V

# Limpar e compilar novamente
- Às vezes, o erro pode ocorrer devido a problemas temporários ou corrompidos nos arquivos intermediários de compilação. Tente limpar o diretório de compilação e compilar novamente:
- cargo clean
- cargo build

# Compilar
- rustc name_file.rs

# Execultar o binário
- ./ name_file

# Cargo
A ferramenta de compilação e gerenciamento de pacotes 
Usado em projetos maiores e quando começamos a trabalhar com dependências

# Criando um novo projeto com o cargo (workspace)
- cargo new name-project

# Compilando no workspace
- cargo build

# Execultado o projeto de uma vez
- cargo run

# Formatação automática
- cargo fmt


# MATERIAIS DE ESTUDO 

- doc
https://rust-br.github.io/rust-book-pt-br/ch00-00-introduction.html

- Cargo
https://www.rust-lang.org/pt-BR/learn/get-started

- playlist video do yt
https://www.youtube.com/watch?v=zWXloY0sslE&list=PLjSf4DcGBdiGCNOrCoFgtj0KrUq1MRUME&ab_channel=CodeShow