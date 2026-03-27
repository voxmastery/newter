import React, {useState, useEffect, useRef, useCallback} from 'react';
import {createPortal} from 'react-dom';
import {useHistory} from '@docusaurus/router';
import useSearchPalette from '@site/src/utils/useSearchPalette';
import styles from './styles.module.css';

const RECENT_SEARCHES_KEY = 'newt-docs-recent-searches';
const MAX_RECENT = 5;

const SUGGESTED_PAGES = [
  {title: 'Getting Started', path: '/docs/getting-started', section: 'Introduction'},
  {title: 'Elements', path: '/docs/language/elements', section: 'Language'},
  {title: 'State', path: '/docs/language/state', section: 'Language'},
  {title: 'CLI Reference', path: '/docs/compiler/cli', section: 'Compiler'},
];

function getRecentSearches() {
  try {
    const stored = localStorage.getItem(RECENT_SEARCHES_KEY);
    return stored ? JSON.parse(stored) : [];
  } catch {
    return [];
  }
}

function addRecentSearch(item) {
  try {
    const recent = getRecentSearches().filter((r) => r.path !== item.path);
    recent.unshift(item);
    localStorage.setItem(
      RECENT_SEARCHES_KEY,
      JSON.stringify(recent.slice(0, MAX_RECENT)),
    );
  } catch {
    // localStorage unavailable
  }
}

