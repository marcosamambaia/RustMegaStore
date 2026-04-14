===============================================================
Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore
===============================================================




##  Descrição
Este projeto implementa um **sistema de busca otimizado** e um **motor de recomendação de produtos** para o catálogo da MegaStore, utilizando **Rust**, **SQLx** e **Petgraph**.  
O objetivo é oferecer buscas rápidas, precisas e recomendações inteligentes em um catálogo com milhões de itens.

---

##  Tecnologias utilizadas
- **Rust** (linguagem principal)
- **SQLx** (integração com MySQL)
- **Petgraph** (estrutura de grafos para recomendações)
- **Eframe/Egui** (interface gráfica)
- **Tokio** (runtime assíncrono)
- **Cargo test** (testes unitários e de integração)

---

##  Como executar
1. Clone o repositório:
   ```bash
   git clone https://github.com/marcosamambaia/RustMegaStore
   cd RustMegaStore


2. Configure o banco MySQL:
```CREATE DATABASE megastore;
CREATE USER 'megastore'@'%' IDENTIFIED BY '1234';
GRANT ALL PRIVILEGES ON megastore.* TO 'megastore'@'%';
FLUSH PRIVILEGES;
```
3. Configure e execute:
```cargo run
```

## Como rodar os testes

cargo test

## Os testes cobrem:

Busca por nome e categoria (search_tests.rs)

Recomendações de produtos (recommendation_tests.rs)

Inserção de novos produtos (insert_tests.rs)

## Arquitetura do sistema
db.rs → Funções de acesso ao banco (busca, inserção).

recommendation.rs → Motor de recomendação baseado em grafos.

gui.rs → Interface gráfica com busca, cadastro e recomendações.

main.rs → Ponto de entrada da aplicação.

tests/ → Testes unitários e de integração.

## Algoritmos e Estruturas de Dados
Tabela Hash → Indexação rápida de produtos por nome, marca e categoria.

Grafos (Petgraph) → Representação das relações entre produtos (categoria, marca).

Busca linear + filtros → Para consultas simples no catálogo.

## Desempenho e Escalabilidade
Consultas otimizadas com SQLx e índices no banco.

Estruturas de grafos permitem recomendações rápidas mesmo com grandes volumes.

Escalável para milhões de produtos, desde que o banco esteja indexado corretamente.

## Contribuições
Contribuições são bem-vindas!
Faça um fork, crie uma branch e envie um pull request.

## Licença
Este projeto está licenciado sob a MIT License.
