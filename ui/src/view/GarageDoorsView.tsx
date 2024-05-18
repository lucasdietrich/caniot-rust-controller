import React, { useEffect, useState } from "react";
import { Card, Row, Col, Badge, Space, Spin } from "antd";
import GarageDoorStatus from "../components/GarageDoorStatus";
import GarageGateStatus from "../components/GarageGateStatus";
import DeviceStatusCard from "../components/DeviceStatusCard";
import {
  Command,
  CommandMessage,
  DoorState,
  Status,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_garage_pb";
import { Device } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import garageStore from "../store/GarageStore";
import { Empty } from "google-protobuf/google/protobuf/empty_pb";
import devicesStore from "../store/DevicesStore";
import { LoadingOutlined } from "@ant-design/icons";

interface IProps {
  refreshInterval?: number;
}

function GarageDoorsView({ refreshInterval = 5000 }: IProps) {
  const [garageState, setGarageState] = useState<Status | undefined>(undefined);
  const [loading, setLoading] = useState(true);
  const [garageDevice, setGarageDevice] = useState<Device | undefined>(undefined);
  const [time, setTime] = useState(Date.now());

  useEffect(() => {
    devicesStore.getGarageDevice((resp: Device) => {
      setGarageDevice(resp);
      garageStore.getState(new Empty(), (resp: Status) => {
        setGarageState(resp);
        setLoading(false);
      });
    });

    const interval = setInterval(() => setTime(Date.now()), refreshInterval);
    return () => {
      clearInterval(interval);
    };
  }, [time]);

  const handleDoorClick = (commandType: Command) => {
    setLoading(true);

    let command = new CommandMessage();
    command.setCommand(commandType);
    garageStore.setState(command, (resp) => {
      setGarageState(resp);
      devicesStore.getGarageDevice((resp: Device) => {
        setGarageDevice(resp);
        setLoading(false);
      });
    });
  };

  const onLeftDoorClick = () => handleDoorClick(Command.LEFT);
  const onRightDoorClick = () => handleDoorClick(Command.RIGHT);

  return (
    <Row gutter={16}>
      <Col span={14}>
        <Card
          title={
            <Badge
              status={
                garageState === undefined || garageState?.getGateClosed() === DoorState.UNKNOWN
                  ? "error"
                  : "success"
              }
              text={
                <Space size="middle">
                  Portes de garage
                  <Spin spinning={loading} indicator={<LoadingOutlined spin />} />
                </Space>
              }
            />
          }
        >
          <Row gutter={20}>
            <Col flex="300px">
              <GarageDoorStatus
                closed={garageState?.getLeftClosed() == DoorState.CLOSED}
                progress={garageState?.getLeftProgress() || 0}
                onDoorClick={onLeftDoorClick}
              />
            </Col>
            <Col flex="120px">
              <GarageGateStatus closed={garageState?.getGateClosed() == DoorState.CLOSED} />
            </Col>
            <Col flex="300px">
              <GarageDoorStatus
                closed={garageState?.getRightClosed() == DoorState.CLOSED}
                progress={garageState?.getRightProgress() || 0}
                onDoorClick={onRightDoorClick}
              />
            </Col>
          </Row>
          <Row style={{ paddingTop: 20 }}>
            <DeviceStatusCard device={undefined} title="Garage Doors" />
          </Row>
        </Card>
      </Col>
      <Col span={10}>
        <DeviceStatusCard title="Contrôleur portes de garage" device={garageDevice} />
      </Col>
    </Row>
  );
}

export default GarageDoorsView;
