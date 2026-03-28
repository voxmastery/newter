import React from "react";
import {
  AbsoluteFill,
  useCurrentFrame,
  useVideoConfig,
  interpolate,
  spring,
} from "remotion";
import { COLORS, FONTS } from "../styles";

export const LogoIntro: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps, durationInFrames } = useVideoConfig();

  // Logo scale-in spring
  const logoScale = spring({
    frame,
    fps,
    config: { damping: 12, stiffness: 80, mass: 0.8 },
  });

  // Tagline fade-in (starts at 1.5s)
  const taglineOpacity = interpolate(frame, [45, 75], [0, 1], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

  const taglineY = interpolate(frame, [45, 75], [20, 0], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

  // Violet accent line width animation (starts at 2s)
  const lineWidth = interpolate(frame, [60, 110], [0, 320], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

  // Fade out at end (last 1s = 30 frames)
  const fadeOut = interpolate(
    frame,
    [durationInFrames - 30, durationInFrames],
    [1, 0],
    {
      extrapolateLeft: "clamp",
      extrapolateRight: "clamp",
    }
  );

  return (
    <AbsoluteFill
      style={{
        justifyContent: "center",
        alignItems: "center",
        opacity: fadeOut,
      }}
    >
      {/* Logo text */}
      <div
        style={{
          fontSize: 160,
          fontWeight: 800,
          fontFamily: FONTS.sans,
          color: COLORS.text,
          transform: `scale(${logoScale})`,
          letterSpacing: "-0.03em",
        }}
      >
        newt
      </div>

      {/* Violet accent line */}
      <div
        style={{
          width: lineWidth,
          height: 4,
          backgroundColor: COLORS.violet,
          borderRadius: 2,
          marginTop: 16,
          marginBottom: 24,
        }}
      />

      {/* Tagline */}
      <div
        style={{
          fontSize: 36,
          color: COLORS.textMuted,
          fontFamily: FONTS.sans,
          fontWeight: 400,
          opacity: taglineOpacity,
          transform: `translateY(${taglineY}px)`,
        }}
      >
        A UI language that compiles to anything
      </div>
    </AbsoluteFill>
  );
};
