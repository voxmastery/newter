import React, { useState, useCallback } from 'react';
import styles from './styles.module.css';

const examples = {
  Dashboard: `let accent = #2563eb;

component StatCard(label, value, color) {
    card(fill: #ffffff, stroke: #e5e7eb, radius: 12, padding: 20)(
        column(gap: 8)(
            text { content: label, fontSize: 12 }
            text { content: value, fontSize: 28, fontWeight: "700" }
        )
    )
}

screen Dashboard {
    column(fill: #f9fafb)(
        header(fill: #ffffff, stroke: #e5e7eb, padding: 16)(
            row(gap: 12)(
                text("Dashboard", fontSize: 20, fontWeight: "700")
                spacer()
                button("Settings", stroke: #e5e7eb, radius: 8)
            )
        )
    )
}`,
  Counter: `state count = 0;

screen Counter {
    column(gap: 24, padding: 48, fill: #f9fafb)(
        text("Counter", fontSize: 32, fontWeight: "700")
        text("Current value: {count}", fontSize: 18)
        row(gap: 12)(
            button("+ Add", fill: #2563eb, radius: 8, onClick: { count = count + 1 })
            button("- Remove", fill: #6b7280, radius: 8, onClick: { count = count - 1 })
            button("Reset", fill: #ef4444, radius: 8, onClick: { count = 0 })
        )
    )
}`,
  Form: `state submitted = false;

screen ContactForm {
    center(fill: #f9fafb)(
        card(fill: #ffffff, stroke: #e5e7eb, radius: 16, padding: 32)(
            column(gap: 20)(
                text("Contact Us", fontSize: 24, fontWeight: "700")
                column(gap: 8)(
                    text("Name", fontSize: 14, fontWeight: "500")
                    input(stroke: #d1d5db, radius: 8, padding: 12)
                )
                button("Send Message", fill: #2563eb, radius: 8, fontSize: 16)
            )
        )
    )
}`,
};

const tabNames = Object.keys(examples);

export default function HomepageCodeShowcase() {
  const [activeTab, setActiveTab] = useState('Dashboard');
  const [isFading, setIsFading] = useState(false);

  const handleTabClick = useCallback((name) => {
    if (name === activeTab) return;
    setIsFading(true);
    setTimeout(() => {
      setActiveTab(name);
      setIsFading(false);
    }, 100);
  }, [activeTab]);

  return (
    <section className={styles.showcase}>
      <div className={styles.container}>
        <h2 className={styles.sectionTitle}>See Newt in action</h2>
        <p className={styles.sectionSubtitle}>
          Real examples you can copy and run right now.
        </p>
        <div className={styles.tabs}>
          {tabNames.map((name) => (
            <button
              key={name}
              className={`${styles.tab} ${activeTab === name ? styles.tabActive : ''}`}
              onClick={() => handleTabClick(name)}
            >
              {name}
            </button>
          ))}
        </div>
        <div className={styles.codeWrapper}>
          <div className={`${styles.codeContent} ${isFading ? styles.codeContentFading : ''}`}>
            <div className={styles.codeBlock}>
              <pre><code>{examples[activeTab]}</code></pre>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
