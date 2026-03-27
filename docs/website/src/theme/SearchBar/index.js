import React, {useCallback} from 'react';
import styles from './styles.module.css';

export default function SearchBar() {
  const handleOpen = useCallback(() => {
    document.dispatchEvent(new CustomEvent('openSearch'));
  }, []);

  return (
    <button className={styles.searchTrigger} onClick={handleOpen} type="button" aria-label="Search documentation (Cmd+K)">
      <svg className={styles.searchIcon} viewBox="0 0 20 20" fill="none" width="16" height="16">
        <path d="M9 17A8 8 0 1 0 9 1a8 8 0 0 0 0 16z" stroke="currentColor" strokeWidth="1.5"/>
        <path d="m19 19-4.35-4.35" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/>
      </svg>
      <span className={styles.searchLabel}>Search docs...</span>
      <kbd className={styles.kbdBadge}>
        <span className={styles.kbdMeta}>⌘</span>K
      </kbd>
    </button>
  );
}
