import { Composition } from "remotion";
import { NewtDemo } from "./NewtDemo";
import { FPS, WIDTH, HEIGHT, DURATION_FRAMES } from "./styles";

export const RemotionRoot: React.FC = () => {
  return (
    <Composition
      id="NewtDemo"
      component={NewtDemo}
      durationInFrames={DURATION_FRAMES}
      fps={FPS}
      width={WIDTH}
      height={HEIGHT}
    />
  );
};
