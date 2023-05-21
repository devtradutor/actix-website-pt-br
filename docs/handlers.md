---
title: Manipuladores
---
import CodeBlock from "@site/src/components/code_block.js";

# Manipuladores de Solicitação

Um manipulador de solicitação é uma função assíncrona que aceita zero ou mais parâmetros que podem ser extraídos de uma solicitação (ou seja, [_impl FromRequest_][implfromrequest]) e retorna um tipo que pode ser convertido em um HttpResponse (ou seja, [_impl Responder_][respondertrait]).

O processamento da solicitação ocorre em duas etapas. Primeiro, o objeto manipulador é chamado, retornando qualquer objeto que implemente o [_trait Responder_][respondertrait]. Em seguida, `respond_to()` é chamado no objeto retornado, convertendo-o em um `HttpResponse` ou `Error`.

Por padrão, o Actix Web fornece implementações de `Responder` para alguns tipos padrão, como `&'static str`, `String`, etc.

> Para uma lista completa de implementações, consulte a [_documentação do Responder_][responderimpls].

Aqui estão alguns exemplos de manipuladores válidos:

```rust
async fn index(_req: HttpRequest) -> &'static str {
    "Olá, mundo!"
}
```

```rust
async fn index(_req: HttpRequest) -> String {
    "Olá, mundo!".to_owned()
}
```

Você também pode alterar a assinatura para retornar `impl Responder`, o que funciona bem se tipos mais complexos estiverem envolvidos.

```rust
async fn index(_req: HttpRequest) -> impl Responder {
    web::Bytes::from_static(b"Olá, mundo!")
}
```

```rust
async fn index(req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    ...
}
```

## Resposta com tipo personalizado

Para retornar um tipo personalizado diretamente de uma função manipuladora, o tipo precisa implementar o trait `Responder`.

Vamos criar uma resposta para um tipo personalizado que é serializado para uma resposta `application/json`:

<CodeBlock example="responder-trait" file="main.rs" section="responder-trait" />

## Transmissão do corpo da resposta

O corpo da resposta pode ser gerado de forma assíncrona. Nesse caso, o corpo precisa implementar o trait de stream `Stream<Item=Bytes, Error=Error>`, ou seja:

<CodeBlock example="async-handlers" file="stream.rs" section="stream" />

## Tipos de retorno diferentes (Either)

Às vezes, você precisa retornar diferentes tipos de respostas. Por exemplo, você pode verificar erros e retorná-los, retornar respostas assíncronas ou qualquer resultado que exija dois tipos diferentes.

Para esse caso, o tipo [Either][either] pode ser usado. `Either` permite combinar dois tipos de resposta diferentes em um único tipo.

<CodeBlock example="either" file="main.rs" section="either" />

[implfromrequest]: https://docs.rs/actix-web/4/actix_web/trait.FromRequest.html
[respondertrait]: https://docs.rs/actix-web/4/actix_web/trait.Responder.html
[responderimpls]: https://docs.rs/actix-web/4/actix_web/trait.Responder.html#foreign-impls
[either]: https://docs.rs/actix-web/4/actix_web/enum.Either.html