function highlightMatch(text, query) {
  if (!query || query.length < 2) return text;
  const regex = new RegExp(`(${query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi');
  const parts = text.split(regex);
  return parts.map((part, i) =>
    regex.test(part) ? (
      <mark key={i} className={styles.highlight}>
        {part}
      </mark>
    ) : (
      part
    ),
  );
}

export default function SearchPalette() {
  const {isOpen, isClosing, close} = useSearchPalette();
  const [query, setQuery] = useState('');
  const [results, setResults] = useState([]);
  const [activeIndex, setActiveIndex] = useState(0);
  const [searchIndex, setSearchIndex] = useState(null);
  const inputRef = useRef(null);
  const resultsRef = useRef(null);
  const history = useHistory();

  // Load search index
  useEffect(() => {
    if (isOpen && !searchIndex) {
      import('@easyops-cn/docusaurus-search-local/dist/client/client/utils/proxiedGenerated')
        .then((module) => {
          setSearchIndex(module);
        })
        .catch(() => {
          // Search index not available during dev
        });
    }
  }, [isOpen, searchIndex]);

  // Focus input when opened
  useEffect(() => {
    if (isOpen) {
      setQuery('');
      setResults([]);
      setActiveIndex(0);
      setTimeout(() => inputRef.current?.focus(), 50);
    }
  }, [isOpen]);

  // Search
  useEffect(() => {
    if (!query || query.trim().length < 2) {
      setResults([]);
      setActiveIndex(0);
      return;
    }

    if (searchIndex?.searchDocs) {
      try {
        const searchResults = searchIndex.searchDocs(query.trim());
        const formatted = searchResults.slice(0, 8).map((r) => ({
          title: r.document?.sectionTitle || r.document?.pageTitle || r.title || 'Untitled',
          path: r.document?.sectionRoute || r.document?.pageRoute || r.url || '#',
          section: r.document?.pageTitle || '',
          snippet: r.document?.highlight || '',
        }));
        setResults(formatted);
        setActiveIndex(0);
      } catch {
        setResults([]);
      }
    }
  }, [query, searchIndex]);

  const navigateTo = useCallback(
    (path, title) => {
      addRecentSearch({title, path});
      close();
      history.push(path);
    },
    [close, history],
  );

  const handleKeyDown = useCallback(
    (e) => {
      const items = results.length > 0 ? results : query.length < 2 ? [...getRecentSearches(), ...SUGGESTED_PAGES] : [];

      if (e.key === 'ArrowDown') {
        e.preventDefault();
        setActiveIndex((prev) => Math.min(prev + 1, items.length - 1));
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        setActiveIndex((prev) => Math.max(prev - 1, 0));
      } else if (e.key === 'Enter' && items[activeIndex]) {
        e.preventDefault();
        navigateTo(items[activeIndex].path, items[activeIndex].title);
      }
    },
    [results, activeIndex, query, navigateTo],
  );

  // Focus trap for accessibility
  const handlePaletteKeyDown = useCallback((e) => {
    if (e.key === 'Tab') {
      const focusable = e.currentTarget.querySelectorAll(
        'input, button, [tabindex]:not([tabindex="-1"])',
      );
      if (focusable.length === 0) return;
      const first = focusable[0];
      const last = focusable[focusable.length - 1];
      if (e.shiftKey && document.activeElement === first) {
        e.preventDefault();
        last.focus();
      } else if (!e.shiftKey && document.activeElement === last) {
        e.preventDefault();
        first.focus();
      }
    }
  }, []);

  // Scroll active item into view
  useEffect(() => {
    if (resultsRef.current) {
      const active = resultsRef.current.querySelector(`.${styles.resultItemActive}`);
      active?.scrollIntoView({block: 'nearest'});
    }
  }, [activeIndex]);

  if (!isOpen && !isClosing) return null;

  if (typeof document === 'undefined') return null;

  const recentSearches = getRecentSearches();
  const showRecent = query.length < 2 && recentSearches.length > 0;
  const showSuggested = query.length < 2;
  const showResults = query.length >= 2 && results.length > 0;
  const showEmpty = query.length >= 2 && results.length === 0;

  return createPortal(
    <div className={styles.overlay} onClick={close} data-state={isClosing ? 'closing' : 'open'}>
      <div className={styles.palette} onClick={(e) => e.stopPropagation()} onKeyDown={handlePaletteKeyDown} role="dialog" aria-modal="true" aria-label="Search documentation">
        {/* Input */}
        <div className={styles.inputWrapper}>
          <svg className={styles.inputIcon} viewBox="0 0 20 20" fill="none" width="18" height="18">
            <path d="M9 17A8 8 0 1 0 9 1a8 8 0 0 0 0 16z" stroke="currentColor" strokeWidth="1.5"/>
            <path d="m19 19-4.35-4.35" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/>
          </svg>
          <input
            ref={inputRef}
            className={styles.input}
            type="text"
            placeholder="Search documentation..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={handleKeyDown}
            aria-label="Search documentation"
          />
          <kbd className={styles.escBadge}>ESC</kbd>
        </div>

        {/* Results */}
        <div className={styles.results} ref={resultsRef}>
          {showRecent && (
            <div className={styles.group}>
              <div className={styles.groupHeader}>Recent</div>
              {recentSearches.map((item, i) => (
                <button
                  key={item.path}
                  className={`${styles.resultItem} ${i === activeIndex ? styles.resultItemActive : ''}`}
                  onClick={() => navigateTo(item.path, item.title)}
                  type="button"
                >
                  <svg className={styles.resultIcon} viewBox="0 0 16 16" fill="none" width="14" height="14">
                    <path d="M8 14A6 6 0 1 0 8 2a6 6 0 0 0 0 12z" stroke="currentColor" strokeWidth="1.2"/>
                    <path d="M8 5v3l2 2" stroke="currentColor" strokeWidth="1.2" strokeLinecap="round"/>
                  </svg>
                  <span className={styles.resultTitle}>{item.title}</span>
                </button>
              ))}
            </div>
          )}

          {showSuggested && (
            <div className={styles.group}>
              <div className={styles.groupHeader}>Suggested</div>
              {SUGGESTED_PAGES.map((item, i) => {
                const idx = showRecent ? recentSearches.length + i : i;
                return (
                  <button
                    key={item.path}
                    className={`${styles.resultItem} ${idx === activeIndex ? styles.resultItemActive : ''}`}
                    onClick={() => navigateTo(item.path, item.title)}
                    type="button"
                  >
                    <svg className={styles.resultIcon} viewBox="0 0 16 16" fill="none" width="14" height="14">
                      <path d="M3 2h10v12H3z" stroke="currentColor" strokeWidth="1.2"/>
                      <path d="M5 5h6M5 8h4" stroke="currentColor" strokeWidth="1.2" strokeLinecap="round"/>
                    </svg>
                    <div className={styles.resultContent}>
                      <span className={styles.resultTitle}>{item.title}</span>
                      <span className={styles.resultSection}>{item.section}</span>
                    </div>
                  </button>
                );
              })}
            </div>
          )}

          {showResults && (() => {
            const groups = {};
            results.forEach((item) => {
              const key = item.section || 'Results';
              if (!groups[key]) groups[key] = [];
              groups[key].push(item);
            });
            let idx = 0;
            return Object.entries(groups).map(([groupTitle, items]) => (
              <div className={styles.group} key={groupTitle}>
                <div className={styles.groupHeader}>{groupTitle}</div>
                {items.map((item) => {
                  const currentIdx = idx++;
                  return (
                    <button
                      key={`${item.path}-${currentIdx}`}
                      className={`${styles.resultItem} ${currentIdx === activeIndex ? styles.resultItemActive : ''}`}
                      onClick={() => navigateTo(item.path, item.title)}
                      type="button"
                    >
                      <svg className={styles.resultIcon} viewBox="0 0 16 16" fill="none" width="14" height="14">
                        <path d="M3 2h10v12H3z" stroke="currentColor" strokeWidth="1.2"/>
                        <path d="M5 5h6M5 8h4" stroke="currentColor" strokeWidth="1.2" strokeLinecap="round"/>
                      </svg>
                      <div className={styles.resultContent}>
                        <span className={styles.resultTitle}>
                          {highlightMatch(item.title, query)}
                        </span>
                        {item.snippet && (
                          <span className={styles.resultSnippet}>
                            {highlightMatch(item.snippet, query)}
                          </span>
                        )}
                      </div>
                    </button>
                  );
                })}
              </div>
            ));
          })()}

          {showEmpty && (
            <div className={styles.emptyState}>
              No results for &ldquo;{query}&rdquo;
            </div>
          )}
        </div>

        {/* Footer */}
        <div className={styles.footer}>
          <div className={styles.footerShortcuts}>
            <span><kbd>↑↓</kbd> Navigate</span>
            <span><kbd>↵</kbd> Open</span>
            <span><kbd>ESC</kbd> Close</span>
          </div>
          <span className={styles.footerBrand}>newt</span>
        </div>
      </div>
    </div>,
    document.body,
  );
}
