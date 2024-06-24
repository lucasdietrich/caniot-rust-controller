import { Col, Row } from "antd";
import React from "react";
import GarageDoorStatus from "./GarageDoorStatus";
import GarageGateStatus from "./GarageGateStatus";
import { DoorState, Status } from "@caniot-controller/caniot-api-grpc-web/api/ng_garage_pb";

interface IGarageDoorsStatusProps {
  garageState?: Status;

  onLeftDoorClick?: () => void;
  onRightDoorClick?: () => void;

  // style
  height?: string;
}

function GarageDoorsStatus({
  garageState = undefined,
  onLeftDoorClick = undefined,
  onRightDoorClick = undefined,
  height = undefined,
}: IGarageDoorsStatusProps) {
  return (
    <Row gutter={0} style={{ maxWidth: 700 }}>
      <Col span={10}>
        <GarageDoorStatus
          closed={garageState?.getLeftClosed() == DoorState.CLOSED}
          progress={garageState?.getLeftProgress() || 0}
          onDoorClick={onLeftDoorClick}
          height={height}
        />
      </Col>
      <Col span={4}>
        <GarageGateStatus
          closed={garageState?.getGateClosed() == DoorState.CLOSED}
          height={height}
        />
      </Col>
      <Col span={10}>
        <GarageDoorStatus
          closed={garageState?.getRightClosed() == DoorState.CLOSED}
          progress={garageState?.getRightProgress() || 0}
          onDoorClick={onRightDoorClick}
          height={height}
        />
      </Col>
    </Row>
  );
}

export default GarageDoorsStatus;
