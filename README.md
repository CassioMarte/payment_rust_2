# 💳 Payment Rust 2 — Um Projeto de Estudo

> **Um guia prático para aprender Rust e Actix-web do zero**, construindo uma API de pagamentos simples com explicações em cada passo.

---

## 📚 O que é este projeto?

Este é um **projeto educacional** desenhado para iniciantes em Rust que desejam aprender:

- ✅ **Fundamentos de Rust**: sintaxe, tipos, traits, tratamento de erros
- ✅ **Actix-web**: framework web assíncrono de alta performance
- ✅ **Arquitetura limpa**: separação em camadas (Model → Repository → Service → Handler)
- ✅ **Banco de dados**: integração com PostgreSQL usando SQLx
- ✅ **Validação**: regras de negócio com a biblioteca `validator`
- ✅ **Testes**: testes de integração com `tokio::test`

**O código é simples, mas cada decisão é explicada**. Não é um projeto "production-ready", é um projeto **didático**. Cada README complementa o anterior com conceitos, justificativas e melhorias possíveis.

---

## 🗂️ Arquitetura dos READMEs

Os READMEs são progressivos — cada um constrói sobre o anterior. Comece do 1 e vá até o 7.2:

### **Fase 1: Setup do Projeto**

| README | Foco | O que você aprende |
|--------|------|-------------------|
| **[1-README.md](1-README.md)** | 🐳 Docker + Cargo | Como iniciar um projeto Rust com Docker, estrutura básica do Cargo.toml, primeiro `main.rs` com Actix-web e a macro `#[get]` |
| **[2-README.md](2-README.md)** | 🔗 Banco de dados | Docker Compose para PostgreSQL, pool de conexões, variáveis de ambiente (.env), conexão com SQLx |
| **[3-README.md](3-README.md)** | 📊 Schema do banco | Criação da tabela `payments` em SQL, Dockerfile com migrations, como estruturar um banco para APIs |

### **Fase 2: Arquitetura em Camadas**

| README | Foco | O que você aprende |
|--------|------|-------------------|
| **[4-README.md](4-README.md)** | 📦 Models | Structs de dados, derives (`Debug`, `Serialize`, `Deserialize`, `FromRow`), DTOs (NewPayment, UpdatePayment), enums |
| **[4.1-README.md](4.1-README.md)** | ✔️ Validação | Como implementar o trait `Validate`, regras condicionais, erros por campo, por que validar na entrada |
| **[4.2-README.md](4.2-README.md)** | *Futuro* | Aprofundamento em models (se necessário) |

### **Fase 3: Banco de Dados (Repository Pattern)**

| README | Foco | O que você aprende |
|--------|------|-------------------|
| **[5.1-README.md](5.1-README.md)** | 🗄️ Repository | Camada de acesso a dados, funções CRUD (`create`, `get_all`, `get_by_uuid`, `update`), SQLx queries, `query_as<>` vs `query_as!` |
| **[5.2-README.md](5.2-README.md)** | *Futuro* | Padrões avançados de Repository (se necessário) |

### **Fase 4: Lógica de Negócio (Service Layer)**

| README | Foco | O que você aprende |
|--------|------|-------------------|
| **[6.1-README.md](6.1-README.md)** | ⚙️ Service | Por que existem services, orquestração (validação → repository), tratamento de erros com `Result<T, Box<dyn Error>>` |
| **[6.2-README.md](6.2-README.md)** | *Futuro* | Lógica complexa e transações (se necessário) |

### **Fase 5: HTTP (Handler Layer) + Testes**

| README | Foco | O que você aprende |
|--------|------|-------------------|
| **[7.1-README.md](7.1-README.md)** | 🌐 Handlers | Rotas HTTP, extractors do Actix-web, respostas HTTP, tratamento de erros no handler |
| **[7.2-README.md](7.2-README.md)** | 🧪 Testes | Testes de integração com `#[tokio::test]`, setup de banco para testes, asserts, estrutura de testes |

---

## 🏗️ Estrutura do Projeto

```
payment_rust_2/
├── README.md                    ← Você está aqui!
├── 1-README.md                  ← Comece daqui
├── 2-README.md
├── ... (até 7.2-README.md)
│
├── Cargo.toml                   ← Dependências e configuração
├── .env.example                 ← Template de variáveis de ambiente
├── .env                         ← Seu .env local (não versionar)
│
├── docker/
│   ├── Dockerfile              ← Build da aplicação
│   ├── docker-compose.yml       ← PostgreSQL + App
│   └── init.sql                 ← Schema do banco
│
├── src/
│   ├── main.rs                  ← Entrada da aplicação, setup do servidor
│   │
│   ├── model/                   ← Structs de dados (o "o quê")
│   │   ├── mod.rs
│   │   └── payment_model.rs     ← Payment, NewPayment, PaymentStatus...
│   │
│   ├── validator/               ← Regras de validação
│   │   ├── mod.rs
│   │   ├── new_payment.rs       ← Validação de NewPayment
│   │   ├── updatePayment.rs
│   │   └── updatePaymentStatus.rs
│   │
│   ├── repository/              ← Banco de dados (o "como armazenar")
│   │   ├── mod.rs
│   │   └── payment_repository.rs ← CRUD com SQL
│   │
│   ├── service/                 ← Lógica de negócio (o "por que")
│   │   ├── mod.rs
│   │   └── payment_service.rs   ← Orquestra validação + repository
│   │
│   ├── handler/                 ← Rotas HTTP (o "como expor")
│   │   ├── mod.rs
│   │   └── payment_handler.rs   ← Endpoints GET/POST/PUT/PATCH
│   │
│   ├── routes/                  ← Registro de rotas
│   │   ├── mod.rs
│   │   └── payment_routes.rs    ← Mapeia URLs para handlers
│   │
│   └── tests/                   ← Testes de integração
│       ├── mod.rs
│       ├── common.rs            ← Setup compartilhado entre testes
│       ├── repository_test.rs   ← Testa camada de banco
│       ├── service_test.rs      ← Testa lógica de negócio
│       └── validator_test.rs    ← Testa validações
```

