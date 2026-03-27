import React, {useCallback} from 'react';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';

const styles = {
  container: {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    minHeight: '60vh',
    padding: '48px 24px',
    textAlign: 'center',
  },
  code: {
    fontSize: '120px',
    fontWeight: 800,
    lineHeight: 1,
    marginBottom: '16px',
    background: 'linear-gradient(135deg, #7c3aed, #a78bfa)',
    WebkitBackgroundClip: 'text',
    WebkitTextFillColor: 'transparent',
    backgroundClip: 'text',
    letterSpacing: '-0.04em',
  },
  title: {
    fontSize: '28px',
    fontWeight: 700,
    color: 'var(--color-text)',
    margin: '0 0 12px',
  },
  message: {
    fontSize: '16px',
    color: 'var(--color-text-secondary)',
    margin: '0 0 32px',
    maxWidth: '400px',
  },
  buttons: {
    display: 'flex',
    gap: '12px',
    flexWrap: 'wrap',
    justifyContent: 'center',
  },
  primaryBtn: {
    display: 'inline-flex',
    alignItems: 'center',
    padding: '12px 24px',
    background: '#7c3aed',
    color: '#ffffff',
    fontSize: '15px',
    fontWeight: 600,
    borderRadius: '10px',
    textDecoration: 'none',
    transition: 'background 150ms ease, transform 150ms ease',
  },
  secondaryBtn: {
    display: 'inline-flex',
    alignItems: 'center',
    padding: '12px 24px',
    background: 'transparent',
    color: 'var(--color-text)',
    fontSize: '15px',
    fontWeight: 600,
    border: '1px solid var(--color-border-strong)',
    borderRadius: '10px',
    cursor: 'pointer',
    fontFamily: 'var(--font-sans)',
    transition: 'background 150ms ease',
  },
};

export default function NotFound() {
  const openSearch = useCallback(() => {
    document.dispatchEvent(new CustomEvent('openSearch'));
  }, []);

  return (
    <Layout title="Page not found">
      <main style={styles.container}>
        <div style={styles.code}>404</div>
        <h1 style={styles.title}>Page not found</h1>
        <p style={styles.message}>
          The page you're looking for doesn't exist or has been moved.
          Try searching the docs or head back to the homepage.
        </p>
        <div style={styles.buttons}>
          <Link style={styles.primaryBtn} to="/">
            Go Home
          </Link>
          <button style={styles.secondaryBtn} onClick={openSearch} type="button">
            Search Docs
          </button>
        </div>
      </main>
    </Layout>
  );
}
