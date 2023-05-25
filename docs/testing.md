---
title: Testing
---

import CodeBlock from "@site/src/components/code_block.js";

# Testando

Toda aplicação deve ser bem testada. O Actix Web fornece ferramentas para realizar testes unitários e de integração.

## Testes Unitários

Para testes unitários, o actix-web fornece um tipo de construtor de requisição. [_TestRequest_][testrequest] implementa um padrão semelhante a um construtor. Você pode gerar uma instância de `HttpRequest` com `to_http_request()` e chamá-la no seu manipulador.

<CodeBlock example="testing" file="main.rs" section="unit-tests" />

## Testes de Integração

Existem alguns métodos para testar sua aplicação. O Actix Web pode ser usado para executar a aplicação com manipuladores específicos em um servidor HTTP real.

`TestRequest::get()`, `TestRequest::post()` e outros métodos podem ser usados para enviar requisições para o servidor de teste.

Para criar um `Service` para teste, use o método `test::init_service`, que aceita um construtor `App` regular.

> Consulte a [documentação da API][actixdocs] para obter mais informações.

<CodeBlock example="testing" file="integration_one.rs" section="integration-one" />

Se você precisar de uma configuração de aplicativo mais complexa, o teste deve ser muito semelhante à criação da aplicação normal. Por exemplo, você pode precisar inicializar o estado da aplicação. Crie um `App` com um método `data` e anexe o estado da mesma forma que faria em uma aplicação normal.

<CodeBlock example="testing" file="integration_two.rs" section="integration-two" />

## Testes de Resposta de Stream

Se você precisar testar a geração de stream, será suficiente chamar [`into_parts()`][resintoparts] e converter o corpo resultante em um futuro e executá-lo, por exemplo, ao testar [_Server Sent Events_][serversentevents].

<CodeBlock example="testing" file="stream_response.rs" section="stream-response" />

[serversentevents]: https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events
[resintoparts]: https://docs.rs/actix-web/4/actix_web/struct.HttpResponse.html#method.into_parts
[actixdocs]: https://docs.rs/actix-web/4/actix_web/test/index.html
[testrequest]: https://docs.rs/actix-web/4/actix_web/test/struct.TestRequest.html
