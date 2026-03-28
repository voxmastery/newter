import React from 'react';
import Link from '@docusaurus/Link';
import TypewriterCode from '@site/src/components/TypewriterCode';
import styles from './styles.module.css';

const newtCode = `screen Main {
  column(gap: 16, padding: 24) {
    text("Hello Newt!", fontSize: 24)
    row(gap: 8) {
      button("Click me", fill: #7c3aed)
      button("Cancel", stroke: #e5e7eb)
    }
  }
}`;

const reactCode = `function App() {
  return (
    <div style={{display:'flex', flexDirection:'column',
      gap:'16px', padding:'24px'}}>
      <h1 style={{fontSize:'24px'}}>Hello Newt!</h1>
      <div style={{display:'flex', gap:'8px'}}>
        <button style={{background:'#7c3aed',
          color:'#fff'}}>Click me</button>
        <button style={{border:'1px solid #e5e7eb'}}>
          Cancel</button>
      </div>
    </div>
  );
}`;

export default function HomepageHero() {
  return (
    <section className={styles.hero}>
      <div className={styles.container}>
        <div className={styles.badge}>
          <span>v0.1</span>
          <span>&middot;</span>
          <span>73 built-in elements</span>
        </div>
        <h1 className={styles.headline}>
          Describe UIs{' '}
          <span className={styles.headlineAccent}>faster</span> than
          <br />any other language
        </h1>
        <p className={styles.subheading}>
          Newt compiles to canvas, HTML, and JSON — from a syntax you can learn in 5 minutes.
        </p>
        <div className={styles.ctas}>
          <Link className={styles.ctaPrimary} to="/docs/getting-started">
            Get Started
          </Link>
          <Link className={styles.ctaSecondary} to="/docs/examples">
            View Examples
          </Link>
        </div>
        <TypewriterCode />
        <div className={styles.comparison}>
          <div className={styles.comparisonGrid}>
            <div className={styles.codePanel}>
              <div className={styles.codePanelLabel}>Newt — 8 lines</div>
              <div className={styles.codeBlock}>
                <pre><code>{newtCode}</code></pre>
              </div>
            </div>
            <div className={styles.codePanel}>
              <div className={styles.codePanelLabel}>React — 14 lines</div>
              <div className={styles.codeBlock}>
                <pre><code>{reactCode}</code></pre>
              </div>
            </div>
          </div>
          <p className={styles.caption}>Same UI. Less code. No build step.</p>
        </div>
      </div>
    </section>
  );
}
