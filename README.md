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

- src/
  - main.rs # Execução principal do sistema
  - produto.rs # Estrutura de dados Produto
  - sistema_busca.rs # Lógica de busca e armazenamento
- tests/
  - testes_busca.rs # Testes automatizados


---

## ⚙️ Tecnologias Utilizadas

- 🦀 Rust
- 📦 Cargo (gerenciador de pacotes e build)
- 🧠 Estruturas de Dados (HashMap, Vetores)

---

## ▶️ Como Executar o Projeto

### 📌 Pré-requisitos

Antes de começar, você precisa ter instalado:
- Rust
- Cargo (já incluído na instalação do Rust)

Para verificar se está instalado corretamente:
rustc --version
cargo --version

📥 Clonar o repositório

▶️ Executar o projeto
cargo run

🧪 Executar os testes
cargo test

💡 O que esperar
Ao executar o projeto, o sistema irá:

Inserir automaticamente vários produtos
Realizar uma busca exata (ex: "Notebook Dell")
Realizar uma busca parcial (ex: "note")
Exibir os resultados no terminal
Mostrar o tempo de execução de cada busca

👩‍💻 Autora
Ágata Oliveira
