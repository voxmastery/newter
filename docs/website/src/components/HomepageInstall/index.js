import React, { useState } from 'react';
import { Copy, Check } from 'lucide-react';
import styles from './styles.module.css';

const installCommands = {
  Cargo: 'cargo install newter-compiler',
  Binary:
    'curl -fsSL https://github.com/voxmastery/newter/releases/latest/download/newter-compiler-linux-x86_64 -o newter-compiler && chmod +x newter-compiler',
  'VS Code': 'code --install-extension voxmastery.newt',
};

const tabNames = Object.keys(installCommands);

export default function HomepageInstall() {
  const [activeTab, setActiveTab] = useState('Cargo');
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(installCommands[activeTab]);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      // Fallback: do nothing
    }
  };

  return (
    <section className={styles.install}>
      <div className={styles.container}>
        <h2 className={styles.sectionTitle}>Install in seconds</h2>
        <p className={styles.sectionSubtitle}>
          Pick your preferred method and start building.
        </p>
        <div className={styles.tabs}>
          {tabNames.map((name) => (
            <button
              key={name}
              className={`${styles.tab} ${activeTab === name ? styles.tabActive : ''}`}
              onClick={() => {
                setActiveTab(name);
                setCopied(false);
              }}
            >
              {name}
            </button>
          ))}
        </div>
        <div className={styles.codeWrapper}>
          <div className={styles.codeBlock}>
            <pre><code>{installCommands[activeTab]}</code></pre>
          </div>
          <button
            className={`${styles.copyButton} ${copied ? styles.copied : ''}`}
            onClick={handleCopy}
            aria-label="Copy to clipboard"
          >
            {copied ? <Check size={16} /> : <Copy size={16} />}
          </button>
        </div>
      </div>
    </section>
  );
}
