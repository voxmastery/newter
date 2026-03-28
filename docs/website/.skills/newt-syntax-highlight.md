# Newt Prism.js Syntax Highlighting Skill

## File location
src/theme/prism-newt.js

## Token definitions (priority order)
1. comment: /\/\/.*/
2. string: /"(?:[^"\\]|\\.)*"/ with nested interpolation /\{[^}]+\}/
3. hex-color: /#[0-9a-fA-F]{6,8}\b/ (alias: constant)
4. number: /\b\d+(?:\.\d+)?\b/
5. keyword: let, state, screen, component, theme, use, import, if, else, for, in
6. element: all 73 element names (alias: tag)
7. prop: all prop names followed by colon (alias: attr-name)
8. boolean: true, false
9. punctuation: {}()[];:,
10. operator: + - * / % == != < <= > >= && || -> !

## Color theme (Nord-inspired, dark bg #0f111a)
- comment: #636f88 italic
- string: #a3be8c
- interpolation: #88c0d0
- number: #b48ead
- keyword: #81a1c1
- element: #8fbcbb
- property: #88c0d0
- boolean: #d08770
- punctuation: #d8dee9
- operator: #81a1c1
