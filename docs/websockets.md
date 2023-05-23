---
title: Websockets
---

import CodeBlock from "@site/src/components/code_block.js";

# Websockets

O Actix Web suporta WebSockets com a biblioteca `actix-web-actors`. É possível converter o `Payload` de uma requisição em um fluxo de [_ws::Message_][message] usando um [_web::Payload_][payload] e, em seguida, usar combinadores de fluxo para lidar com as mensagens reais. No entanto, é mais simples lidar com as comunicações do WebSocket usando um ator HTTP.

A seguir está um exemplo de um servidor simples de eco de WebSocket:

<CodeBlock example="websockets" file="main.rs" section="websockets" />

> Um exemplo simples de um servidor de eco de WebSocket está disponível no [diretório de exemplos][examples].

> Um exemplo de servidor de chat com a capacidade de conversar por meio de uma conexão WebSocket ou TCP está disponível no [diretório websocket-chat][chat].

[message]: https://docs.rs/actix-web-actors/2/actix_web_actors/ws/enum.Message.html
[payload]: https://docs.rs/actix-web/4/actix_web/web/struct.Payload.html
[examples]: https://github.com/actix/examples/tree/master/websockets
[chat]: https://github.com/actix/examples/tree/master/websockets/chat
