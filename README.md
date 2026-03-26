# 🛒 Sistema de Busca de Produtos em Rust

Este projeto foi desenvolvido com o objetivo de aplicar conceitos de **Estruturas de Dados** e **otimização de buscas**, utilizando a linguagem **Rust**.

A aplicação simula um sistema de busca de produtos, permitindo comparar **busca exata (mais eficiente)** e **busca parcial**, além de medir o tempo de execução de cada operação.

---

## 🚀 Funcionalidades

- 🔎 **Busca exata por nome**
  - Utiliza `HashMap` para acesso rápido (O(1))

- 🔍 **Busca parcial por nome**
  - Percorre os dados para encontrar correspondências

- ⏱ **Medição de tempo de execução**
  - Permite análise de performance entre os tipos de busca

- 🧪 **Testes automatizados**
  - Garantem o funcionamento correto das funcionalidades

---

## 🧱 Estrutura do Projeto

src/
├── main.rs # Execução principal do sistema
├── produto.rs # Estrutura de dados Produto
├── sistema_busca.rs # Lógica de busca e armazenamento
tests/
├── testes_busca.rs # Testes automatizados


---

## ⚙️ Tecnologias Utilizadas

- 🦀 Rust
- 📦 Cargo (gerenciador de pacotes e build)
- 🧠 Estruturas de Dados (HashMap, Vetores)

---

## ▶️ Como Executar o Projeto

Clone o repositório:

```bash
git clone https://github.com/SEU-USUARIO/NOME-DO-REPO.git
cd NOME-DO-REPO

Execute o sistema:

cargo run
🧪 Como Executar os Testes
cargo test
📊 Exemplo de Uso

O sistema permite realizar buscas como:

🔎 Busca exata:
Notebook Dell
🔍 Busca parcial:
note → retorna múltiplos produtos

Além disso, o tempo de execução de cada busca é exibido no terminal, permitindo comparação de desempenho.

💡 Objetivo do Projeto

Demonstrar, na prática, como a escolha da estrutura de dados impacta diretamente na performance de um sistema, além de aplicar boas práticas de organização e testes em Rust.

🚀 Possíveis Melhorias
Interface gráfica ou web
Filtros avançados (preço, categoria, marca)
Persistência de dados (arquivo ou banco de dados)
Transformação em API

👩‍💻 Autora
Ágata Oliveira