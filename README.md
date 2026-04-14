# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

##  Descrição
Este projeto implementa um sistema de busca e recomendação de produtos para o catálogo da **MegaStore**, utilizando **tabelas hash** para indexação eficiente e **grafos** para recomendações de produtos relacionados.  
O objetivo é oferecer buscas rápidas, precisas e escaláveis, mesmo em catálogos com milhões de itens.

##  Tecnologias Utilizadas
- **Rust** (linguagem principal)
- **HashMap** (estrutura de dados para indexação)
- **petgraph** (biblioteca para manipulação de grafos)
- **serde / serde_json** (serialização e persistência de dados)
- **cargo test** (framework de testes nativo do Rust)

## ▶ Como Executar
Clone o repositório e compile o projeto:
```bash
git clone https://github.com/marcosamambaia/RustMegaStore.git
cd megastore_search
cargo run
