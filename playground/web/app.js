/* ============================================================
   Newt Playground — Application Logic
   CodeMirror 5, debounced compilation, tabs, sharing, resize
   ============================================================ */

(function () {
  'use strict';

  // ---- Examples ----

  const EXAMPLES = {
    counter: [
      'state count = 0;',
      '',
      'screen Counter {',
      '    column(gap: 24, padding: 48, fill: #f9fafb)(',
      '        text("Count: {count}", fontSize: 32, fontWeight: "700")',
      '        row(gap: 12)(',
      '            button("+1", fill: #2563eb, radius: 8, onClick: { count = count + 1 })',
      '            button("Reset", fill: #ef4444, radius: 8, onClick: { count = 0 })',
      '        )',
      '    )',
      '}',
    ].join('\n'),

    dashboard: [
      'let accent = #2563eb;',
      '',
      'component StatCard(label, value) {',
      '    card(fill: #ffffff, stroke: #e5e7eb, radius: 12, padding: 20)(',
      '        column(gap: 8)(',
      '            text { content: label, fontSize: 12 }',
      '            text { content: value, fontSize: 28, fontWeight: "700" }',
      '        )',
      '    )',
      '}',
      '',
      'screen Dashboard {',
      '    column(fill: #f3f4f6)(',
      '        header(fill: #ffffff, stroke: #e5e7eb, padding: 16)(',
      '            row(gap: 12)(',
      '                text("Dashboard", fontSize: 20, fontWeight: "700")',
      '                spacer()',
      '                button("Settings", stroke: #e5e7eb, radius: 8)',
      '            )',
      '        )',
      '        row(gap: 16, padding: 24)(',
      '            StatCard("Revenue", "$12,400")',
      '            StatCard("Users", "1,204")',
      '            StatCard("Orders", "342")',
      '        )',
      '    )',
      '}',
    ].join('\n'),

    form: [
      'state submitted = false;',
      '',
      'screen ContactForm {',
      '    center(fill: #f9fafb)(',
      '        card(fill: #ffffff, stroke: #e5e7eb, radius: 16, padding: 32)(',
      '            column(gap: 20)(',
      '                text("Contact Us", fontSize: 24, fontWeight: "700")',
      '                text("We would love to hear from you", fontSize: 14)',
      '',
      '                column(gap: 8)(',
      '                    text("Name", fontSize: 14, fontWeight: "500")',
      '                    input(placeholder: "Your name", stroke: #d1d5db, radius: 8, padding: 12)',
      '                )',
      '',
      '                column(gap: 8)(',
      '                    text("Email", fontSize: 14, fontWeight: "500")',
      '                    input(placeholder: "you@example.com", stroke: #d1d5db, radius: 8, padding: 12)',
      '                )',
      '',
      '                column(gap: 8)(',
      '                    text("Message", fontSize: 14, fontWeight: "500")',
      '                    textarea(placeholder: "What is on your mind?", stroke: #d1d5db, radius: 8, padding: 12)',
      '                )',
      '',
      '                button("Send Message", fill: #7c3aed, radius: 8, fontSize: 16, onClick: { submitted = true })',
      '',
      '                if submitted {',
      '                    alert(fill: #dcfce7, stroke: #10b981, radius: 8, padding: 12)(',
      '                        text("Thanks! We will get back to you soon.", fontSize: 14)',
      '                    )',
      '                }',
      '            )',
      '        )',
      '    )',
      '}',
    ].join('\n'),
  };

  // ---- Compiler (WASM with placeholder fallback) ----

  function compileNewt(source, target) {
    // Use real WASM compiler if loaded
    var wasm = window.__newtWasm;
    if (wasm) {
      try {
        var output;
        if (target === 'html') {
          output = wasm.compile_to_html(source);
        } else if (target === 'react') {
          output = wasm.compile_to_react(source);
        } else if (target === 'json') {
          output = wasm.compile_to_json(source);
        } else {
          return { ok: false, output: 'Unknown target: ' + target };
        }
        return { ok: true, output: output };
      } catch (e) {
        // Show compile errors from WASM
        return { ok: false, output: String(e) };
      }
    }

    // Fallback to placeholder if WASM not loaded yet (loading...)
    if (target === 'html') {
      return { ok: true, output: buildPlaceholderHTML(source) };
    }
    if (target === 'react') {
      return { ok: true, output: buildPlaceholderReact(source) };
    }
    if (target === 'json') {
      return { ok: true, output: buildPlaceholderJSON(source) };
    }
    return { ok: false, output: 'Unknown target: ' + target };
  }

  // Re-compile when WASM loads
  document.addEventListener('wasm-ready', function () {
    var badge = document.querySelector('.status-badge');
    if (badge) {
      badge.textContent = 'WASM Compiler Ready';
      badge.style.color = '#10b981';
    }
    triggerCompile();
  });

  function buildPlaceholderHTML(source) {
    var lines = source.split('\n').length;
    var screens = (source.match(/screen\s+\w+/g) || []).map(function (s) {
      return s.replace('screen ', '');
    });
    var states = (source.match(/state\s+\w+/g) || []).map(function (s) {
      return s.replace('state ', '');
    });

    return [
      '<!DOCTYPE html>',
      '<html><head><style>',
      '  body { font-family: Inter, system-ui, sans-serif; margin: 0; padding: 32px;',
      '         background: #f9fafb; color: #18181b; }',
      '  .card { background: #fff; border-radius: 12px; padding: 24px;',
      '          box-shadow: 0 1px 3px rgba(0,0,0,0.1); max-width: 480px; }',
      '  h2 { margin: 0 0 8px; font-size: 18px; }',
      '  p { margin: 4px 0; font-size: 14px; color: #6b7280; }',
      '  .tag { display: inline-block; padding: 2px 8px; border-radius: 4px;',
      '         background: #ede9fe; color: #7c3aed; font-size: 12px; font-weight: 500;',
      '         margin: 2px; }',
      '  .note { margin-top: 16px; padding: 12px; border-radius: 8px;',
      '          background: #f3f0ff; font-size: 13px; color: #5b21b6; }',
      '</style></head><body>',
      '  <div class="card">',
      '    <h2>Newt Preview</h2>',
      '    <p>' + lines + ' lines parsed</p>',
      screens.length
        ? '    <p>Screens: ' +
          screens.map(function (s) { return '<span class="tag">' + escapeHTML(s) + '</span>'; }).join(' ') +
          '</p>'
        : '',
      states.length
        ? '    <p>State: ' +
          states.map(function (s) { return '<span class="tag">' + escapeHTML(s) + '</span>'; }).join(' ') +
          '</p>'
        : '',
      '    <div class="note">',
      '      Live rendering will be available when the WASM compiler is connected.',
      '      This preview shows parsed metadata from your Newt source.',
      '    </div>',
      '  </div>',
      '</body></html>',
    ].join('\n');
  }

  function buildPlaceholderReact(source) {
    var screens = (source.match(/screen\s+\w+/g) || []).map(function (s) {
      return s.replace('screen ', '');
    });
    var mainScreen = screens[0] || 'App';

    return [
      '// Auto-generated React component (placeholder)',
      '// Connect WASM compiler for real JSX output',
      '',
      'import React, { useState } from "react";',
      '',
      'export default function ' + mainScreen + '() {',
      '  // State declarations will be extracted from Newt source',
      (source.match(/state\s+\w+\s*=\s*.+/g) || [])
        .map(function (line) {
          var match = line.match(/state\s+(\w+)\s*=\s*(.+)/);
          if (!match) return '';
          var name = match[1];
          var val = match[2].trim();
          var setter = 'set' + name.charAt(0).toUpperCase() + name.slice(1);
          return '  const [' + name + ', ' + setter + '] = useState(' + val + ');';
        })
        .filter(Boolean)
        .join('\n'),
      '',
      '  return (',
      '    <div className="app">',
      '      {/* Newt UI tree will render here */}',
      '      <p>Component: ' + escapeHTML(mainScreen) + '</p>',
      '    </div>',
      '  );',
      '}',
    ].join('\n');
  }

  function buildPlaceholderJSON(source) {
    var screens = (source.match(/screen\s+(\w+)/g) || []).map(function (s) {
      return s.replace('screen ', '');
    });
    var states = [];
    (source.match(/state\s+(\w+)\s*=\s*(.+)/g) || []).forEach(function (line) {
      var m = line.match(/state\s+(\w+)\s*=\s*(.+)/);
      if (m) {
        var val = m[2].trim();
        // Attempt to parse value
        if (val === 'true' || val === 'false') val = val === 'true';
        else if (val.match(/^\d+$/)) val = parseInt(val, 10);
        else if (val.match(/^".*"$/)) val = val.slice(1, -1);
        states.push({ name: m[1], initialValue: val });
      }
    });
    var components = (source.match(/component\s+(\w+)/g) || []).map(function (s) {
      return s.replace('component ', '');
    });

    var ast = {
      type: 'NewtProgram',
      version: '0.1.0',
      screens: screens,
      state: states,
      components: components,
      _note: 'Placeholder AST. Connect WASM compiler for full output.',
    };

    return JSON.stringify(ast, null, 2);
  }

  function escapeHTML(str) {
    return str
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;');
  }

  // ---- Editor Initialization ----

  var editor = CodeMirror(document.getElementById('editor'), {
    value: '',
    mode: 'javascript', // closest built-in mode for Newt syntax highlighting
    theme: 'dracula',
    lineNumbers: true,
    autoCloseBrackets: true,
    matchBrackets: true,
    indentUnit: 4,
    tabSize: 4,
    indentWithTabs: false,
    lineWrapping: false,
    extraKeys: {
      Tab: function (cm) {
        cm.replaceSelection('    ', 'end');
      },
      'Ctrl-S': function () {
        // Format placeholder — will format when compiler is ready
        showToast('Formatting is not yet available');
      },
      'Ctrl-Enter': function () {
        updatePreview();
        showToast('Compiled');
      },
      'Ctrl-Shift-C': function () {
        copyOutput();
      },
    },
  });

  // ---- Active Tab State ----

  var activeTab = 'html';
  var debounceTimer = null;

  // ---- Debounced Compilation ----

  editor.on('change', function () {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(updatePreview, 150);
  });

  function updatePreview() {
    var source = editor.getValue();
    updateTab(source, activeTab);
  }

  // Expose for WASM-ready event
  window.triggerCompile = updatePreview;

  function updateTab(source, target) {
    var result = compileNewt(source, target);
    var htmlFrame = document.getElementById('preview-html');
    var reactPre = document.getElementById('preview-react');
    var jsonPre = document.getElementById('preview-json');

    // Hide all
    htmlFrame.classList.add('hidden');
    reactPre.classList.add('hidden');
    jsonPre.classList.add('hidden');

    if (target === 'html') {
      htmlFrame.classList.remove('hidden');
      if (!result.ok) {
        htmlFrame.srcdoc = '<html><body style="font-family:monospace;color:#ef4444;padding:16px;background:#0f111a;"><pre>' + result.output + '</pre></body></html>';
      } else {
        htmlFrame.srcdoc = result.output;
      }
    } else if (target === 'react') {
      reactPre.classList.remove('hidden');
      var reactCode = reactPre.querySelector('code');
      reactCode.textContent = result.ok ? result.output : 'Error: ' + result.output;
      reactCode.style.color = result.ok ? '' : '#ef4444';
    } else if (target === 'json') {
      jsonPre.classList.remove('hidden');
      var jsonCode = jsonPre.querySelector('code');
      jsonCode.textContent = result.ok ? result.output : 'Error: ' + result.output;
      jsonCode.style.color = result.ok ? '' : '#ef4444';
    }
  }

  // ---- Tab Switching ----

  var tabs = document.querySelectorAll('.tab');
  tabs.forEach(function (tab) {
    tab.addEventListener('click', function () {
      tabs.forEach(function (t) {
        t.classList.remove('active');
        t.setAttribute('aria-selected', 'false');
      });
      tab.classList.add('active');
      tab.setAttribute('aria-selected', 'true');
      activeTab = tab.getAttribute('data-target');
      updatePreview();
    });
  });

  // ---- Copy Output ----

  function getActiveOutput() {
    if (activeTab === 'html') {
      var frame = document.getElementById('preview-html');
      return frame.srcdoc || '';
    }
    if (activeTab === 'react') {
      return document.querySelector('#preview-react code').textContent;
    }
    if (activeTab === 'json') {
      return document.querySelector('#preview-json code').textContent;
    }
    return '';
  }

  function copyOutput() {
    var text = getActiveOutput();
    if (!text) return;
    navigator.clipboard.writeText(text).then(function () {
      showToast('Copied to clipboard');
    }).catch(function () {
      showToast('Failed to copy');
    });
  }

  document.getElementById('copy-btn').addEventListener('click', copyOutput);

  // ---- Share ----

  function encodeSource(source) {
    try {
      return btoa(unescape(encodeURIComponent(source)));
    } catch (e) {
      return '';
    }
  }

  function decodeSource(encoded) {
    try {
      return decodeURIComponent(escape(atob(encoded)));
    } catch (e) {
      return '';
    }
  }

  document.getElementById('share-btn').addEventListener('click', function () {
    var source = editor.getValue();
    var encoded = encodeSource(source);
    if (!encoded) {
      showToast('Failed to encode');
      return;
    }
    var url = window.location.origin + window.location.pathname + '#code=' + encoded;
    navigator.clipboard.writeText(url).then(function () {
      window.location.hash = 'code=' + encoded;
      showToast('Share link copied');
    }).catch(function () {
      window.location.hash = 'code=' + encoded;
      showToast('Link updated in address bar');
    });
  });

  // ---- Load from URL Hash ----

  function loadFromHash() {
    var hash = window.location.hash;
    if (hash.startsWith('#code=')) {
      var encoded = hash.slice(6);
      var source = decodeSource(encoded);
      if (source) {
        editor.setValue(source);
        return true;
      }
    }
    return false;
  }

  // ---- Example Selector ----

  document.getElementById('examples').addEventListener('change', function (e) {
    var key = e.target.value;
    if (key && EXAMPLES[key]) {
      editor.setValue(EXAMPLES[key]);
      window.location.hash = '';
    }
    // Reset select to show "Examples" label
    e.target.value = '';
  });

  // ---- Resizable Split Pane ----

  (function initResize() {
    var divider = document.getElementById('divider');
    var playground = document.querySelector('.playground');
    var isDragging = false;
    var isVertical = window.innerWidth <= 768;

    function onResize() {
      isVertical = window.innerWidth <= 768;
    }
    window.addEventListener('resize', onResize);

    divider.addEventListener('mousedown', startDrag);
    divider.addEventListener('touchstart', startDrag, { passive: false });

    function startDrag(e) {
      e.preventDefault();
      isDragging = true;
      divider.classList.add('dragging');
      document.body.style.cursor = isVertical ? 'row-resize' : 'col-resize';
      document.body.style.userSelect = 'none';

      document.addEventListener('mousemove', onDrag);
      document.addEventListener('touchmove', onDrag, { passive: false });
      document.addEventListener('mouseup', stopDrag);
      document.addEventListener('touchend', stopDrag);
    }

    function onDrag(e) {
      if (!isDragging) return;
      e.preventDefault();

      var clientX, clientY;
      if (e.touches) {
        clientX = e.touches[0].clientX;
        clientY = e.touches[0].clientY;
      } else {
        clientX = e.clientX;
        clientY = e.clientY;
      }

      var rect = playground.getBoundingClientRect();

      if (isVertical) {
        var offsetY = clientY - rect.top;
        var totalHeight = rect.height;
        var pct = (offsetY / totalHeight) * 100;
        pct = Math.max(15, Math.min(85, pct));
        playground.style.gridTemplateRows = pct + '% 4px ' + (100 - pct) + '%';
        playground.style.gridTemplateColumns = '1fr';
      } else {
        var offsetX = clientX - rect.left;
        var totalWidth = rect.width;
        var pct = (offsetX / totalWidth) * 100;
        pct = Math.max(15, Math.min(85, pct));
        playground.style.gridTemplateColumns = pct + '% 4px ' + (100 - pct) + '%';
        playground.style.gridTemplateRows = '';
      }

      editor.refresh();
    }

    function stopDrag() {
      isDragging = false;
      divider.classList.remove('dragging');
      document.body.style.cursor = '';
      document.body.style.userSelect = '';
      document.removeEventListener('mousemove', onDrag);
      document.removeEventListener('touchmove', onDrag);
      document.removeEventListener('mouseup', stopDrag);
      document.removeEventListener('touchend', stopDrag);
      editor.refresh();
    }
  })();

  // ---- Toast ----

  var toastTimer = null;
  function showToast(message) {
    var toast = document.getElementById('toast');
    toast.textContent = message;
    toast.classList.add('visible');
    clearTimeout(toastTimer);
    toastTimer = setTimeout(function () {
      toast.classList.remove('visible');
    }, 2000);
  }

  // ---- Initialize ----

  if (!loadFromHash()) {
    editor.setValue(EXAMPLES.counter);
  }

  // Trigger initial compile after a brief delay for CodeMirror to settle
  setTimeout(function () {
    updatePreview();
    editor.refresh();
    editor.focus();
  }, 100);

  // ---- AI Chat Panel ----

  var NEWT_SYSTEM_PROMPT = 'You are a Newt UI code generator. Newt is a declarative UI language.\n\n' +
    'CRITICAL RULES:\n' +
    '- Output ONLY valid .newt code, no explanation, no markdown fences\n' +
    '- Always wrap in a screen block: screen Main { ... }\n' +
    '- State declarations end with semicolons: state count = 0;\n' +
    '- Use call syntax: element(props)(children)\n' +
    '- When passing a component parameter as first arg with named props, use block syntax: text { content: label, fontSize: 14 }\n' +
    '- Available semantic tokens: primary, secondary, danger, success, warning, muted, ghost, bold, heading, subheading, caption, rounded, compact, comfortable, spacious, elevated\n' +
    '- 73 elements: header, footer, container, sidebar, section, row, column, stack, center, box, widget, card, grid, spacer, image, button, input, text, icon, tag, badge, modal, toast, alert, tooltip, loader, progressBar, tabs, nav, accordion, breadcrumb, dropdown, checkbox, radio, toggle, slider, stepper, form, carousel, chart, table, avatar, skeleton, drawer, select, textarea, separator, timeline, rating, treeView, splitter, and more\n' +
    '- Common props: fill, stroke, radius, padding, gap, fontSize, fontWeight, width, height, shadow, onClick, placeholder, content\n\n' +
    'Example:\nstate count = 0;\n\nscreen Counter {\n    column(gap: 16, padding: 32)(\n        text("Count: {count}", heading)\n        button("+1", primary, rounded, onClick: { count = count + 1 })\n    )\n}';

  var aiPanel = document.getElementById('ai-panel');
  var aiToggle = document.getElementById('ai-toggle');
  var aiClose = document.getElementById('ai-close');
  var aiKey = document.getElementById('ai-key');
  var aiPrompt = document.getElementById('ai-prompt');
  var aiSend = document.getElementById('ai-send');
  var aiMessages = document.getElementById('ai-messages');

  // Load saved API key
  var savedKey = localStorage.getItem('newt-ai-key') || '';
  if (savedKey) {
    aiKey.value = savedKey;
    aiKey.classList.add('has-key');
  }

  aiKey.addEventListener('input', function () {
    var key = aiKey.value.trim();
    localStorage.setItem('newt-ai-key', key);
    aiKey.classList.toggle('has-key', key.length > 10);
  });

  aiToggle.addEventListener('click', function () {
    aiPanel.classList.remove('hidden');
    aiPrompt.focus();
  });

  aiClose.addEventListener('click', function () {
    aiPanel.classList.add('hidden');
  });

  function addMessage(text, role) {
    var div = document.createElement('div');
    div.className = 'ai-message ' + role;
    div.textContent = text;
    aiMessages.appendChild(div);
    aiMessages.scrollTop = aiMessages.scrollHeight;
    return div;
  }

  function addLoading() {
    var div = document.createElement('div');
    div.className = 'ai-loading';
    div.innerHTML = '<span></span><span></span><span></span>';
    aiMessages.appendChild(div);
    aiMessages.scrollTop = aiMessages.scrollHeight;
    return div;
  }

  async function generateWithAI(prompt) {
    var key = aiKey.value.trim();
    if (!key) {
      addMessage('Please enter your Anthropic API key above.', 'error');
      aiKey.focus();
      return;
    }

    addMessage(prompt, 'user');
    var loading = addLoading();
    aiSend.disabled = true;
    aiPrompt.disabled = true;

    try {
      var resp = await fetch('https://api.anthropic.com/v1/messages', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-api-key': key,
          'anthropic-version': '2023-06-01',
          'anthropic-dangerous-direct-browser-access': 'true',
        },
        body: JSON.stringify({
          model: 'claude-haiku-4-5-20251001',
          max_tokens: 2048,
          system: NEWT_SYSTEM_PROMPT,
          messages: [{ role: 'user', content: prompt }],
        }),
      });

      loading.remove();

      if (!resp.ok) {
        var errData = await resp.json().catch(function () { return {}; });
        throw new Error(errData.error?.message || 'API error: ' + resp.status);
      }

      var data = await resp.json();
      var code = data.content[0].text;

      // Strip markdown fences if model includes them
      code = code.replace(/^```newt?\n?/gm, '').replace(/```$/gm, '').trim();

      // Validate with WASM if available
      var wasm = window.__newtWasm;
      if (wasm) {
        try {
          wasm.check_syntax(code);
          addMessage('Generated valid Newt code!', 'assistant');
        } catch (e) {
          addMessage('Generated code (may have issues: ' + String(e).substring(0, 80) + ')', 'assistant');
        }
      } else {
        addMessage('Generated code (WASM not loaded for validation)', 'assistant');
      }

      // Set code in editor
      editor.setValue(code);
      updatePreview();

    } catch (e) {
      loading.remove();
      addMessage(String(e.message || e), 'error');
    } finally {
      aiSend.disabled = false;
      aiPrompt.disabled = false;
      aiPrompt.value = '';
      aiPrompt.focus();
    }
  }

  aiSend.addEventListener('click', function () {
    var prompt = aiPrompt.value.trim();
    if (prompt) generateWithAI(prompt);
  });

  aiPrompt.addEventListener('keydown', function (e) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      var prompt = aiPrompt.value.trim();
      if (prompt) generateWithAI(prompt);
    }
  });

})();