---

## 🎯 Como Usar Este Projeto para Aprender

### **Passo 1: Leia os READMEs em Ordem**
Cada README é auto-contido mas progressivo. **Não pule etapas**:
1. Comece com [1-README.md](1-README.md) — entenda o setup
2. Avance até [7.2-README.md](7.2-README.md) — finalize com testes

### **Passo 2: Entenda o Fluxo de uma Requisição**
Uma requisição POST `/payment` passa por:

```
Cliente HTTP
    ↓
[Handler] payment_handler.rs
    ↓ (extrai JSON e passa para Service)
[Service] payment_service.rs
    ↓ (valida com Validator + chama Repository)
[Validator] new_payment.rs
    ↓ (regras de negócio)
[Repository] payment_repository.rs
    ↓ (executa SQL)
PostgreSQL
    ↓
Response JSON
```

**Por que essa separação?** Cada camada tem responsabilidade única:
- **Model**: define a forma dos dados
- **Validator**: garante que dados a entrada estão corretas
- **Repository**: acessa o banco, sem lógica de negócio
- **Service**: orquestra e implementa regras (ex: "não refundar mais que pagou")
- **Handler**: conversa com HTTP, sem conhecer SQL

### **Passo 3: Modifique e Experimente**
- Tente adicionar um novo campo a `Payment`
- Crie uma nova validação
- Escreva um novo teste

Cada README tem seções "✨ Melhorias Possíveis" para desafiá-lo.

---

## 🚀 Quick Start

### Pré-requisitos
- Docker e Docker Compose instalados
- Rust (opcional, se rodar via Docker)

### Rodar o Projeto

```bash
# 1. Clone este repositório
git clone <url>
cd payment_rust_2

# 2. Crie seu .env
cp .env.example .env
# Edite .env com suas credenciais

# 3. Suba o banco de dados
docker compose -f docker/docker-compose.yml up -d

# 4. Rode a aplicação
cargo build
cargo run

# 5. Teste uma requisição
curl -X POST http://127.0.0.1:8080/payment \
  -H "Content-Type: application/json" \
  -d '{"amount": 100.0, "currency": "USD", "payment_method": "CreditCard", "payment_reason": "Test", "status": "Completed"}'
```

### Rodar Testes

```bash
# Configure a variável de ambiente para testes
export DATABASE_TEST_URL="postgresql://user:password@localhost:5432/payment_test_db"

# Execute os testes
cargo test
```

---

## 📖 Recursos Educacionais em Cada README

Cada README segue este padrão:

1. **Conceito**: O que você vai aprender e por que
2. **Código de Exemplo**: Exemplos práticos com comentários
3. **Explicação Linha por Linha**: Se é complexo, explicamos cada linha
4. **Justificativa**: Por que fazemos assim (não apenas "o como")
5. **✨ Melhorias Possíveis**: Próximos passos para aprofundar

Exemplo do [4.1-README.md](4.1-README.md):
```markdown
## Por que Validar na Entrada?

Se não validar, o banco recebe dados inválidos, histórico fica sujo,
e outros sistemas que consomem a API recebem lixo.

Validar NO HANDLER garante que nunca chega lixo no banco.
```

---

## 🎓 Conceitos-chave Explicados

Todos esses conceitos estão nos READMEs, mas em resumo:

| Conceito | Onde Aprender | Por que Importa |
|----------|---------------|-----------------|
| **Traits** | [4-README.md](4-README.md) | Reutilização de comportamento (Serialize, Deserialize, Validate) |
| **Result<T, E>** | [6.1-README.md](6.1-README.md) | Tratamento de erros sem exceções |
| **Async/Await** | [2-README.md](2-README.md) | APIs rápidas que usam threads eficientemente |
| **Extractors** | [6.1-README.md](6.1-README.md) | Actix injeta dados (JSON, Path, Pool) automaticamente |
| **Camadas** | Vários | Código testável, manutenível, escalável |

---

## 🤝 Contribuições & Sugestões

Este projeto é **para aprender**, então se quiser:
- ✅ Sugerir melhorias didáticas
- ✅ Avisar sobre erros nos READMEs
- ✅ Indicar tópicos faltando

Abra uma issue ou um PR. Vamos melhorar juntos!

---

## 📝 Licença

Este projeto é fornecido como material educacional. Sinta-se livre para usar, modificar e estudar.

---

## 🎬 Comece Agora!

👉 **[Vá para 1-README.md](1-README.md)** e comece sua jornada em Rust!

---

**Última atualização**: Abril 2026  
**Status**: Projeto em construção (Fase 5 em progresso)
