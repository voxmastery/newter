import React from "react";
import {
  AbsoluteFill,
  useCurrentFrame,
  useVideoConfig,
  interpolate,
  spring,
} from "remotion";
import { COLORS, FONTS } from "../styles";

interface CardData {
  readonly stat: string;
  readonly title: string;
  readonly subtitle?: string;
}

const CARDS: readonly CardData[] = [
  { stat: "73", title: "Built-in Elements" },
  { stat: "3", title: "Output Targets", subtitle: "Canvas \u00B7 HTML \u00B7 JSON" },
  {
    stat: "100%",
    title: "Written in Rust",
    subtitle: "Fast. Safe. Reliable.",
  },
];

const FeatureCard: React.FC<{
  readonly card: CardData;
  readonly index: number;
}> = ({ card, index }) => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const delay = 20 + index * 12;

  const cardSpring = spring({
    frame: Math.max(0, frame - delay),
    fps,
    config: { damping: 14, stiffness: 80, mass: 0.7 },
  });

  const translateY = interpolate(cardSpring, [0, 1], [60, 0]);
  const opacity = cardSpring;

  return (
    <div
      style={{
        flex: 1,
        backgroundColor: COLORS.cardBg,
        borderRadius: 20,
        padding: "48px 36px",
        border: "1px solid #2a2a2a",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        textAlign: "center",
        gap: 12,
        transform: `translateY(${translateY}px)`,
        opacity,
      }}
    >
      <div
        style={{
          fontSize: 72,
          fontWeight: 800,
          color: COLORS.violet,
          fontFamily: FONTS.sans,
          lineHeight: 1,
        }}
      >
        {card.stat}
      </div>
      <div
        style={{
          fontSize: 24,
          fontWeight: 600,
          color: COLORS.text,
          fontFamily: FONTS.sans,
          marginTop: 8,
        }}
      >
        {card.title}
      </div>
      {card.subtitle && (
        <div
          style={{
            fontSize: 18,
            color: COLORS.textMuted,
            fontFamily: FONTS.sans,
            marginTop: 4,
          }}
        >
          {card.subtitle}
        </div>
      )}
    </div>
  );
};

export const FeatureCards: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  // Title animation
  const titleSpring = spring({
    frame,
    fps,
    config: { damping: 14, stiffness: 80, mass: 0.8 },
  });

  // Violet accent under title
  const accentWidth = interpolate(frame, [15, 45], [0, 120], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

  return (
    <AbsoluteFill
      style={{
        justifyContent: "center",
        alignItems: "center",
        padding: "80px 120px",
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          width: "100%",
          gap: 60,
        }}
      >
        {/* Title */}
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            gap: 12,
            opacity: titleSpring,
            transform: `translateY(${interpolate(titleSpring, [0, 1], [30, 0])}px)`,
          }}
        >
          <div
            style={{
              fontSize: 64,
              fontWeight: 800,
              color: COLORS.text,
              fontFamily: FONTS.sans,
            }}
          >
            Why Newt?
          </div>
          <div
            style={{
              width: accentWidth,
              height: 4,
              backgroundColor: COLORS.violet,
              borderRadius: 2,
            }}
          />
        </div>

        {/* Cards */}
        <div
          style={{
            display: "flex",
            gap: 32,
            width: "100%",
          }}
        >
          {CARDS.map((card, index) => (
            <FeatureCard key={card.stat} card={card} index={index} />
          ))}
        </div>
      </div>
    </AbsoluteFill>
  );
};
