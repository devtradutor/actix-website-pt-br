---
title: Middleware
---

import CodeBlock from "@site/src/components/code_block.js";

# Middleware

O sistema de middleware do Actix Web nos permite adicionar comportamentos adicionais ao processamento de requisições/respostas. O middleware pode interagir com o processo de uma requisição recebida, permitindo modificar as requisições e interromper o processamento para retornar uma resposta antecipada.

O middleware também pode interagir com o processamento de respostas.

Tipicamente, o middleware está envolvido nas seguintes ações:

- Pré-processar a requisição
- Pós-processar uma resposta
- Modificar o estado da aplicação
- Acessar serviços externos (redis, registro, sessões)

O middleware é registrado para cada `App`, `scope` ou `Resource` e é executado em ordem oposta ao registro. Em geral, um _middleware_ é um tipo que implementa o [_Service trait_][servicetrait] e [_Transform trait_][transformtrait]. Cada método nos traits tem uma implementação padrão. Cada método pode retornar um resultado imediatamente ou um objeto _futuro_.

A seguir, demonstra-se a criação de um middleware simples:

<CodeBlock example="middleware" file="main.rs" section="simple" />

Alternativamente, para casos de uso simples, você pode usar [_wrap_fn_][wrap_fn] para criar middlewares pequenos e ad hoc:

<CodeBlock example="middleware" file="wrap_fn.rs" section="wrap-fn" />

> O Actix Web fornece vários middlewares úteis, como _logging_, _user sessions_, _compress_, etc.

**Aviso: se você usar `wrap()` ou `wrap_fn()` várias vezes, a última ocorrência será executada primeiro.**

## Logging

O registro é implementado como um middleware. É comum registrar um middleware de registro como o primeiro middleware para a aplicação. O middleware de registro deve ser registrado para cada aplicação.

O middleware `Logger` usa a biblioteca padrão de registro para registrar informações. Você deve habilitar o registro para o pacote _actix_web_ para ver o log de acesso ([env_logger][envlogger] ou similar).

### Uso

Crie um middleware `Logger` com o `format` especificado. O `Logger` padrão pode ser criado com o método `default`, que usa o formato padrão:

```ignore
  %a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
```

<CodeBlock example="middleware" file="logger.rs" section="logger" />

A seguir, temos um exemplo do formato padrão de registro:

```
INFO:actix_web::middleware::logger: 127.0.0.1:59934 [02/Dec/2017:00:21:43 -0800] "GET / HTTP/1.1" 302 0 "-" "curl/7.54.0" 0.000397
INFO:actix_web::middleware::logger: 127.0.0.1:59947 [02/Dec/2017:00:22:40 -0800] "GET /index.html HTTP/1.1" 200 0 "-" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.13; rv:57.0) Gecko/20100101 Firefox/57.0" 0.000646
```

### Formato

- `%%` O sinal de porcentagem
- `%a` Endereço IP remoto (endereço IP do proxy se estiver usando um proxy reverso)
- `%t` Hora em que a requisição começou a ser processada
- `%P` O ID do processo filho que atendeu à requisição
- `%r` Primeira linha da requisição
- `%s` Código de status da resposta
- `%b` Tamanho da resposta em bytes, incluindo os cabeçalhos HTTP
- `%T` Tempo decorrido para atender à requisição, em segundos, com fração decimal no formato .06f
- `%D` Tempo decorrido para atender à requisição, em milissegundos
- `%{FOO}i` request.headers['FOO']
- `%{FOO}o` response.headers['FOO']
- `%{FOO}e` os.environ['FOO']

## Cabeçalhos padrão

Para definir cabeçalhos de resposta padrão, o middleware `DefaultHeaders` pode ser usado. O middleware _DefaultHeaders_ não define o cabeçalho se os cabeçalhos da resposta já contiverem um cabeçalho especificado.

<CodeBlock example="middleware" file="default_headers.rs" section="default-headers" />

## Sessões de usuário

O Actix Web fornece uma solução geral para gerenciamento de sessões. O middleware [**actix-session**][actixsession] pode usar vários tipos de backend para armazenar dados de sessão.

> Por padrão, apenas o backend de sessão de cookie está implementado. Outras implementações de backend podem ser adicionadas.

[**CookieSession**][cookiesession] usa cookies como armazenamento de sessão. `CookieSessionBackend` cria sessões que estão limitadas a armazenar menos de 4000 bytes de dados, pois o payload deve caber em um único cookie. Um erro interno do servidor é gerado se uma sessão contiver mais de 4000 bytes.

Um cookie pode ter uma política de segurança _signed_ (assinado) ou _private_ (privado). Cada um tem um respectivo construtor `CookieSession`.

Um cookie _signed_ pode ser visualizado, mas não modificado pelo cliente. Um cookie _private_ (privado) não pode ser visualizado nem modificado pelo cliente.

Os construtores recebem uma chave como argumento. Esta é a chave privada para a sessão de cookie - quando esse valor é alterado, todos os dados da sessão são perdidos.

Em geral, você cria um middleware `SessionStorage` e o inicializa com uma implementação de backend específica, como `CookieSession`. Para acessar os dados da sessão, você deve usar o extractor [`Session`][requestsession]. Este método retorna um objeto [_Session_][sessionobj], que nos permite obter ou definir dados da sessão.
> `actix_session::storage::CookieSessionStore` está disponível na funcionalidade "cookie-session" do crate.

<CodeBlock example="middleware" file="user_sessions.rs" section="user-session" />

## Manipuladores de erros

O middleware `ErrorHandlers` nos permite fornecer manipuladores personalizados para respostas de erro.

Você pode usar o método `ErrorHandlers::handler()` para registrar um manipulador de erro personalizado para um código de status específico. Você pode modificar uma resposta existente ou criar uma completamente nova. O manipulador de erro pode retornar uma resposta imediatamente ou retornar um futuro que se resolve em uma resposta.
<CodeBlock example="middleware" file="errorhandler.rs" section="error-handler" />

[sessionobj]: https://docs.rs/actix-session/0.7/actix_session/struct.Session.html
[requestsession]: https://docs.rs/actix-session/0.7/actix_session/struct.Session.html
[cookiesession]: https://docs.rs/actix-session/0.7/actix_session/storage/struct.CookieSessionStore.html
[actixsession]: https://docs.rs/actix-session/0.7/actix_session/
[envlogger]: https://docs.rs/env_logger/*/env_logger/
[servicetrait]: https://docs.rs/actix-web/4/actix_web/dev/trait.Service.html
[transformtrait]: https://docs.rs/actix-web/4/actix_web/dev/trait.Transform.html
[wrap_fn]: https://docs.rs/actix-web/4/actix_web/struct.App.html#method.wrap_fn
