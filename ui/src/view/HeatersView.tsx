import { Badge, Card, Col, Form, Result, Row, Space } from "antd";
import React, { useEffect, useState } from "react";
import HeaterModeSelector from "../components/HeaterModeSelector";
import { CheckCircleFilled, CheckCircleOutlined } from "@ant-design/icons";
import {
  Command,
  State,
  Status,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_heaters_pb";
import heatersStore from "../store/HeatersStore";
import { Empty } from "google-protobuf/google/protobuf/empty_pb";

function HeatersView() {
  const [heatersStatus, setHeatersStatus] = useState<Status | undefined>(
    undefined
  );

  useEffect(() => {
    heatersStore.getStatus(new Empty(), (resp: Status) => {
      setHeatersStatus(resp);
    });
  }, []);

  const onModeChange = (heaterIndex: number, mode: State) => {
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
                text="Chauffages"
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
