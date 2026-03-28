import {themes as prismThemes} from 'prism-react-renderer';

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'Newt',
  tagline: 'A UI language that compiles to canvas, HTML, and JSON',
  favicon: 'img/favicon.svg',

  future: {
    v4: true,
  },

  markdown: {
    mermaid: true,
  },

  url: 'https://newter.vercel.app',
  baseUrl: '/',

  organizationName: 'voxmastery',
  projectName: 'newter',

  onBrokenLinks: 'throw',

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: './sidebars.js',
          editUrl: 'https://github.com/voxmastery/newter/tree/main/docs/website/',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      }),
    ],
  ],

  themes: [
    '@docusaurus/theme-mermaid',
    [
      require.resolve('@easyops-cn/docusaurus-search-local'),
      /** @type {import("@easyops-cn/docusaurus-search-local").PluginOptions} */
      ({
        hashed: true,
        language: ['en'],
        indexBlog: false,
        docsRouteBasePath: '/docs',
        highlightSearchTermsOnTargetPage: true,
        searchBarShortcutHint: false,
        searchResultLimits: 8,
        searchResultContextMaxLength: 60,
      }),
    ],
  ],

  headTags: [
    {
      tagName: 'meta',
      attributes: {
        property: 'og:image',
        content: 'https://newter.vercel.app/img/og-image.svg',
      },
    },
    {
      tagName: 'meta',
      attributes: {
        name: 'twitter:card',
        content: 'summary_large_image',
      },
    },
    {
      tagName: 'meta',
      attributes: {
        name: 'twitter:image',
        content: 'https://newter.vercel.app/img/og-image.svg',
      },
    },
    {
      tagName: 'link',
      attributes: {
        rel: 'preconnect',
        href: 'https://fonts.googleapis.com',
      },
    },
    {
      tagName: 'link',
      attributes: {
        rel: 'preconnect',
        href: 'https://fonts.gstatic.com',
        crossorigin: 'anonymous',
      },
    },
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      image: 'img/og-image.svg',
      metadata: [
        {name: 'description', content: 'Newt is a UI language that compiles to canvas, HTML, and JSON. 73 built-in elements, reactive state, and a syntax you can learn in 5 minutes.'},
        {name: 'keywords', content: 'newt, UI language, canvas, HTML, JSON, declarative UI, Rust compiler'},
        {property: 'og:type', content: 'website'},
      ],
      colorMode: {
        defaultMode: 'light',
        respectPrefersColorScheme: true,
      },
      navbar: {
        title: 'newt',
        logo: {
          alt: 'Newt Logo',
          src: 'img/logo.svg',
        },
        items: [
          {
            type: 'docSidebar',
            sidebarId: 'docs',
            position: 'left',
            label: 'Docs',
          },
          {
            to: '/docs/examples',
            label: 'Examples',
            position: 'left',
          },
          {
            href: 'https://github.com/voxmastery/newter',
            position: 'right',
            className: 'header-github-link',
            'aria-label': 'GitHub repository',
          },
        ],
      },
      footer: {
        style: 'light',
        copyright: `Newt UI Language — MIT License`,
      },
      prism: {
        theme: prismThemes.dracula,
        darkTheme: prismThemes.dracula,
      },
    }),
};

export default config;
