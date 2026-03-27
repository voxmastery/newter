/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  docs: [
    {
      type: 'category',
      label: 'Getting Started',
      collapsed: false,
      items: [
        'getting-started/index',
        'getting-started/installation',
      ],
    },
    {
      type: 'category',
      label: 'Language',
      collapsed: false,
      items: [
        'language/index',
        'language/elements',
        'language/props',
        'language/components',
        'language/state',
        'language/control-flow',
        'language/themes',
        'language/string-interpolation',
        'language/imports',
      ],
    },
    {
      type: 'category',
      label: 'Compiler',
      items: [
        'compiler/index',
        'compiler/cli',
        'compiler/html-export',
        'compiler/canvas-ide',
      ],
    },
    {
      type: 'category',
      label: 'Tooling',
      items: [
        'tooling/vscode',
        'tooling/lsp',
      ],
    },
    {
      type: 'category',
      label: 'Examples',
      items: [
        'examples/index',
      ],
    },
  ],
};

export default sidebars;
