import { Badge, Card, Progress } from "antd";
import React from "react";

interface IGarageGateStatusProps {
  closed: boolean;
}

function GarageGateStatus({ closed }: IGarageGateStatusProps) {
  return (
    <>
      <img src="/static/gate.png" height={230} />
      <Progress percent={100} showInfo={false} status={closed ? "success" : "exception"} />
    </>
  );
}

export default GarageGateStatus;
