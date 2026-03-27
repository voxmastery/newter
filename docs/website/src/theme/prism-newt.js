/**
 * Prism.js language definition for Newt UI Language.
 * Registered globally so code blocks with language="newt" are highlighted.
 */
(function registerNewt() {
  if (typeof Prism === 'undefined') {
    if (typeof global !== 'undefined' && global.Prism) {
      // Node/SSR environment
    } else {
      return;
    }
  }

  const P = typeof Prism !== 'undefined' ? Prism : global.Prism;

  P.languages.newt = {
    'comment': {
      pattern: /\/\/.*/,
      greedy: true,
    },
    'string': {
      pattern: /"(?:[^"\\]|\\.)*"/,
      greedy: true,
      inside: {
        'interpolation': {
          pattern: /\{[^}]+\}/,
          inside: {
            'interpolation-punctuation': {
              pattern: /^\{|\}$/,
              alias: 'punctuation',
            },
            'expression': {
              pattern: /[\s\S]+/,
            },
          },
        },
      },
    },
    'constant': {
      pattern: /#[0-9a-fA-F]{6}([0-9a-fA-F]{2})?\b/,
    },
    'boolean': /\b(?:true|false)\b/,
    'keyword': /\b(?:let|state|screen|component|theme|use|import|if|else|for|in)\b/,
    'tag': {
      pattern: /\b(?:header|footer|container|sidebar|section|row|column|stack|center|box|widget|card|grid|accordion|bento|breadcrumb|hamburger|kebab|meatballs|doner|tabs|pagination|linkList|nav|button|input|password|search|checkbox|radio|dropdown|combobox|multiselect|datePicker|picker|slider|stepper|toggle|form|modal|confirmDialog|toast|notification|alert|messageBox|tooltip|loader|progressBar|badge|text|icon|tag|comment|feed|carousel|chart|image|spacer)\b/,
    },
    'attr-name': {
      pattern: /\b(?:fill|stroke|radius|padding|gap|fontSize|fontWeight|width|height|minWidth|maxWidth|minHeight|maxHeight|grow|shrink|align|justify|direction|shadow|transition|role|ariaLabel|focusOrder|onClick|href|src|content|columns|rows|aspectRatio|name|placeholder|strokeWidth)\b(?=\s*:)/,
    },
    'function': /\b(?:range)\b(?=\s*\()/,
    'number': /\b\d+(?:\.\d+)?\b/,
    'operator': /->|&&|\|\||[+\-*/%]=?|[=!<>]=?/,
    'punctuation': /[{}()[\],;:]/,
  };
})();
