import { AbsoluteFill, Sequence } from "remotion";
import { LogoIntro } from "./scenes/LogoIntro";
import { CodeTypewriter } from "./scenes/CodeTypewriter";
import { FeatureCards } from "./scenes/FeatureCards";
import { InstallCTA } from "./scenes/InstallCTA";
import { COLORS, FONTS, FPS } from "./styles";

export const NewtDemo: React.FC = () => {
  return (
    <AbsoluteFill
      style={{
        backgroundColor: COLORS.bg,
        fontFamily: FONTS.sans,
      }}
    >
      {/* Scene 1: Logo Intro (0-8s, frames 0-239) */}
      <Sequence from={0} durationInFrames={FPS * 8}>
        <LogoIntro />
      </Sequence>

      {/* Scene 2: Code Typewriter (8-18s, frames 240-539) */}
      <Sequence from={FPS * 8} durationInFrames={FPS * 10}>
        <CodeTypewriter />
      </Sequence>

      {/* Scene 3: Feature Cards (18-25s, frames 540-749) */}
      <Sequence from={FPS * 18} durationInFrames={FPS * 7}>
        <FeatureCards />
      </Sequence>

      {/* Scene 4: Install CTA (25-30s, frames 750-899) */}
      <Sequence from={FPS * 25} durationInFrames={FPS * 5}>
        <InstallCTA />
      </Sequence>
    </AbsoluteFill>
  );
};
