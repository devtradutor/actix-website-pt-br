---
title: Databases
---

import CodeBlock from "@site/src/components/code_block.js";

# Async Options

Temos vários projetos de exemplo que mostram o uso de adaptadores de banco de dados assíncronos:

- Postgres: https://github.com/actix/examples/tree/master/databases/postgres
- SQLite: https://github.com/actix/examples/tree/master/databases/sqlite
- MongoDB: https://github.com/actix/examples/tree/master/databases/mongodb

# Diesel

As versões atuais do Diesel (v1/v2) não suportam operações assíncronas, portanto, é importante usar a função [`web::block`][web-block] para transferir suas operações de banco de dados para a thread-pool de tempo de execução do Actix.

Você pode criar funções de ação que correspondam a todas as operações que seu aplicativo realizará no banco de dados.

<CodeBlock example="databases" file="main.rs" section="handler" />

Agora você deve configurar o pool do banco de dados usando uma biblioteca como `r2d2`, que disponibiliza várias conexões de banco de dados para o seu aplicativo. Isso significa que vários manipuladores podem interagir com o banco de dados ao mesmo tempo e ainda aceitar novas conexões. Simplesmente, o pool será parte do estado do seu aplicativo. (Nesse caso, é benéfico não usar uma struct de invólucro de estado, pois o pool cuida do acesso compartilhado para você.)

<CodeBlock example="databases" file="main.rs" section="main" />

Agora, em um manipulador de solicitação, use o extrator `Data<T>` para obter o pool a partir do estado do aplicativo e obter uma conexão a partir dele. Isso fornece uma conexão de banco de dados de propriedade que pode ser passada para um fechamento [`web::block`][web-block]. Em seguida, basta chamar a função de ação com os argumentos necessários e usar o `.await` no resultado.

Este exemplo também mapeia o erro para um `HttpResponse` antes de usar o operador `?`, mas isso não é necessário se o seu tipo de erro de retorno implementar [`ResponseError`][response-error].

<CodeBlock example="databases" file="main.rs" section="index" />

Isso é tudo! Veja o exemplo completo aqui: https://github.com/actix/examples/tree/master/databases/diesel

[web-block]: https://docs.rs/actix-web/4/actix_web/web/fn.block.html
[response-error]: https://docs.rs/actix-web/4/actix_web/trait.ResponseError.html
