module.exports = {
  docs: {
    'Introdução': ['welcome', 'whatis'],
    'Fundamentos': [
      'getting-started',
      'application',
      'server',
      'extractors',
      'handlers',
    ],
    'Avançado': [
      'errors',
      'url-dispatch',
      'request',
      'response',
      'testing',
      'middleware',
      'static-files',
    ],
    Protocols: ['websockets', 'http2'],
    'Padrões': ['autoreload', 'databases'],
    Diagrams: ['http_server_init', 'conn_lifecycle'],
    Actix: [
      'actix/sec-0-quick-start',
      'actix/sec-1-getting-started',
      'actix/sec-2-actor',
      'actix/sec-3-address',
      'actix/sec-4-context',
      'actix/sec-5-arbiter',
      'actix/sec-6-sync-arbiter',
    ],
    'API Documentation': [
      {
        type: 'link',
        label: 'actix',
        href: 'https://docs.rs/actix/latest/actix/',
      },
      {
        type: 'link',
        label: 'actix-web',
        href: 'https://docs.rs/actix-web/latest/actix_web/',
      },
    ],
  },
};
