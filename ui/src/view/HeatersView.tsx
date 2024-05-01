import { Badge, Card, Col, Form, Result, Row, Space, Spin } from "antd";
import React, { useEffect, useState } from "react";
import HeaterModeSelector from "../components/HeaterModeSelector";
import {
  CheckCircleFilled,
  CheckCircleOutlined,
  LoadingOutlined,
} from "@ant-design/icons";
import {
  Command,
  State,
  Status,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_heaters_pb";
import heatersStore from "../store/HeatersStore";
import { Empty } from "google-protobuf/google/protobuf/empty_pb";
import Icon from "@ant-design/icons/lib/components/Icon";

function HeatersView() {
  const [heatersStatus, setHeatersStatus] = useState<Status | undefined>(
    undefined
  );
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    heatersStore.getStatus(new Empty(), (resp: Status) => {
      setLoading(false);
      setHeatersStatus(resp);
    });
  }, []);

  const onModeChange = (heaterIndex: number, mode: State) => {
    setLoading(true);

    console.log("Heater", heaterIndex, "mode changed to", mode);

    let command = new Command();

    // build a list with 4 elements, all set to NONE
    // set the mode for the selected heater to the selected mode
    let HeaterModesList = Array.from({ length: 4 }, () => State.NONE);
    HeaterModesList[heaterIndex] = mode;

    command.setHeaterList(HeaterModesList);
    heatersStore.setStatus(command, (resp) => {
      console.log("Heaters status updated", resp.getHeaterList());
      setHeatersStatus(resp);
      setLoading(false);
    });
  };

  return (
    <>
      <Row gutter={16}>
        <Col span={16}>
          <Card
            title={
              <Badge
                status={heatersStatus?.getPowerStatus() ? "success" : "error"}
                text={
                  <Space size="middle">
                    Chauffage
                    <Spin
                      spinning={loading}
                      indicator={<LoadingOutlined spin />}
                    />
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
        <Col span={8}>
          <Card title="Test"></Card>
        </Col>
      </Row>
    </>
  );
}

export default HeatersView;
