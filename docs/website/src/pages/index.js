import React from 'react';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import HomepageHero from '@site/src/components/HomepageHero';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import HomepageCodeShowcase from '@site/src/components/HomepageCodeShowcase';
import HomepageInstall from '@site/src/components/HomepageInstall';
import HomepageStats from '@site/src/components/HomepageStats';
import styles from './index.module.css';

export default function Home() {
  const {siteConfig} = useDocusaurusContext();

  return (
    <Layout
      title="Newt — A UI description language"
      description="Describe UIs faster than any other language. Newt compiles to canvas, HTML, and JSON."
    >
      <main className={styles.homepage}>
        <HomepageHero />
        <HomepageFeatures />
        <HomepageCodeShowcase />
        <HomepageInstall />
        <HomepageStats />
      </main>
    </Layout>
  );
}
