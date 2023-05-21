---
title: Extratores
---

import CodeBlock from "@site/src/components/code_block.js";

# Extração de informações com segurança de tipo

O Actix Web oferece uma facilidade para acessar informações de requisição com segurança de tipo, chamada de _extractors_ (ou seja, `impl FromRequest`). Existem muitas implementações de extractors embutidas (veja os [implementadores](https://docs.rs/actix-web/latest/actix_web/trait.FromRequest.html#implementors)).

Um extractor pode ser acessado como argumento de uma função de handler. O Actix Web suporta até 12 extractors por função de handler. A posição do argumento não importa.

<CodeBlock example="extractors" file="main.rs" section="option-one" />

## Path

[_Path_][pathstruct] fornece informações extraídas do caminho da requisição. As partes do caminho que podem ser extraídas são chamadas de "segmentos dinâmicos" e são marcadas com chaves. Você pode desserializar qualquer segmento variável do caminho.

Por exemplo, para um recurso registrado no caminho `/users/{user_id}/{friend}`, dois segmentos podem ser desserializados: `user_id` e `friend`. Esses segmentos podem ser extraídos como uma tupla na ordem em que são declarados (por exemplo, `Path<(u32, String)>`).

<CodeBlock example="extractors" file="path_one.rs" section="path-one" />

Também é possível extrair informações do caminho para um tipo que implementa o trait `Deserialize` do `serde`, combinando os nomes dos segmentos dinâmicos com os nomes dos campos. Aqui está um exemplo equivalente que usa o `serde` em vez de um tipo tupla.

<CodeBlock example="extractors" file="path_two.rs" section="path-two" />

Como uma alternativa não segura de tipo, também é possível consultar (veja [documentação do `match_info`](https://docs.rs/actix-web/latest/actix_web/struct.HttpRequest.html#method.match_info)) a requisição para parâmetros de caminho por nome dentro de um handler:

<CodeBlock example="extractors" file="path_three.rs" section="path-three" />

## Query

O tipo [`Query<T>`][querystruct] fornece funcionalidade de extração para os parâmetros de consulta da requisição. Por baixo, ele usa a crate `serde_urlencoded`.

<CodeBlock example="extractors" file="query.rs" section="query" />

## JSON

[`Json<T>`][jsonstruct] permite desserializar o corpo de uma requisição em uma estrutura. Para extrair informações digitadas do corpo de uma requisição, o tipo `T` deve implementar `serde::Deserialize`.

<CodeBlock example="extractors" file="json_one.rs" section="json-one" />

Alguns extractors fornecem uma maneira de configurar o processo de extração. Para configurar um extractor, passe seu objeto de configuração para o método `.app_data()` do recurso. No caso do extractor _Json_, ele retorna um [_JsonConfig_][jsonconfig]. Você pode configurar o tamanho máximo da carga JSON, bem como uma função de tratamento de erros personalizada.

O exemplo a seguir limita o tamanho da carga útil para 4kb e usa um tratador de erros personalizado.

<CodeBlock example="extractors" file="json_two.rs" section="json-two"/>

## Formulários Codificados em URL

Um corpo de formulário codificado em URL pode ser extraído para uma estrutura, assim como `Json<T>`. Esse tipo deve implementar `serde::Deserialize`.

[_FormConfig_][formconfig] permite configurar o processo de extração.

<CodeBlock example="extractors" file="form.rs" section="form" />

## Outros

O Actix Web também oferece muitos outros extractors. Aqui estão alguns importantes:

- [`Data`][datastruct] - Para acessar partes do estado da aplicação.
- [`HttpRequest`][httprequest] - `HttpRequest` é ele mesmo um extractor, caso você precise de acesso a outras partes da requisição.
- `String` - Você pode converter a carga útil de uma requisição para uma `String`. [Um exemplo][stringexample] está disponível na documentação do Rust.
- [`Bytes`][bytes] - Você pode converter a carga útil de uma requisição em _Bytes_. [Um exemplo][bytesexample] está disponível na documentação do Rust.
- [`Payload`][payload] - Extractor de carga útil de baixo nível, principalmente para construir outros extractors. [Um exemplo][payloadexample] está disponível na documentação do Rust.

## Extractor de Estado da Aplicação

O estado da aplicação pode ser acessado a partir do manipulador com o extractor `web::Data`; no entanto, o estado é acessível apenas como uma referência somente leitura. Se você precisar de acesso mutável ao estado, ele deve ser implementado.

Aqui está um exemplo de um manipulador que armazena o número de requisições processadas:

<CodeBlock example="request-handlers" file="main.rs" section="data" />

Embora esse manipulador funcione, `data.count` contará apenas o número de requisições tratadas _por cada thread de trabalho_. Para contar o número total de requisições em todas as threads, você deve usar um `Arc` compartilhado e [atomics][atomics].

<CodeBlock example="request-handlers" file="handlers_arc.rs" section="arc" />

**Observação**: Se você deseja compartilhar o estado _inteiro_ entre todas as threads, use `web::Data` e `app_data`, conforme descrito em [Estado Mutável Compartilhado][shared_mutable_state].

Tenha cuidado ao usar primitivas de sincronização bloqueadoras, como `Mutex` ou `RwLock`, dentro do estado da sua aplicação. O Actix Web trata as requisições de forma assíncrona. É um problema se a [_seção crítica_][critical_section] no seu handler for muito grande ou conter um ponto de `.await`. Se isso for uma preocupação, recomendamos que você também leia [o conselho do Tokio sobre o uso de `Mutex` bloqueador em código assíncrono][tokio_std_mutex].

[pathstruct]: https://docs.rs/actix-web/4/actix_web/dev/struct.Path.html
[querystruct]: https://docs.rs/actix-web/4/actix_web/web/struct.Query.html
[jsonstruct]: https://docs.rs/actix-web/4/actix_web/web/struct.Json.html
[jsonconfig]: https://docs.rs/actix-web/4/actix_web/web/struct.JsonConfig.html
[formconfig]: https://docs.rs/actix-web/4/actix_web/web/struct.FormConfig.html
[datastruct]: https://docs.rs/actix-web/4/actix_web/web/struct.Data.html
[httprequest]: https://docs.rs/actix-web/4/actix_web/struct.HttpRequest.html
[stringexample]: https://docs.rs/actix-web/4/actix_web/trait.FromRequest.html#impl-FromRequest-for-String
[bytes]: https://docs.rs/actix-web/4/actix_web/web/struct.Bytes.html
[bytesexample]: https://docs.rs/actix-web/4/actix_web/trait.FromRequest.html#impl-FromRequest-5
[payload]: https://docs.rs/actix-web/4/actix_web/web/struct.Payload.html
[payloadexample]: https://docs.rs/actix-web/4/actix_web/web/struct.Payload.html
[docsrs_match_info]: https://docs.rs/actix-web/latest/actix_web/struct.HttpRequest.html#method.match_info
[actix]: /actix/docs/
[atomics]: https://doc.rust-lang.org/std/sync/atomic/
[shared_mutable_state]: /docs/application#shared-mutable-state
[critical_section]: https://en.wikipedia.org/wiki/Critical_section
[tokio_std_mutex]: https://tokio.rs/tokio/tutorial/shared-state#on-using-stdsyncmutex
