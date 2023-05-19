import clsx from 'clsx';
import React from 'react';
import Link from '@docusaurus/Link';
import Layout from '@theme/Layout';
import CodeBlock from '../components/code_block.js';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import useBaseUrl from '@docusaurus/useBaseUrl';
import styles from './styles.module.scss';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import {
  faShieldAlt,
  faBatteryFull,
  faPuzzlePiece,
  faTachometerAlt,
} from '@fortawesome/free-solid-svg-icons';

const Home = () => {
  const context = useDocusaurusContext();
  const siteConfig = context;

  return (
    <Layout description={siteConfig.tagline}>
      <Hero />
      <main className={styles.main}>
        <Highlights />
        <Examples />
      </main>
    </Layout>
  );
};

const highlights = [
  {
    icon: faShieldAlt,
    title: 'Segurança de Tipos',
    description: (
      <>
      Esqueça objetos com tipagem baseada em strings, desde a requisição até 
      a resposta, tudo possui tipos.
      </>
    ),
  },
  {
    icon: faBatteryFull,
    title: 'Rico em Recursos',
    description: (
      <>O Actix fornece muitos recursos prontos para uso. HTTP/2, logging, etc.</>
    ),
  },
  {
    icon: faPuzzlePiece,
    title: 'Extensível',
    description: (
      <>Crie facilmente suas próprias bibliotecas que podem ser utilizadas por qualquer aplicação Actix.</>
    ),
  },
  {
    icon: faTachometerAlt,
    title: 'Extremamente Rápido',
    description: (
      <>
      O Actix é extremamente rápido. Não apenas confie em nossas palavras -- <a href='https://www.techempower.com/benchmarks/#section=data-r21&hw=ph&test=fortune' target='_blank' rel='noopener noreferrer'>veja por si mesmo!</a>
      </>
    ),
  },
];

const Hero = () => {
  const context = useDocusaurusContext();
  const { siteConfig } = context;

  return (
    <header id="hero" className={clsx('hero', styles.banner)}>
      <div className="container">
        <img
          src={useBaseUrl(`img/logo.png`)}
          alt="Actix Logo"
          className={styles.logo}
        />

        <h1 className="hero__title">{siteConfig.title}</h1>
        <p className={clsx('hero__subtitle', styles.subtitle)}>
          {siteConfig.tagline}
        </p>

        <div className={styles.buttons}>
          <Link
            className="button button--primary button--lg"
            to={useBaseUrl('docs/')}
          >
            Começar
          </Link>
        </div>
      </div>
    </header>
  );
};

const Highlights = () => {
  return (
    <>
      <section id="highlights" className={styles.highlights}>
        <div className="container">
          <div className="row">
            <div className="col">
              <div className="row">
                {highlights.map((highlight, idx) => (
                  <div
                    className={clsx('col col--6', styles.highlight)}
                    key={idx}
                  >
                    <div className="item">
                      <div className={styles.header}>
                        <div className={styles.icon}>
                          <FontAwesomeIcon icon={highlight.icon} size="lg" />
                        </div>
                        <h2 className={styles.title}>{highlight.title}</h2>
                      </div>
                      <p>{highlight.description}</p>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      </section>
    </>
  );
};

const Examples = () => {
  return (
    <div className={styles.examples}>
      <div className={styles.example}>
        <div className={styles.exampleContent}>
          <div className={styles.featureText}>
            <h3 className={styles.featureTitle}>Olá Mundo!</h3>
            <p>
            Começar com o Actix é fácil. Um aplicativo Actix vem com um sistema 
            de roteamento de URL que permite corresponder a URLs e chamar 
            manipuladores individuais.
            </p>
          </div>
          <div className={styles.example__code}>
            <CodeBlock example="request-routing" section="request-routing" />
          </div>
        </div>
      </div>
      <div className={styles.example}>
        <div className={styles.exampleContent}>
          <div className={styles.featureText}>
            <h3 className={styles.featureTitle}>Respondedores flexíveis</h3>
            <p>
            As funções manipuladoras no Actix podem retornar uma ampla variedade de objetos que 
            implementam a trait <code>Responder</code>. Isso torna fácil retornar respostas 
            consistentes em suas APIs.
            </p>
          </div>
          <div className={styles.example__code}>
            <CodeBlock
              example="flexible-responders"
              section="flexible-responders"
            />
          </div>
        </div>
      </div>
      <div className={styles.example}>
        <div className={styles.exampleContent}>
          <div className={styles.featureText}>
            <h3 className={styles.featureTitle}>Extratores poderosos</h3>
            <p>O Actix vem com um sistema de extração poderoso que extrai dados da requisição HTTP 
              recebida e os passa para as suas funções de visualização. Isso não apenas torna a API 
              conveniente, mas também significa que suas funções de visualização podem ser código síncrono e 
              ainda se beneficiar do tratamento assíncrono de I/O.
            </p>
          </div>
          <div className={styles.example__code}>
            <CodeBlock
              example="powerful-extractors"
              section="powerful-extractors"
            />
          </div>
        </div>
      </div>
      <div className={styles.example}>
        <div className={styles.exampleContent}>
          <div className={styles.featureText}>
            <h3 className={styles.featureTitle}>Manipulação de formulários facilitada</h3>
            <p>
            Manipular dados de formulários JSON ou codificados em URL é fácil. Basta definir uma 
            estrutura que possa ser desserializada e o Actix cuidará do resto.            
            </p>
          </div>
          <div className={styles.example__code}>
            <CodeBlock
              example="easy-form-handling"
              section="easy-form-handling"
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default Home;
