---
title: HTTP/2
---

import RenderCodeBlock from '@theme/CodeBlock';
import CodeBlock from '@site/src/components/code_block.js';
import { actixWebMajorVersion } from "@site/vars";

`actix-web` atualiza automaticamente as conexões para o *HTTP/2*, se possível.

# Negociação

Quando uma das funcionalidades `rustls` ou `openssl` está habilitada, o `HttpServer` fornece os métodos [bind_rustls][bindrustls] e [bind_openssl][bindopenssl], respectivamente.

<!-- DEPENDENCY -->


<RenderCodeBlock className="language-toml">
{`[dependencies]
actix-web = { version = "${actixWebMajorVersion}", features = ["openssl"] }
openssl = { version = "0.10", features = ["v110"] }
`}
</RenderCodeBlock>

<CodeBlock example="http2" file="main.rs" section="main" />

As atualizações para o HTTP/2 descritas no [RFC 7540 §3.2][rfcsection32] não são suportadas. O início do HTTP/2 com conhecimento prévio é suportado tanto para conexões de texto simples quanto TLS ([RFC 7540 §3.4][rfcsection34]) (quando utilizando os construtores de serviço de nível mais baixo `actix-http`).

> Confira [os exemplos de TLS][examples] para um exemplo concreto.


[rfcsection32]: https://httpwg.org/specs/rfc7540.html#rfc.section.3.2
[rfcsection34]: https://httpwg.org/specs/rfc7540.html#rfc.section.3.4
[bindrustls]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.bind_rustls
[bindopenssl]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.bind_openssl
[tlsalpn]: https://tools.ietf.org/html/rfc7301
[examples]: https://github.com/actix/examples/tree/master/https-tls
