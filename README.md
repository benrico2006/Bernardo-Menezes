# RSA em Rust — Atividade Prática

> **Aviso**: Implementação **educacional** do RSA, sem padding (PKCS#1/OAEP) e sem proteções a side‑channels. **Não use em produção.**

## Como rodar

1. Instale Rust (rustup).
2. Na pasta do projeto, execute:

```bash
cargo run --release
```

Isso irá:
- Gerar um par de chaves RSA (por padrão 512 bits, altere para 2048+ se seu computador aguentar).
- Criptografar uma mensagem de exemplo.
- Descriptografar e imprimir o texto original.

## Estrutura do Código

- `src/main.rs`: geração de primos (Miller-Rabin), `modinv`, geração de chaves RSA, `encrypt` e `decrypt` usando `modpow`.
- `Cargo.toml`: dependências `num-bigint`, `rand`, `num-integer`, `num-traits` e `hex` (apenas para imprimir em hexadecimal).

## O que falta para uso real

- Padding seguro (OAEP).
- Tamanhos de chave >= 2048 bits e controles de tamanho de mensagem (fragmentação).
- Proteções contra ataques temporais e de canal lateral.
- Serialização segura das chaves (PEM) e testes mais robustos.

## Usar no GitHub Codespaces

1. Crie um repositório no GitHub e envie esta pasta.
2. Abra **Code → Codespaces → Create codespace on main**.
3. Aceite o prompt para usar o **Dev Container** (ou rode `Dev Containers: Rebuild Container`).
4. No terminal do Codespaces:
   ```bash
   cargo run --release
   ```
5. Preencha `REGISTRO_EXECUCAO.md` com a saída.

CI opcional (`.github/workflows/rust.yml`) já incluso.
