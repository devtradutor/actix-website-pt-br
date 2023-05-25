---
title: Arquivos Estático
---

import CodeBlock from "@site/src/components/code_block.js";

# Arquivo individual

É possível servir arquivos estáticos com um padrão de caminho personalizado e `NamedFile`. Para corresponder a uma parte do caminho, podemos usar uma expressão regular `[.*]`.

<CodeBlock example="static-files" file="main.rs" section="individual-file" />

:::warning 
Corresponder a uma parte do caminho com a expressão regular `[.*]` e usá-la para retornar um `NamedFile` tem sérias implicações de segurança. 
Isso oferece a possibilidade de um atacante inserir `../` na URL e acessar todos os arquivos no host aos quais o usuário que executa o servidor tem acesso.
:::

## Diretório

Para servir arquivos de diretórios e subdiretórios específicos, pode-se usar [`Files`][files]. O `Files` deve ser registrado com o método `App::service()`, caso contrário, não será possível servir subcaminhos.

<CodeBlock example="static-files" file="directory.rs" section="directory" />

Por padrão, a listagem de arquivos para subdiretórios está desativada. Uma tentativa de carregar a listagem de diretórios retornará uma resposta _404 Not Found_. Para habilitar a listagem de arquivos, use o método [`Files::show_files_listing()`][showfileslisting].

Em vez de mostrar a listagem de arquivos de um diretório, é possível redirecionar para um arquivo de índice específico. Use o método [`Files::index_file()`][indexfile] para configurar esse redirecionamento.

## Configuração

`NamedFiles` pode especificar várias opções para servir arquivos:

- `set_content_disposition` - função usada para mapear o tipo MIME do arquivo para o tipo `Content-Disposition` correspondente
- `use_etag` - especifica se o `ETag` deve ser calculado e incluído nos cabeçalhos.
- `use_last_modified` - especifica se o timestamp de modificação do arquivo deve ser usado e adicionado ao cabeçalho `Last-Modified`.

Todos os métodos acima são opcionais e fornecidos com as melhores configurações padrão, mas é possível personalizar qualquer um deles.

<CodeBlock example="static-files" file="configuration.rs" section="config-one" />

A configuração também pode ser aplicada ao serviço de diretório:

<CodeBlock example="static-files" file="configuration_two.rs" section="config-two" />

[files]: https://docs.rs/actix-files/0.6/actix_files/struct.Files.html#
[showfileslisting]: https://docs.rs/actix-files/0.6/actix_files/struct.Files.html#method.show_files_listing
[indexfile]: https://docs.rs/actix-files/0.6/actix_files/struct.Files.html#method.index_file
