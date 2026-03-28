import React from "react";
import {
  AbsoluteFill,
  useCurrentFrame,
  useVideoConfig,
  interpolate,
  spring,
} from "remotion";
import { COLORS, FONTS } from "../styles";

const NEWT_CODE = `state count = 0

screen Counter {
    column(gap: 24, padding: 32)(
        text("Count: {count}", fontSize: 32)
        row(gap: 12)(
            button("+1", fill: #2563eb, radius: 8, onClick: { count = count + 1 })
            button("Reset", fill: #ef4444, radius: 8, onClick: { count = 0 })
        )
    )
}`;

const CHARS_PER_FRAME = 2.5;

// Syntax highlighting tokens
const highlightCode = (code: string): React.ReactNode[] => {
  const keywords = ["state", "screen", "column", "row", "text", "button"];
  const result: React.ReactNode[] = [];
  let i = 0;
  let key = 0;

  while (i < code.length) {
    // Check for keywords
    let matched = false;
    for (const kw of keywords) {
      if (code.substring(i, i + kw.length) === kw) {
        const after = code[i + kw.length];
        if (!after || /[\s({,]/.test(after)) {
          result.push(
            <span key={key++} style={{ color: COLORS.violet }}>
              {kw}
            </span>
          );
          i += kw.length;
          matched = true;
          break;
        }
      }
    }
    if (matched) continue;

    // Check for strings
    if (code[i] === '"') {
      let end = i + 1;
      while (end < code.length && code[end] !== '"') end++;
      const str = code.substring(i, end + 1);
      result.push(
        <span key={key++} style={{ color: COLORS.green }}>
          {str}
        </span>
      );
      i = end + 1;
      continue;
    }

    // Check for numbers
    if (/\d/.test(code[i]) && (i === 0 || /[\s:=,({]/.test(code[i - 1]))) {
      let end = i;
      while (end < code.length && /\d/.test(code[end])) end++;
      const num = code.substring(i, end);
      result.push(
        <span key={key++} style={{ color: COLORS.orange }}>
          {num}
        </span>
      );
      i = end;
      continue;
    }

    // Check for hex colors (#xxxxxx)
    if (code[i] === "#" && /[0-9a-fA-F]/.test(code[i + 1] || "")) {
      let end = i + 1;
      while (end < code.length && /[0-9a-fA-F]/.test(code[end])) end++;
      const hex = code.substring(i, end);
      result.push(
        <span key={key++} style={{ color: hex }}>
          {hex}
        </span>
      );
      i = end;
      continue;
    }

    // Check for property names (word followed by colon)
    if (/[a-zA-Z]/.test(code[i])) {
      let end = i;
      while (end < code.length && /[a-zA-Z_]/.test(code[end])) end++;
      const word = code.substring(i, end);
      const afterWord = code.substring(end).trimStart();
      if (afterWord[0] === ":") {
        result.push(
          <span key={key++} style={{ color: COLORS.blue }}>
            {word}
          </span>
        );
      } else {
        result.push(<span key={key++}>{word}</span>);
      }
      i = end;
      continue;
    }

    // Default: single character
    result.push(<span key={key++}>{code[i]}</span>);
    i++;
  }

  return result;
};

const TrafficLights: React.FC = () => (
  <div style={{ display: "flex", gap: 8 }}>
    <div
      style={{
        width: 12,
        height: 12,
        borderRadius: "50%",
        backgroundColor: "#ef4444",
      }}
    />
    <div
      style={{
        width: 12,
        height: 12,
        borderRadius: "50%",
        backgroundColor: "#facc15",
      }}
    />
    <div
      style={{
        width: 12,
        height: 12,
        borderRadius: "50%",
        backgroundColor: "#22c55e",
      }}
    />
  </div>
);

const RenderedPreview: React.FC = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "column",
      alignItems: "center",
      gap: 24,
      padding: 32,
    }}
  >
    <div
      style={{
        fontSize: 32,
        color: COLORS.text,
        fontFamily: FONTS.sans,
        fontWeight: 500,
      }}
    >
      Count: 0
    </div>
    <div style={{ display: "flex", gap: 12 }}>
      <div
        style={{
          padding: "12px 28px",
          backgroundColor: COLORS.blue,
          color: "#fff",
          borderRadius: 8,
          fontSize: 18,
          fontFamily: FONTS.sans,
          fontWeight: 600,
        }}
      >
        +1
      </div>
      <div
        style={{
          padding: "12px 28px",
          backgroundColor: COLORS.red,
          color: "#fff",
          borderRadius: 8,
          fontSize: 18,
          fontFamily: FONTS.sans,
          fontWeight: 600,
        }}
      >
        Reset
      </div>
    </div>
  </div>
);

export const CodeTypewriter: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps, durationInFrames } = useVideoConfig();

  const charsToShow = Math.min(
    Math.floor(frame * CHARS_PER_FRAME),
    NEWT_CODE.length
  );
  const visibleCode = NEWT_CODE.substring(0, charsToShow);
  const typingDone = charsToShow >= NEWT_CODE.length;

  // Blinking cursor
  const cursorVisible = Math.floor(frame / 15) % 2 === 0;

  // Right panel fades in after typing completes
  const typingEndFrame = Math.ceil(NEWT_CODE.length / CHARS_PER_FRAME);
  const previewOpacity = interpolate(
    frame,
    [typingEndFrame + 10, typingEndFrame + 40],
    [0, 1],
    {
      extrapolateLeft: "clamp",
      extrapolateRight: "clamp",
    }
  );

  // Panel slide-in
  const panelSlide = spring({
    frame,
    fps,
    config: { damping: 15, stiffness: 60, mass: 0.8 },
  });

  return (
    <AbsoluteFill
      style={{
        justifyContent: "center",
        alignItems: "center",
        padding: 80,
      }}
    >
      <div
        style={{
          display: "flex",
          width: "100%",
          maxWidth: 1600,
          gap: 40,
          transform: `translateY(${interpolate(panelSlide, [0, 1], [40, 0])}px)`,
          opacity: panelSlide,
        }}
      >
        {/* Left panel: Code editor */}
        <div
          style={{
            flex: 1,
            backgroundColor: COLORS.codeBg,
            borderRadius: 16,
            overflow: "hidden",
            border: "1px solid #2a2a2a",
          }}
        >
          {/* Editor header */}
          <div
            style={{
              display: "flex",
              alignItems: "center",
              gap: 16,
              padding: "16px 20px",
              backgroundColor: COLORS.codeHeader,
              borderBottom: "1px solid #2a2a2a",
            }}
          >
            <TrafficLights />
            <span
              style={{
                fontSize: 14,
                color: COLORS.textMuted,
                fontFamily: FONTS.mono,
              }}
            >
              counter.newt
            </span>
          </div>

          {/* Code area */}
          <div
            style={{
              padding: "24px 28px",
              fontFamily: FONTS.mono,
              fontSize: 18,
              lineHeight: 1.7,
              color: COLORS.text,
              whiteSpace: "pre",
              minHeight: 380,
            }}
          >
            {highlightCode(visibleCode)}
            {cursorVisible && (
              <span
                style={{
                  display: "inline-block",
                  width: 2,
                  height: 22,
                  backgroundColor: COLORS.violet,
                  marginLeft: 1,
                  verticalAlign: "text-bottom",
                }}
              />
            )}
          </div>
        </div>

        {/* Right panel: Rendered output */}
        <div
          style={{
            flex: 0.65,
            backgroundColor: COLORS.cardBg,
            borderRadius: 16,
            border: "1px solid #2a2a2a",
            display: "flex",
            flexDirection: "column",
            justifyContent: "center",
            alignItems: "center",
            opacity: previewOpacity,
          }}
        >
          <div
            style={{
              fontSize: 13,
              color: COLORS.textMuted,
              fontFamily: FONTS.mono,
              marginBottom: 16,
              textTransform: "uppercase",
              letterSpacing: "0.1em",
            }}
          >
            Preview
          </div>
          <RenderedPreview />
        </div>
      </div>
    </AbsoluteFill>
  );
};
