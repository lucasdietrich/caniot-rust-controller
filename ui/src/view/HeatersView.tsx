import { Badge, Card, Col, Form, Result, Row, Space, Spin } from "antd";
import React, { useEffect, useState } from "react";
import HeaterModeSelector from "../components/HeaterModeSelector";
import { CheckCircleFilled, CheckCircleOutlined, LoadingOutlined } from "@ant-design/icons";
import { Command, State, Status } from "@caniot-controller/caniot-api-grpc-web/api/ng_heaters_pb";
import heatersStore from "../store/HeatersStore";
import { Empty } from "google-protobuf/google/protobuf/empty_pb";
import Icon from "@ant-design/icons/lib/components/Icon";
import DeviceStatusCard from "../components/DeviceStatusCard";
import { Device } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import devicesStore from "../store/DevicesStore";
import { DeviceId } from "@caniot-controller/caniot-api-grpc-web/api/common_pb";

const REFRESH_INTERVAL = 5000; // ms

function HeatersView() {
  const [heatersStatus, setHeatersStatus] = useState<Status | undefined>(undefined);
  const [loading, setLoading] = useState(true);
  const [heatersDevice, setHeatersDevice] = useState<Device | undefined>(undefined);
  const [time, setTime] = useState(Date.now());

  useEffect(() => {
    heatersStore.getStatus(new Empty(), (resp: Status) => {
      setHeatersStatus(resp);

      devicesStore.getHeatersDevice((resp: Device) => {
        setHeatersDevice(resp);
        setLoading(false);
      });
    });

    const interval = setInterval(() => setTime(Date.now()), REFRESH_INTERVAL);
    return () => {
      clearInterval(interval);
    };
  }, [time]);

  const onModeChange = (heaterIndex: number, mode: State) => {
    setLoading(true);

    let command = new Command();

    // build a list with 4 elements, all set to NONE
    // set the mode for the selected heater to the selected mode
    let HeaterModesList = Array.from({ length: 4 }, () => State.NONE);
    HeaterModesList[heaterIndex] = mode;

    command.setHeaterList(HeaterModesList);
    heatersStore.setStatus(command, (resp) => {
      setHeatersStatus(resp);
      devicesStore.getHeatersDevice((resp: Device) => {
        setHeatersDevice(resp);
        setLoading(false);
      });
    });
  };

  return (
    <>
      <Row gutter={16}>
        <Col span={14}>
          <Card
            title={
              <Badge
                status={heatersStatus?.getPowerStatus() ? "success" : "error"}
                text={
                  <Space size="middle">
                    Chauffage
                    <Spin spinning={loading} indicator={<LoadingOutlined spin />} />
                  </Space>
                }
              />
            }
          >
            <HeaterModeSelector
              label="Chauffage 1"
              heaterIndex={0}
              initialMode={heatersStatus?.getHeaterList()[0]}
              onModeChange={onModeChange}
            ></HeaterModeSelector>
            <HeaterModeSelector
              label="Chauffage 2"
              heaterIndex={1}
              initialMode={heatersStatus?.getHeaterList()[1]}
              onModeChange={onModeChange}
            ></HeaterModeSelector>
            <HeaterModeSelector
              label="Chauffage 3"
              heaterIndex={2}
              initialMode={heatersStatus?.getHeaterList()[2]}
              onModeChange={onModeChange}
            ></HeaterModeSelector>
            <HeaterModeSelector
              label="Chauffage 4"
              heaterIndex={3}
              initialMode={heatersStatus?.getHeaterList()[3]}
              onModeChange={onModeChange}
            ></HeaterModeSelector>
          </Card>
        </Col>
        <Col span={10}>
          <DeviceStatusCard title="Chauffage" resp={heatersDevice} />
        </Col>
      </Row>
    </>
  );
}

export default HeatersView;
