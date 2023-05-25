---
title: Respostas
---

import CodeBlock from "@site/src/components/code_block.js";


# Resposta

Um padrão semelhante ao de um construtor é usado para construir uma instância de `HttpResponse`. `HttpResponse` fornece vários métodos que retornam uma instância de `HttpResponseBuilder`, que implementa vários métodos auxiliares para construir respostas.

> Consulte a [documentação][responsebuilder] para descrições de tipos.

Os métodos `.body`, `.finish` e `.json` finalizam a criação da resposta e retornam uma instância construída de _HttpResponse_. Se esses métodos forem chamados várias vezes na mesma instância do construtor, o construtor gerará um erro.

<CodeBlock example="responses" file="main.rs" section="builder" />

## Resposta JSON

O tipo `Json` permite responder com dados JSON bem formados: basta retornar um valor do tipo `Json<T>`, onde `T` é o tipo de uma estrutura a ser serializada em _JSON_. O tipo `T` deve implementar a trait `Serialize` do _serde_.

Para que o exemplo a seguir funcione, você precisa adicionar `serde` às suas dependências no arquivo `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

<CodeBlock example="responses" file="json_resp.rs" section="json-resp" />

Usar o tipo `Json` dessa forma, em vez de chamar o método `.json` em um `HttpResponse`, torna imediatamente claro que a função retorna JSON e não qualquer outro tipo de resposta.

## Codificação de conteúdo

O Actix Web pode comprimir automaticamente cargas úteis com o [_Compress middleware_][compressmidddleware]. Os seguintes codecs são suportados:

- Brotli
- Gzip
- Deflate
- Identity

O cabeçalho `Content-Encoding` de uma resposta tem o valor padrão `ContentEncoding::Auto`, que realiza uma negociação automática de compressão de conteúdo com base no cabeçalho `Accept-Encoding` da requisição.

<CodeBlock example="responses" file="auto.rs" section="auto" />

Desative explicitamente a compressão de conteúdo em um manipulador definindo `Content-Encoding` como um valor `Identity`:

<CodeBlock example="responses" file="identity.rs" section="identity" />

Ao lidar com um corpo já comprimido (por exemplo, ao servir ativos pré-comprimidos), defina manualmente o cabeçalho `Content-Encoding` na resposta para ignorar o middleware:

[responsebuilder]: link_para_a_documentação
[compressmidddleware]: link_para_o_Compress_middleware
<CodeBlock example="responses" file="identity_two.rs" section="identity-two" />

[responsebuilder]: https://docs.rs/actix-web/4/actix_web/struct.HttpResponseBuilder.html
[compressmidddleware]: https://docs.rs/actix-web/4/actix_web/middleware/struct.Compress.html
