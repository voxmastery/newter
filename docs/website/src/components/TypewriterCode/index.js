import React, {useState, useEffect, useRef, useCallback} from 'react';
import styles from './styles.module.css';

const examples = [
  `screen Hello {
    column(gap: 16, padding: 24)(
        text("Hello Newt!", fontSize: 24)
        button("Get Started", fill: #7c3aed, radius: 8)
    )
}`,
  `state count = 0

screen Counter {
    column(gap: 16, padding: 32)(
        text("Count: {count}", fontSize: 28)
        button("+1", fill: #2563eb, onClick: { count = count + 1 })
    )
}`,
  `component Card(title, body) {
    card(fill: #ffffff, stroke: #e5e7eb, radius: 12, padding: 20)(
        column(gap: 8)(
            text(title, fontSize: 16, fontWeight: "700")
            text(body, fontSize: 14)
        )
    )
}`,
];

const CHAR_DELAY = 40;
const PAUSE_BETWEEN = 3000;

export default function TypewriterCode() {
  const [exampleIndex, setExampleIndex] = useState(0);
  const [charIndex, setCharIndex] = useState(0);
  const [isPaused, setIsPaused] = useState(false);
  const timerRef = useRef(null);

  const source = examples[exampleIndex];

  useEffect(() => {
    if (isPaused) {
      timerRef.current = setTimeout(() => {
        const nextExample = (exampleIndex + 1) % examples.length;
        setExampleIndex(nextExample);
        setCharIndex(0);
        setIsPaused(false);
      }, PAUSE_BETWEEN);
      return () => clearTimeout(timerRef.current);
    }

    if (charIndex < source.length) {
      timerRef.current = setTimeout(() => {
        setCharIndex((prev) => prev + 1);
      }, CHAR_DELAY);
      return () => clearTimeout(timerRef.current);
    }

    // Finished typing current example — pause
    setIsPaused(true);
  }, [charIndex, isPaused, exampleIndex, source]);

  return (
    <div className={styles.typewriter}>
      <div className={styles.header}>
        <div className={styles.dots}>
          <span className={styles.dot} />
          <span className={styles.dot} />
          <span className={styles.dot} />
        </div>
        <span className={styles.filename}>example.newt</span>
      </div>
      <div className={styles.codeArea}>
        <pre className={styles.code}>
          <code>{source.slice(0, charIndex)}</code>
          <span className={styles.cursor} />
        </pre>
      </div>
    </div>
  );
}
