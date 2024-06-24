import React, { useEffect, useState } from "react";
import { Card, Row, Col, Badge, Space, Spin } from "antd";
import GarageDoorStatus from "../components/GarageDoorStatus";
import GarageGateStatus from "../components/GarageGateStatus";
import DeviceDetailsCard from "../components/DeviceDetailsCard";
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
import LoadableCard from "../components/LoadableCard";
import GarageDoorsStatus from "../components/GarageDoorsStatus";

interface IGarageDoorsViewProps {
  refreshInterval?: number;
  isMobile?: boolean;
}

function GarageDoorsView({ refreshInterval = 5000, isMobile = false }: IGarageDoorsViewProps) {
  const [garageState, setGarageState] = useState<Status | undefined>(undefined);
  const [loading, setLoading] = useState(true);
  const [garageDevice, setGarageDevice] = useState<Device | undefined>(undefined);
  const [time, setTime] = useState(Date.now());

  useEffect(() => {
    devicesStore.getGarageDevice((resp: Device) => {
      setGarageDevice(resp);
      garageStore.getState((resp: Status) => {
        setGarageState(resp);
        setLoading(false);
      });
    });

    const intervalRefresh = setInterval(() => setTime(Date.now()), refreshInterval);
    return () => {
      clearInterval(intervalRefresh);
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
      <Col xl={14} xs={24} style={{ marginBottom: 16 }}>
        <LoadableCard
          title="Portes de garage"
          status={garageState !== undefined && garageState?.getGateClosed() !== DoorState.UNKNOWN}
          loading={loading}
          onRefresh={() => {
            setLoading(true);
            setTime(Date.now());
          }}
        >
          <GarageDoorsStatus
            garageState={garageState}
            onLeftDoorClick={onLeftDoorClick}
            onRightDoorClick={onRightDoorClick}
          ></GarageDoorsStatus>

          <Row style={{ paddingTop: 20 }}>
            <DeviceDetailsCard device={undefined} title="Garage Doors" />
          </Row>
        </LoadableCard>
      </Col>
      <Col xl={10} xs={24}>
        <DeviceDetailsCard title="Contrôleur portes de garage" device={garageDevice} />
      </Col>
    </Row>
  );
}

export default GarageDoorsView;
