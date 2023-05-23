# Actix Website

## Começando


Para Construir o site depende do [Docusaurus][docusaurus], você precisa ter o `npm` ou `yarn` instalado. Você pode executar o site localmente com o seguinte comando:

```sh
git clone https://github.com/actix/actix-website.git
cd actix-website
npm install  # ou yarn install
npm start  # ou yarn start
```

Em seguida, visite http://localhost:3000.

## Atualizando diagramas

Os diagramas estão localizados em [/static/img/diagrams/](https://github.com/actix/actix-website/tree/master/static/img/diagrams) e são construídos com o [Mermaid CLI][mermaid_cli].

Por exemplo, para editar o diagrama `connection_overview`:

```sh
cd static/img/diagrams
vi connection_overview.mmd
# Compile diagrams:
mmdc -i connection_overview.mmd -o connection_overview.svg
```

## Licença

Este site está licenciado sob uma das seguintes opções:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT])

<!-- LINKS -->

[docusaurus]: https://docusaurus.io/
[mermaid_cli]: https://github.com/mermaidjs/mermaid.cli
