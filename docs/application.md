---
title: Aplicação
---

import CodeBlock from "@site/src/components/code_block.js";

## Escrevendo um Aplicativo
`actix-web` fornece várias primitivas para construir servidores e aplicativos da web com Rust. Ele oferece roteamento, middleware, pré-processamento de solicitações, pós-processamento de respostas, etc.

Todos os servidores `actix-web` são construídos em torno da instância [`App`][app]. É usada para registrar rotas para recursos e middleware. Também armazena o estado do aplicativo compartilhado entre todos os manipuladores dentro do mesmo escopo.

O [`scope`][scope] de um aplicativo atua como um espaço para todos os caminhos de rota, ou seja, todas as rotas para um escopo específico de aplicativo têm o mesmo prefixo de caminho de URL. O prefixo do aplicativo sempre contém uma barra "/" inicial. Se um prefixo fornecido não contiver uma barra inicial, ela será inserida automaticamente. O prefixo deve consistir em segmentos de caminho de valor.

> Para um aplicativo com escopo `/app`, qualquer solicitação com os caminhos `/app`, `/app/` ou `/app/test` corresponderia; no entanto, o caminho `/application` não corresponderia.

<CodeBlock example="application" file="app.rs" section="setup" />

Neste exemplo, um aplicativo com o prefixo `/app` e um recurso `index.html` é criado. Este recurso está disponível através do URL `/app/index.html`.

> Para mais informações, consulte a seção [URL Dispatch][usingappprefix].

## Estado

O estado do aplicativo é compartilhado com todas as rotas e recursos dentro do mesmo escopo. O estado pode ser acessado usando o extrator [`web::Data<T>`][data], onde `T` é o tipo do estado. O estado também é acessível para o middleware.

Vamos escrever um aplicativo simples e armazenar o nome do aplicativo no estado:

<CodeBlock example="application" file="state.rs" section="setup" />

Em seguida, passe o estado ao inicializar o `App` e iniciar o aplicativo:

<CodeBlock example="application" file="state.rs" section="start_app" />

Qualquer número de tipos de estado pode ser registrado dentro do aplicativo.

## Estado Compartilhado Mutável {#shared-mutable-state}

O `HttpServer` aceita uma fábrica de aplicativos em vez de uma instância de aplicativo. Um `HttpServer` constrói uma instância de aplicativo para cada thread. Portanto, os dados do aplicativo devem ser construídos várias vezes. Se você deseja compartilhar dados entre diferentes threads, um objeto compartilhável deve ser usado, por exemplo, `Send` + `Sync`.

Internamente, [`web::Data`][data] usa `Arc`. Portanto, para evitar a criação de dois `Arc`s, devemos criar nossos dados antes de registrá-los usando [`App::app_data()`][appdata].

No exemplo a seguir, escreveremos um aplicativo com estado compartilhado mutável. Primeiro, definimos nosso estado e criamos nosso manipulador:

<CodeBlock example="application" file="mutable_state.rs" section="setup_mutable" />

e registramos os dados em um `App`:

<CodeBlock example="application" file="mutable_state.rs" section="make_app_mutable" />

Principais pontos a serem observados:
- O estado inicializado _dentro_ do fechamento passado para `HttpServer::new` é local para a thread do trabalhador e pode ficar dessincronizado se for modificado.
- Para obter um estado _globalmente compartilhado_, ele deve ser criado **fora**(outside) do fechamento passado para `HttpServer::new` e movido/clonado.

## Usando um Escopo de Aplicativo para Compor Aplicativos

O método [`web::scope()`][webscope] permite definir um prefixo de grupo de recursos. Esse escopo representa um prefixo de recurso que será adicionado a todos os padrões de recurso adicionados pela configuração de recursos. Isso pode ser usado para montar um conjunto de rotas em uma localização diferente da pretendida pelo autor original, mantendo os mesmos nomes de recursos.

Por exemplo:

<CodeBlock example="application" file="scope.rs" section="scope" />

No exemplo acima, a rota `show_users` terá um padrão de rota efetivo de `/users/show` em vez de `/show`, porque o argumento de escopo do aplicativo será prefixado ao padrão. A rota só será correspondida se o caminho URL for `/users/show`, e quando a função [`HttpRequest.url_for()`][urlfor] for chamada com o nome da rota `show_users`, ela gerará um URL com o mesmo caminho.

## Guardas de Aplicativo e hospedagem virtual

Você pode pensar em uma guarda como uma função simples que aceita uma referência a um objeto de _request_ e retorna _true_ ou _false_. Formalmente, uma guarda é qualquer objeto que implementa a trait [`Guard`][guardtrait]. O Actix Web fornece várias guardas. Você pode verificar a seção [funções][guardfuncs] da documentação da API.

Uma das guardas fornecidas é [`Host`][guardheader]. Ela pode ser usada como um filtro com base nas informações do cabeçalho da solicitação.

<CodeBlock example="application" file="vh.rs" section="vh" />

## Configuração

Para simplicidade e reutilização, tanto [`App`][appconfig] quanto [`web::Scope`][webscopeconfig] fornecem o método `configure`. Essa função é útil para mover partes da configuração para um módulo diferente ou até mesmo uma biblioteca. Por exemplo, parte da configuração do recurso pode ser movida para um módulo diferente.

<CodeBlock example="application" file="config.rs" section="config" />

O resultado do exemplo acima seria:

```
/         -> "/"
/app      -> "app"
/api/test -> "test"
```

Cada [`ServiceConfig`][serviceconfig] pode ter seus próprios `data`, `routes` e `services`.

<!-- LINKS -->

[usingappprefix]: /docs/url-dispatch/index.html#using-an-application-prefix-to-compose-applications
[stateexample]: https://github.com/actix/examples/blob/master/basics/state/src/main.rs
[guardtrait]: https://docs.rs/actix-web/4/actix_web/guard/trait.Guard.html
[guardfuncs]: https://docs.rs/actix-web/4/actix_web/guard/index.html#functions
[guardheader]: https://docs.rs/actix-web/4/actix_web/guard/fn.Header.html
[data]: https://docs.rs/actix-web/4/actix_web/web/struct.Data.html
[app]: https://docs.rs/actix-web/4/actix_web/struct.App.html
[appconfig]: https://docs.rs/actix-web/4/actix_web/struct.App.html#method.configure
[appdata]: https://docs.rs/actix-web/4/actix_web/struct.App.html#method.app_data
[scope]: https://docs.rs/actix-web/4/actix_web/struct.Scope.html
[webscopeconfig]: https://docs.rs/actix-web/4/actix_web/struct.Scope.html#method.configure
[webscope]: https://docs.rs/actix-web/4/actix_web/web/fn.scope.html
[urlfor]: https://docs.rs/actix-web/4/actix_web/struct.HttpRequest.html#method.url_for
[serviceconfig]: https://docs.rs/actix-web/4/actix_web/web/struct.ServiceConfig.html
