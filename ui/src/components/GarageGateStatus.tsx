import { Badge, Card, Progress } from "antd";
import React from "react";

interface IProps {
  closed: boolean;
}

function GarageGateStatus({ closed }: IProps) {
  return (
    <>
      <img src="/gate.png" height={230} />
      <Progress percent={100} showInfo={false} status={closed ? "success" : "exception"} />
    </>
  );
}

export default GarageGateStatus;
