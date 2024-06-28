import {
  Row,
  Col,
  Card,
  Button,
  List,
  Typography,
  Statistic,
  Badge,
  Divider,
  Alert,
  Space,
} from "antd";
import Hello from "../components/HelloComponent";
import HelloCard from "./HelloCard";
import ListLabelledItem from "../components/ListLabelledItem";
import DeviceDetailsCard from "../components/DeviceDetailsCard";
import { useEffect, useState } from "react";
import { Device } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import devicesStore from "../store/DevicesStore";
import { DeviceId } from "@caniot-controller/caniot-api-grpc-web/api/common_pb";
import GarageDoorsStatus from "../components/GarageDoorsStatus";
import LoadableCard from "../components/LoadableCard";
import { useNavigate } from "react-router-dom";
import { Status as GarageStatus } from "@caniot-controller/caniot-api-grpc-web/api/ng_garage_pb";
import garageStore from "../store/GarageStore";
import DeviceMetricsWidget from "../components/DeviceMetricsWidget";
import DeviceAlert from "../components/DeviceAlert";

const { Countdown } = Statistic;

interface HomeProps {
  refreshInterval?: number;
  isMobile?: boolean;
}

function Home({ refreshInterval = 5000, isMobile = false }: HomeProps) {
  const [garageDevice, setGarageDevice] = useState<Device | undefined>(undefined);
  const [garageState, setGarageState] = useState<GarageStatus | undefined>(undefined);
  const [garageLoading, setGarageLoading] = useState(true);

  const [heatersDevice, setHeatersDevice] = useState<Device | undefined>(undefined);
  const [heatersLoading, setHeatersLoading] = useState(true);

  const [outdoorAlarmsDevice, setOutdoorAlarmsDevice] = useState<Device | undefined>(undefined);
  const [outdoorAlarmsLoading, setOutdoorAlarmsLoading] = useState(true);

  const [time, setTime] = useState(Date.now());

  const navigate = useNavigate();

  useEffect(() => {
    let did = new DeviceId();
    did.setDid(0);

    setHeatersLoading(true);
    setOutdoorAlarmsLoading(true);
    setGarageLoading(true);

    devicesStore.getGarageDevice((resp: Device) => {
      setGarageDevice(resp);
      garageStore.getState((resp: GarageStatus) => {
        setGarageState(resp);
        setGarageLoading(false);
      });
    });

    devicesStore.getHeatersDevice((resp: Device) => {
      setHeatersDevice(resp);
      setHeatersLoading(false);
    });

    devicesStore.getOutdoorAlarmDevice((resp: Device) => {
      setOutdoorAlarmsDevice(resp);
      setOutdoorAlarmsLoading(false);
    });

    const intervalRefresh = setInterval(() => setTime(Date.now()), refreshInterval);
    return () => {
      clearInterval(intervalRefresh);
    };
  }, [time]);

  let garageDoorsStatusWidget = (
    <LoadableCard
      title="Garage"
      onGoto={() => navigate("/devices/garage")}
      loading={garageLoading}
      status={garageDevice !== undefined}
      bordered={false}
    >
      <GarageDoorsStatus
        height="100px"
        garageState={garageState}
        alert={garageDevice?.getActiveAlert()}
      />
    </LoadableCard>
  );

  let garageDoorsMetricsWidget = (
    <DeviceMetricsWidget
      title="Garage"
      loading={garageLoading}
      device={garageDevice}
      navigateTo="/devices/garage"
    />
  );

  let heatersMetricsWidget = (
    <DeviceMetricsWidget
      title="Chauffage"
      loading={heatersLoading}
      device={heatersDevice}
      navigateTo="/devices/heaters"
    />
  );

  let outdoorAlarmsMetricsWidget = (
    <DeviceMetricsWidget
      title="Alarme extérieure"
      loading={outdoorAlarmsLoading}
      device={outdoorAlarmsDevice}
      navigateTo="/devices/alarms"
    />
  );

  return (
    <>
      <Row gutter={16}>
        <Col xs={12} md={8} xl={6} style={{ marginBottom: 8 }}>
          {outdoorAlarmsMetricsWidget}
        </Col>
        <Col xs={12} sm={12} md={8} xl={6} style={{ marginBottom: 8 }}>
          {garageDoorsMetricsWidget}
        </Col>
        <Col xs={12} md={8} xl={6} style={{ marginBottom: 8 }}>
          {heatersMetricsWidget}
        </Col>
        <Col xs={12} md={8} xl={6} style={{ marginBottom: 8 }}>
          {garageDoorsStatusWidget}
        </Col>

        <Col xs={24} xl={12}>
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
    </>
  );
}

export default Home;
