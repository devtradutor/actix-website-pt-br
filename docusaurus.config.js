const path = require('path');

module.exports = {
  title: 'Actix',
  tagline: 'O Actix Web é um framework web poderoso, pragmático e extremamente rápido para Rust.',
  url: 'https://actix.rs',
  baseUrl: '/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  favicon: 'img/logo.png',
  organizationName: 'actix', // Usually your GitHub org/user name.
  projectName: 'actix-web', // Usually your repo name.
  themeConfig: {
    navbar: {
      title: 'Actix',
      logo: {
        alt: 'Actix Logo',
        src: 'img/logo-icon.png',
        width: 32,
        height: 32,
      },
      items: [
        {
          to: 'docs',
          activeBasePath: 'docs',
          label: 'Documentação',
          position: 'left',
        },
        {
          to: 'community',
          activeBasePath: 'community',
          label: 'Comunidade',
          position: 'left',
        },
        {
          to: 'code',
          activeBasePath: 'code',
          label: 'Código',
          position: 'left',
        },
      ],
    },
    footer: {
      copyright: `Copyright © ${new Date().getFullYear()} The Actix Team`,
    },
    prism: {
      // dracula is closest to docs.rs, where keywords are highlighted
      theme: require('prism-react-renderer/themes/dracula'),
      additionalLanguages: ['rust', 'toml'],
      defaultLanguage: 'rust'
    },
    colorMode: {
      respectPrefersColorScheme: true,
    }
  },
  plugins: ["docusaurus-plugin-sass"],
  presets: [
    [
      '@docusaurus/preset-classic',
      {
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl:
            'https://github.com/werickdasilva/actix-website-pt-br/edit/master',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      },
    ],
  ],
};
