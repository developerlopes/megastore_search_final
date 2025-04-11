# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto
Este projeto implementa um sistema de busca eficiente para o catálogo de produtos da MegaStore. O objetivo é proporcionar uma pesquisa rápida, precisa e escalável utilizando estruturas de dados otimizadas e técnicas de pré-processamento. A aplicação é capaz de localizar produtos a partir de consultas simples ou compostas, permitindo uma navegação eficaz em grandes volumes de dados.

## Tecnologias Utilizadas
- **Linguagem**: Rust
- **Crates**:
  - `regex` – para manipulação de expressões regulares durante o pré-processamento dos textos
  - `lazy_static` – para facilitar a inicialização de estruturas globais
- **Ferramentas de Teste**: Framework nativo de testes do Rust (`cargo test`)

## Como Executar o Sistema de Busca
1. Clone ou baixe o repositório
```bash
cd megastore_search_final
```
2. Compile o projeto:
```bash
cargo build
```
3. Execute o projeto:
```bash
cargo run
```

## Como Executar os Testes
1. Utilize o comando abaixo na raiz do projeto:
```bash
cargo test
```

## Exemplos de Uso!!
```bash
Digite sua consulta: martelo
Resultados encontrados:
- Martelo de Borracha
- Martelo Unha

Digite sua consulta: martelo cabo de madeira
Resultados encontrados:
- Martelo de Borracha
```

## Arquitetura do Sistema
O projeto está estruturado da seguinte forma:
- `main.rs`: Ponto de entrada da aplicação, responsável pela interface de linha de comando.
- `lib.rs`: Contém os módulos centrais de lógica:
  - `preprocessing`: Realiza a limpeza e tokenização dos textos
  - `search`: Contém a lógica de busca utilizando tabelas hash
- `testes.rs`: Implementa testes unitários e de integração para os principais componentes

## Algoritmos e Estruturas de Dados Utilizados
- **Pré-processamento**: Tokenização com remoção de pontuação, conversão para minúsculas e normalização dos dados
- **Índice invertido com HashMap**: Utilização de `HashMap<String, Vec<usize>>` para mapear palavras para índices de produtos
- **Interseção de vetores**: Para realizar a filtragem de múltiplas palavras na busca

## Considerações sobre Desempenho e Escalabilidade
- O uso de `HashMap` permite buscas em tempo constante para cada termo individual
- A interseção entre vetores de índices permite uma filtragem eficiente para múltiplos termos
- O sistema foi testado com centenas de produtos mantendo tempos de resposta abaixo de 50ms

## Contribuições
Este é um projeto acadêmico e não está aberto para contribuições externas neste momento. Futuramente, poderá ser estendido com interface gráfica ou API web.

## Licença
Este projeto está licenciado sob a Licença MIT.
