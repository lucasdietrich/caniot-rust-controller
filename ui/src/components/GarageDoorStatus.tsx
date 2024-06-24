import { LoadingOutlined } from "@ant-design/icons";
import { Badge, Button, Card, Progress } from "antd";
import React from "react";

interface IGarageDoorStatusProps {
  closed: boolean;
  progress: number;
  onDoorClick?: () => void;

  height?: string;
}

function GarageDoorStatus({
  closed,
  progress,
  onDoorClick = undefined,
  height = "200px",
}: IGarageDoorStatusProps) {
  return (
    <>
      <Button
        type="link"
        onClick={onDoorClick}
        style={{
          padding: 0,
          border: "none",
          background: "none",
          width: "100%",
          cursor: onDoorClick ? "pointer" : "default",
          height: height,
        }}
      >
        <img src="/static/door.png" height={height} width="100%" alt="Garage Door" />
      </Button>
      {closed && <Progress percent={100} showInfo={false} status="success" />}
      {!closed && progress != 0 && <Progress percent={progress} showInfo={false} status="active" />}
      {!closed && progress == 0 && <Progress percent={100} showInfo={false} status="exception" />}
    </>
  );
}

export default GarageDoorStatus;
