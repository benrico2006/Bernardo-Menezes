## Mini‑aula: Criptografia RSA (versão expressa e didática)

**Ideia central**: usar aritmética modular com números gigantes de forma que seja fácil **multiplicar** dois primos grandes, mas **muito difícil** fatorar o produto.

### Passo a passo matemático
1. **Escolha de primos**: gere dois primos grandes `p` e `q` (no projeto, ~bits/2 cada).
2. **Módulo**: `n = p * q`.
3. **Totiente**: `φ(n) = (p − 1)(q − 1)` (porque `p` e `q` são primos).
4. **Expoente público**: escolha `e` coprimo de `φ(n)` (comum: `65537`).
5. **Chave privada**: calcule `d` tal que `e * d ≡ 1 (mod φ(n))` (inverso modular).
6. **Chave pública**: `(n, e)`; **chave privada**: `(n, d)`.

### Criptografar e Descriptografar
- **Criptografia**: `c = m^e mod n`.
- **Decriptação**: `m = c^d mod n`.
(onde `m` é a mensagem como inteiro; no código, usamos bytes ↔ inteiros).

### Por que funciona?
Porque `d` é o inverso de `e` módulo `φ(n)`, então `m^(ed) ≡ m (mod n)`
(pelo Pequeno Teorema de Fermat / Teorema de Euler), desde que `m < n` e coprimo a `n`.

### Boas práticas (produção)
- Use chaves de **2048+ bits**.
- **Nunca** use “RSA cru”: aplique **padding** seguro (ex.: OAEP).
- Proteja a implementação de **side‑channels** (tempo, cache).
- Prefira bibliotecas bem testadas (ex.: *rsa*, *ring*).

### Aplicações
- Troca de segredos (ex.: troca de chaves em TLS — hoje em dia mais comum usar DH/ECDH).
- Assinaturas digitais (com esquemas de padding próprios, ex.: PSS).
