import {useState, useEffect, useCallback, useRef} from 'react';

export default function useSearchPalette() {
  const [isOpen, setIsOpen] = useState(false);
  const [isClosing, setIsClosing] = useState(false);
  const closeTimerRef = useRef(null);

  const open = useCallback(() => {
    if (closeTimerRef.current) {
      clearTimeout(closeTimerRef.current);
      closeTimerRef.current = null;
    }
    setIsClosing(false);
    setIsOpen(true);
    document.documentElement.style.overflow = 'hidden';
  }, []);

  const close = useCallback(() => {
    setIsClosing(true);
    closeTimerRef.current = setTimeout(() => {
      setIsOpen(false);
      setIsClosing(false);
      document.documentElement.style.overflow = '';
      closeTimerRef.current = null;
    }, 100);
  }, []);

  const toggle = useCallback(() => {
    if (isOpen) close();
    else open();
  }, [isOpen, open, close]);

  useEffect(() => {
    function handleKeyDown(e) {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        toggle();
      }
      if (e.key === 'Escape' && isOpen) {
        e.preventDefault();
        close();
      }
    }

    function handleOpenSearch() {
      open();
    }

    document.addEventListener('keydown', handleKeyDown);
    document.addEventListener('openSearch', handleOpenSearch);

    return () => {
      document.removeEventListener('keydown', handleKeyDown);
      document.removeEventListener('openSearch', handleOpenSearch);
    };
  }, [isOpen, toggle, open, close]);

  useEffect(() => {
    return () => {
      if (closeTimerRef.current) {
        clearTimeout(closeTimerRef.current);
      }
    };
  }, []);

  return {isOpen, isClosing, open, close, toggle};
}
