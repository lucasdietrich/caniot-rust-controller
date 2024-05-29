import { LoadingOutlined } from "@ant-design/icons";
import { Badge, Button, Card, Progress } from "antd";
import React from "react";

interface IGarageDoorStatusProps {
  closed: boolean;
  progress: number;
  onDoorClick?: () => void;
}

function GarageDoorStatus({ closed, progress, onDoorClick = () => {} }: IGarageDoorStatusProps) {
  const handleImageClick = () => {
    onDoorClick();
  };

  return (
    <>
      <Button
        type="link"
        onClick={handleImageClick}
        style={{ padding: 0, border: "none", background: "none" }}
      >
        <img src="/static/door.png" height={230} alt="Garage Door" />
      </Button>
      {closed && <Progress percent={100} showInfo={false} status="success" />}
      {!closed && progress != 0 && <Progress percent={progress} showInfo={false} status="active" />}
      {!closed && progress == 0 && <Progress percent={100} showInfo={false} status="exception" />}
    </>
  );
}

export default GarageDoorStatus;
