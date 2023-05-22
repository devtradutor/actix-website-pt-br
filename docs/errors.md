---
title: Erros
---


import CodeBlock from "@site/src/components/code_block.js";

# Erros

O Actix Web usa seu próprio tipo [`actix_web::error::Error`][actixerror] e a trait [`actix_web::error::ResponseError`][responseerror] para tratamento de erros em manipuladores da web.

Se um manipulador retornar um `Error` (referindo-se ao [trait geral Rust `std::error::Error`][stderror]) em um `Result` que também implementa o trait `ResponseError`, o actix-web renderizará esse erro como uma resposta HTTP com seu [`actix_web::http::StatusCode`][status_code] correspondente. Um erro interno do servidor é gerado por padrão:

```rust
pub trait ResponseError {
    fn error_response(&self) -> Response<Body>;
    fn status_code(&self) -> StatusCode;
}
```

Um `Responder` converte `Result`s compatíveis em respostas HTTP:

```rust
impl<T: Responder, E: Into<Error>> Responder for Result<T, E>
```

O `Error` no código acima é a definição de erro do actix-web, e quaisquer erros que implementem `ResponseError` podem ser convertidos automaticamente para ele.

O Actix Web fornece implementações de `ResponseError` para alguns erros comuns não relacionados ao actix. Por exemplo, se um manipulador responder com um `io::Error`, esse erro será convertido em um `HttpInternalServerError`:

```rust
use std::io;
use actix_files::NamedFile;

fn index(_req: HttpRequest) -> io::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}
```

Consulte [a documentação da API do actix-web][responseerrorimpls] para obter uma lista completa de implementações externas para `ResponseError`.

## Um exemplo de resposta de erro personalizado

Aqui está um exemplo de implementação para `ResponseError`, usando a biblioteca [derive_more] para enums declarativos de erro.

<CodeBlock example="errors" file="main.rs" section="response-error" />

`ResponseError` possui uma implementação padrão para `error_response()` que renderizará um _500_ (erro interno do servidor), e isso é o que acontecerá quando o manipulador `index` for executado acima.

Substitua `error_response()` para produzir resultados mais úteis:

<CodeBlock example="errors" file="override_error.rs" section="override" />

## Auxiliares de erro

O Actix Web fornece um conjunto de funções auxiliares de erro que são úteis para gerar códigos de erro HTTP específicos a partir de outros erros. Aqui, convertemos `MyError`, que não implementa o trait `ResponseError`, em um _400_ (solicitação inválida) usando `map_err`:

<CodeBlock example="errors" file="helpers.rs" section="helpers" />

Consulte a [documentação da API do módulo `error` do actix-web][actixerror] para obter uma lista completa de auxiliares de erro disponíveis.

## Registro de erros

O Actix registra todos os erros no nível de log `WARN`. Se o nível de log de um aplicativo estiver definido como `DEBUG` e `RUST_BACKTRACE` estiver ativado, a rastreabilidade também será registrada. Isso pode ser configurado com variáveis ambientais:

```
>> RUST_BACKTRACE=1 RUST_LOG=actix_web=debug cargo run
```

O tipo `Error` usa o rastreio de erro da causa, se disponível. Se a falha subjacente não fornecer um rastreio, um novo rastreio será construído apontando para o ponto em que a conversão ocorreu (em vez da origem do erro).

## Práticas recomendadas no tratamento de erros

Pode ser útil pensar em dividir os erros produzidos por um aplicativo em dois grupos principais: aqueles destinados aos usuários e aqueles que não são.

Um exemplo do primeiro é que posso usar `failure` para especificar um enum `UserError` que encapsula um `ValidationError` para retornar sempre que um usuário enviar uma entrada incorreta:

<CodeBlock example="errors" file="recommend_one.rs" section="recommend-one" />

Isso funcionará exatamente como pretendido, porque a mensagem de erro definida com `display` é escrita com a intenção explícita de ser lida por um usuário.

No entanto, enviar de volta a mensagem de erro não é desejável para todos os erros -- há muitas falhas que ocorrem em um ambiente de servidor onde provavelmente gostaríamos que os detalhes fossem ocultados do usuário. Por exemplo, se um banco de dados falhar e as bibliotecas de cliente começarem a produzir erros de tempo limite de conexão, ou se um modelo HTML estiver formatado incorretamente e apresentar erros ao ser renderizado. Nesses casos, pode ser preferível mapear os erros para um erro genérico adequado para consumo do usuário.

Aqui está um exemplo que mapeia um erro interno para um `InternalError` voltado para o usuário, com uma mensagem personalizada:

<CodeBlock example="errors" file="recommend_two.rs" section="recommend-two" />

Ao dividir os erros em aqueles que são voltados para o usuário e aqueles que não são, podemos garantir que não exponhamos acidentalmente os usuários a erros lançados por partes internas do aplicativo que eles não deveriam ver.

## Registro de erros

Este é um exemplo básico usando `middleware::Logger`, que depende de `env_logger` e `log`:

```toml
[dependencies]
env_logger = "0.8"
log = "0.4"
```

<CodeBlock example="errors" file="logging.rs" section="logging" />

[actixerror]: https://docs.rs/actix-web/4/actix_web/error/struct.Error.html
[errorhelpers]: https://docs.rs/actix-web/4/actix_web/trait.ResponseError.html
[derive_more]: https://crates.io/crates/derive_more
[responseerror]: https://docs.rs/actix-web/4/actix_web/error/trait.ResponseError.html
[responseerrorimpls]: https://docs.rs/actix-web/4/actix_web/error/trait.ResponseError.html#foreign-impls
[stderror]: https://doc.rust-lang.org/std/error/trait.Error.html
[status_code]: https://docs.rs/actix-web/4/actix_web/http/struct.StatusCode.html

