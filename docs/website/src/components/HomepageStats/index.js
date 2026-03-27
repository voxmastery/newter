import React from 'react';
import styles from './styles.module.css';

const stats = [
  { value: '120+', label: 'GitHub Stars' },
  { value: '500+', label: 'VS Code Installs' },
  { value: '58', label: 'Built-in Elements' },
  { value: '300+', label: 'crates.io Downloads' },
];

export default function HomepageStats() {
  return (
    <section className={styles.stats}>
      <div className={styles.container}>
        {stats.map((stat, idx) => (
          <React.Fragment key={idx}>
            {idx > 0 && <div className={styles.divider} />}
            <div className={styles.stat}>
              <div className={styles.statValue}>{stat.value}</div>
              <div className={styles.statLabel}>{stat.label}</div>
            </div>
          </React.Fragment>
        ))}
      </div>
    </section>
  );
}
