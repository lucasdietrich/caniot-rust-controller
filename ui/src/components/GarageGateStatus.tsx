import { Badge, Card, Progress } from "antd";
import React from "react";

interface IGarageGateStatusProps {
  closed: boolean;
  height?: string;
}

function GarageGateStatus({ closed, height = "200px" }: IGarageGateStatusProps) {
  return (
    <>
      <img src="/static/gate.png" height={height} width="100%" />
      <Progress percent={100} showInfo={false} status={closed ? "success" : "exception"} />
    </>
  );
}

export default GarageGateStatus;
