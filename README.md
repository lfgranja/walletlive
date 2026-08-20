# WalletLive 🚀 - Carteira de Investimentos Fullstack

Este projeto é a evolução do desafio final do Bootcamp de Rust da Digital Innovation One (DIO) em parceria com o Santander. Trata-se de uma aplicação Fullstack desenvolvida em Rust para cadastro, acompanhamento e gestão de ativos de investimento.

## 🛠️ Tecnologias Utilizadas

- **Backend:** Rust, Axum, Tokio
- **Banco de Dados:** PostgreSQL operado via SQLx
- **Frontend / Views:** Askama (SSR Templates) + HTML/CSS
- **Autenticação:** JWT (JSON Web Tokens) e controle de Cookies
- **Infraestrutura:** Docker & Docker Compose
- **Engenharia e Qualidade (MDMV Standard):**
  - Git Hooks com **Lefthook**
  - Atomic Conventional Commits com **Cocogitto**
  - CI/CD automatizado via **GitHub Actions**
  - Automação de Release e Changelog via **Release-plz**

## 💡 Melhorias Implementadas (Evolução do Projeto)

Além da base exigida pelo bootcamp, implementei as seguintes melhorias focadas em Engenharia de Software e Valor ao Usuário:

1. **Cálculo de Patrimônio Consolidado:** Implementação do cálculo funcional iterativo (`.iter().map().sum()`) no backend para calcular o valor total da carteira do usuário com base nos ativos registrados, formatado e injetado diretamente na View com Askama.
2. **Pipeline CI/CD Completo:** Integração contínua que roda checagens rigorosas de `cargo clippy`, formatação e testes antes de qualquer *merge*.
3. **Automação de Release:** Configuração do `release-plz` para leitura semântica de commits e geração automática de Changelogs e novas versões do projeto.
4. **Frase Histórica Aleatória ("Do Giz à Ferrugem"):** A cada carregamento do Dashboard, uma curiosidade histórica sobre o mercado financeiro é sorteada de um banco de frases embutido no backend e exibida ao usuário, utilizando `rand::seq::SliceRandom` para seleção aleatória via Server-Side Rendering com Askama.

## ⚙️ Como Executar a Aplicação

1. Clone o repositório: `git clone https://github.com/lfgranja/walletlive.git`
2. Suba o Banco de Dados com Docker: `docker compose up -d`
3. Copie o arquivo de ambiente: `cp .env.example .env`
4. Execute as migrações (se aplicável) e rode o projeto: `cargo run`
5. Acesse no navegador: `http://localhost:8080`

## 🧪 Como Testar

Os testes automatizados cobrem as regras de negócio e validações da aplicação.
Para rodar a suíte de testes completa, incluindo os hooks locais:

```bash
cargo test
```

## 🧠 O Que Aprendi

Este desafio consolidou minha capacidade de estruturar uma arquitetura completa (Modular Monolith) em Rust. Compreendi profundamente o fluxo de ownership, o roteamento rápido do Axum e como macros como o Askama e SQLx garantem segurança em tempo de compilação, eliminando bugs antes mesmo do código rodar em produção.
