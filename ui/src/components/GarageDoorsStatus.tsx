import { Col, Row } from "antd";
import React from "react";
import GarageDoorStatus from "./GarageDoorStatus";
import GarageGateStatus from "./GarageGateStatus";
import { DoorState, Status } from "@caniot-controller/caniot-api-grpc-web/api/ng_garage_pb";
import { DeviceAlert as gDeviceAlert } from "@caniot-controller/caniot-api-grpc-web/api/common_pb";
import DeviceAlert from "./DeviceAlert";

interface IGarageDoorsStatusProps {
  garageState?: Status;
  alert?: gDeviceAlert;

  onLeftDoorClick?: () => void;
  onRightDoorClick?: () => void;

  // style
  height?: string;
  isMobile?: boolean;
}

function GarageDoorsStatus({
  garageState = undefined,
  alert = undefined,
  onLeftDoorClick = undefined,
  onRightDoorClick = undefined,
  height = undefined,
  isMobile = false,
}: IGarageDoorsStatusProps) {
  return (
    <Row>
      <Col span={24}>
        <DeviceAlert alert={alert} isMobile={isMobile} />
      </Col>
      <Col span={24} style={{ marginTop: 10 }}>
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
      </Col>
    </Row>
  );
}

export default GarageDoorsStatus;
