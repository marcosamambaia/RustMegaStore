====================================================================
  Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore
====================================================================

 DESCRIÇÃO
------------
Sistema de busca otimizado e motor de recomendação para o catálogo da MegaStore.
Desenvolvido em **Rust**, com integração ao **MySQL** via **SQLx**, e recomendações
inteligentes usando **Petgraph** (grafos).

====================================================================
 TECNOLOGIAS
--------------
- Rust (linguagem principal)
- SQLx (ORM assíncrono para MySQL)
- Petgraph (estrutura de grafos)
- Tokio (runtime assíncrono)
- Eframe/Egui (interface gráfica)
- Cargo test (testes unitários e integração)

====================================================================
 BANCO DE DADOS
-----------------
Estrutura principal da tabela de produtos:
```
    CREATE TABLE products (
        id INT AUTO_INCREMENT PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        brand VARCHAR(255) NOT NULL,
        category VARCHAR(255) NOT NULL,
        product_type VARCHAR(255) NOT NULL,
        price DECIMAL(10,2) NOT NULL
    );
```
Usuário dedicado para o sistema:
```
    CREATE USER 'megastore'@'%' IDENTIFIED BY '1234';
    GRANT ALL PRIVILEGES ON megastore.* TO 'megastore'@'%';
    FLUSH PRIVILEGES;
```
String de conexão utilizada:
```
    mysql://megastore:1234@127.0.0.1/megastore
```
====================================================================
 EXECUÇÃO
-----------
1. Clone o repositório:
```
       git clone https://github.com/seuusuario/RustMegaStore.git
       cd RustMegaStore
```
2. Configure o banco MySQL conforme instruções acima.

3. Compile e execute:
```
       cargo run
```

====================================================================
 TESTES
---------
Para rodar os testes automatizados:
```
       cargo test
```
Testes disponíveis:
- Busca por nome e categoria (tests/search_tests.rs)
- Recomendações de produtos (tests/recommendation_tests.rs)
- Inserção de novos produtos (tests/insert_tests.rs)

====================================================================
 ARQUITETURA
---------------
- src/db.rs ........ Funções de acesso ao banco
- src/recommendation.rs .. Motor de recomendação (grafos)
- src/gui.rs ....... Interface gráfica
- src/main.rs ...... Ponto de entrada
- src/lib.rs ....... Reexporta módulos
- tests/ ........... Testes unitários e integração

====================================================================
 ALGORITMOS E ESTRUTURAS
--------------------------
- Hash Tables → indexação rápida por nome, marca e categoria
- Grafos (Petgraph) → relações entre produtos e categorias
- SQL otimizado com índices → consultas eficientes

====================================================================
 DESEMPENHO
-------------
- Consultas otimizadas com SQLx
- Recomendações rápidas via grafos
- Escalável para milhões de produtos

====================================================================
 CONTRIBUIÇÕES
----------------
Faça um fork, crie uma branch e envie um pull request.

====================================================================
 LICENÇA
----------
MIT License

====================================================================
