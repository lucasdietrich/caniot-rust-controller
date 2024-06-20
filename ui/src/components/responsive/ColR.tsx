import { Col } from "antd";
import { PropsWithChildren } from "react";

function ColR({ children }: PropsWithChildren<{}>) {
  return (
    <Col xs={24} xl={12}>
      {children}
    </Col>
  );
}

export default ColR;
