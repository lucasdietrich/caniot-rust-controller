import React, { PropsWithChildren } from "react";
import { MobileMaxSize } from "../../App";
import Media from "react-media";

interface MediaMobileProps {
  render: (isMobile: boolean) => JSX.Element;
}

const queries = {
  "screen-mobile": {
    maxWidth: MobileMaxSize,
  },
  "screen-large": {
    minWidth: MobileMaxSize + 1,
  },
};

function MediaMobile({ render }: PropsWithChildren<MediaMobileProps>) {
  return <Media queries={queries}>{(matches) => render(matches["screen-mobile"])}</Media>;
}

export default MediaMobile;
