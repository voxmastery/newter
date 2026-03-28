import React from "react";
import {
  AbsoluteFill,
  useCurrentFrame,
  useVideoConfig,
  interpolate,
  spring,
} from "remotion";
import { COLORS, FONTS } from "../styles";

export const InstallCTA: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  // Heading spring-in
  const headingSpring = spring({
    frame,
    fps,
    config: { damping: 14, stiffness: 80, mass: 0.8 },
  });

  const headingY = interpolate(headingSpring, [0, 1], [40, 0]);

  // Terminal block appears after heading
  const terminalSpring = spring({
    frame: Math.max(0, frame - 15),
    fps,
    config: { damping: 14, stiffness: 80, mass: 0.7 },
  });

  const terminalY = interpolate(terminalSpring, [0, 1], [30, 0]);

  // Blinking cursor in terminal
  const cursorVisible = Math.floor(frame / 15) % 2 === 0;

  // GitHub URL + MIT text fade in
  const footerOpacity = interpolate(frame, [50, 75], [0, 1], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

  const footerY = interpolate(frame, [50, 75], [15, 0], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
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
          flexDirection: "column",
          alignItems: "center",
          gap: 40,
        }}
      >
        {/* Heading */}
        <div
          style={{
            fontSize: 56,
            fontWeight: 800,
            color: COLORS.text,
            fontFamily: FONTS.sans,
            opacity: headingSpring,
            transform: `translateY(${headingY}px)`,
          }}
        >
          Get started in seconds
        </div>

        {/* Terminal block */}
        <div
          style={{
            backgroundColor: COLORS.codeBg,
            borderRadius: 16,
            border: "1px solid #2a2a2a",
            padding: "24px 40px",
            opacity: terminalSpring,
            transform: `translateY(${terminalY}px)`,
          }}
        >
          <div
            style={{
              fontFamily: FONTS.mono,
              fontSize: 24,
              color: COLORS.text,
              display: "flex",
              alignItems: "center",
            }}
          >
            <span style={{ color: COLORS.green, marginRight: 12 }}>$</span>
            <span>cargo install newter-compiler</span>
            {cursorVisible && (
              <span
                style={{
                  display: "inline-block",
                  width: 2,
                  height: 28,
                  backgroundColor: COLORS.violet,
                  marginLeft: 4,
                  verticalAlign: "text-bottom",
                }}
              />
            )}
          </div>
        </div>

        {/* Footer: GitHub URL + MIT */}
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            gap: 16,
            opacity: footerOpacity,
            transform: `translateY(${footerY}px)`,
          }}
        >
          {/* GitHub button */}
          <div
            style={{
              display: "flex",
              alignItems: "center",
              gap: 12,
              backgroundColor: COLORS.cardBg,
              border: "1px solid #2a2a2a",
              borderRadius: 12,
              padding: "14px 32px",
            }}
          >
            <svg
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.604-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.463-1.11-1.463-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.831.092-.646.35-1.086.636-1.337-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.578 9.578 0 0112 6.836c.85.004 1.705.115 2.504.337 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.203 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.578.688.48C19.138 20.161 22 16.416 22 12c0-5.523-4.477-10-10-10z"
                fill={COLORS.text}
              />
            </svg>
            <span
              style={{
                fontSize: 20,
                color: COLORS.text,
                fontFamily: FONTS.sans,
                fontWeight: 500,
              }}
            >
              github.com/voxmastery/newter
            </span>
          </div>

          {/* MIT Licensed */}
          <div
            style={{
              fontSize: 18,
              color: COLORS.textMuted,
              fontFamily: FONTS.sans,
            }}
          >
            MIT Licensed
          </div>
        </div>
      </div>
    </AbsoluteFill>
  );
};
