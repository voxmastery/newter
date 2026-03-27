import React from 'react';
import { LayoutGrid, Zap, Layers, Monitor, Code2, Palette } from 'lucide-react';
import styles from './styles.module.css';

const features = [
  {
    icon: LayoutGrid,
    title: '58 Built-in Elements',
    description:
      'From box and text to modal, chart, and carousel — every common UI element is a first-class citizen.',
  },
  {
    icon: Zap,
    title: 'Reactive State',
    description:
      'Declare state variables and wire onClick handlers. Newt re-renders automatically when state changes.',
  },
  {
    icon: Layers,
    title: 'Multi-Target Output',
    description:
      'One .newt file compiles to a GPU-rendered canvas, a static HTML page, or a JSON layout tree.',
  },
  {
    icon: Monitor,
    title: 'Canvas IDE',
    description:
      'Run newter-compiler serve for a live-reload browser preview. Edit your file, see changes instantly.',
  },
  {
    icon: Code2,
    title: 'VS Code Extension',
    description:
      'Full language support: syntax highlighting, completions, diagnostics, hover info, and go-to-definition.',
  },
  {
    icon: Palette,
    title: 'Themes & Components',
    description:
      'Define reusable components with parameters. Create theme palettes and apply them with use theme.',
  },
];

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className={styles.container}>
        <h2 className={styles.sectionTitle}>Everything you need, built in</h2>
        <p className={styles.sectionSubtitle}>
          No plugins, no config files, no dependency hell.
        </p>
        <div className={styles.grid}>
          {features.map((feature, idx) => {
            const Icon = feature.icon;
            return (
              <div key={idx} className={styles.card}>
                <div className={styles.iconWrapper}>
                  <Icon size={20} />
                </div>
                <h3 className={styles.cardTitle}>{feature.title}</h3>
                <p className={styles.cardDescription}>{feature.description}</p>
              </div>
            );
          })}
        </div>
      </div>
    </section>
  );
}
