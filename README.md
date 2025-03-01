# PrimeSocket

PrimeSocket é uma biblioteca Python com *bindings* para um servidor e cliente UDP escritos em Rust.
Ela é projetada para processar computação de números primos de maneira eficiente e assíncrona.

## Tabela de Conteúdos

- [Recursos](#recursos)
- [Instalação](#instalação)
  - [Instalação local](#instalação-local)
  - [Compilação do Rust (opcional)](#compilação-do-rust-opcional)
- [Uso](#uso)
  - [Executando o Servidor](#executando-o-servidor)
  - [Executando o Cliente](#executando-o-cliente)
- [Desenvolvimento](#desenvolvimento)
  - [Como Contribuir](#como-contribuir)
  - [Testes](#testes)
- [Licença](#licença)

---

## Recursos

- **Servidor UDP**: Processa solicitações de clientes e gerencia computações de primos.
- **Cliente UDP**: Conecta-se ao servidor e solicita blocos de dados para cálculos do *sieve*.
- **Desempenho**: Core do servidor e cliente implementados em Rust para maior eficiência.
- **Fácil Integração**: Disponível como um pacote Python instalável.
- **Verbose (opcional)**: Exibe logs detalhados para depuração.

---

## Instalação

### Instalação local

Para instalar a biblioteca localmente:

```sh
pip install .
```

Ou para instalar as dependências:

```sh
pip install -r requirements.txt
```

#### Compilação do Rust (opcional)

Se estiver desenvolvendo e precisar recompilar o *core* em Rust, instale com [maturin](https://github.com/PyO3/maturin):

```sh
maturin develop
```

> **Nota**: Se ocorrer algum erro relacionado ao `SocketCore`, faça o download da versão pré-compilada do [GitHub Releases](https://github.com/Joao-vpf/primesocket/releases) e instale manualmente.

---

## Uso

### Executando o Servidor

Exemplo em Python:

```python
from primesocket import PrimeServer

server = PrimeServer(port=8080, end=1000, verbose=1)
server.start()
```

Ou via terminal:

```sh
primesocket-server --port 8080 --end 1000 --verbose 1
```

### Executando o Cliente

Exemplo em Python:

```python
from primesocket import PrimeClient

client = PrimeClient(ip="127.0.0.1", port=8080, verbose=1, timeout=120)
client.start()
```

Ou via terminal:

```sh
primesocket-client --ip 127.0.0.1 --port 8080 --verbose 1 --timeout 120
```

---

## Desenvolvimento

### Como Contribuir

1. Clone o repositório:
   ```sh
   git clone https://github.com/seuusuario/primesocket.git
   cd primesocket
   ```

2. Instale as dependências:
   ```sh
   pip install -r requirements.txt
   ```

3. Compile a biblioteca Rust (opcional):
   ```sh
   maturin develop
   ```

### Testes

Para executar os testes:

```sh
pytest tests/
```

---

## Licença

Este projeto é licenciado sob a [Licença MIT](LICENSE). Sinta-se à vontade para usar, modificar e distribuir.