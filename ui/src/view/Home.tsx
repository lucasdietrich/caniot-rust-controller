import { Row, Col, Card, Button, List, Typography } from "antd";
import Hello from "../components/Hello";
import { ReloadOutlined } from "@ant-design/icons";
import HelloCard from "./HelloCard";
import ListLabelledItem from "../components/ListLabelledItem";
import DeviceStatusCard from "../components/DeviceStatusCard";
import { useEffect, useState } from "react";
import { Device } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import devicesStore from "../store/DevicesStore";
import { DeviceId } from "@caniot-controller/caniot-api-grpc-web/api/common_pb";

function Home() {
  const [demoDevice, setdemoDevice] = useState<Device | undefined>(undefined);

  useEffect(() => {
    let did = new DeviceId();
    did.setDid(0);

    devicesStore.get(did, (resp: Device) => {
      setdemoDevice(resp);
    });
  }, []);

  return (
    <>
      <Row gutter={16}>
        <Col span={12}>
          <HelloCard user_name="Lucas" />
        </Col>
        <Col span={12}>
          <Card title="Firmware" bordered={false}>
            <List>
              <ListLabelledItem label="Firmware version">v0.1.0-beta</ListLabelledItem>
              <ListLabelledItem label="Firmware data">04/10/2021 12:00:00</ListLabelledItem>
              <ListLabelledItem label="Firmware status">
                <Typography.Text type="success">Running</Typography.Text>
              </ListLabelledItem>
            </List>
          </Card>
        </Col>
      </Row>
      <Row gutter={16} style={{ paddingTop: 20 }}>
        <Col span={12}>
          <HelloCard user_name="Tom" />
        </Col>
        <Col span={12}>
          <DeviceStatusCard title="Demo" resp={demoDevice} />
        </Col>
      </Row>
    </>
  );
}

export default Home;
