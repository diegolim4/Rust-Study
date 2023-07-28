/* 
    Memórias Static, Stack e Heap

    Em Rust precisamos programar com uma abordagem chamada Memory awereness
    Sempre que definimos uma variavel precisamos ter uma noção exata de 
    qual espaço de memoria esse valor vai ser alocado.
    O que ajuda na perfomace do programa

    Os 3 espaços de memorias e como o Rust utiliza-os:

    - Static
     Um espaço fixo de memória que é alocado quando o programa começa a ser execultado    
    
    - Stack
     Aqui é alocado as variaveis do tipo primitivo: int, string, bool, arr, etc..
     Ele tem um tamanho fixo tmbm mas é possivel remover e colocar elementos dentro dele

    - Heap
     Aqui os valores alocados são dinamicos e grandes recebemos atráves de um input do usuário ou retorno 
     de uma base de dados.
      


*/