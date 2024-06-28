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
import SoftwareInfosCard from "../components/SoftwareInfosCard";
import { Infos, SoftwareInfos } from "@caniot-controller/caniot-api-grpc-web/api/ng_internal_pb";
import internalStore from "../store/InternalStore";
import FirmwareInfosCard from "../components/FirmwareInfosCard";
import ControllerStatsCard from "../components/ControllerStatsCard";

const { Countdown } = Statistic;

interface HomeProps {
  refreshInterval?: number;
  isMobile?: boolean;
}

function Home({ refreshInterval = 5000, isMobile = false }: HomeProps) {
  const [infosLoading, setInfosLoading] = useState(true);
  const [infos, setInfos] = useState<Infos | undefined>(undefined);

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

    setInfosLoading(true);
    setHeatersLoading(true);
    setOutdoorAlarmsLoading(true);
    setGarageLoading(true);

    internalStore.getInfos((resp: Infos) => {
      setInfos(resp);
      setInfosLoading(false);
    });

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

  const garageDoorsStatusWidget = (
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

  const garageDoorsMetricsWidget = (
    <DeviceMetricsWidget
      title="Garage"
      loading={garageLoading}
      device={garageDevice}
      navigateTo="/devices/garage"
    />
  );

  const heatersMetricsWidget = (
    <DeviceMetricsWidget
      title="Chauffage"
      loading={heatersLoading}
      device={heatersDevice}
      navigateTo="/devices/heaters"
    />
  );

  const outdoorAlarmsMetricsWidget = (
    <DeviceMetricsWidget
      title="Alarme extérieure"
      loading={outdoorAlarmsLoading}
      device={outdoorAlarmsDevice}
      navigateTo="/devices/alarms"
    />
  );

  const devicesActiveAlerts = (
    <LoadableCard title="Alertes actives" loading={false} bordered={false}>
      <DeviceAlert alert={garageDevice?.getActiveAlert()} />
      <DeviceAlert alert={heatersDevice?.getActiveAlert()} />
      <DeviceAlert alert={outdoorAlarmsDevice?.getActiveAlert()} />
    </LoadableCard>
  );

  return (
    <>
      <Row gutter={16}>
        <Col xs={24} md={12} xl={12} style={{ marginBottom: 8 }}>
          {devicesActiveAlerts}
        </Col>
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

        <Col xs={24} xl={12} style={{ marginBottom: 8 }}>
          <SoftwareInfosCard infos={infos?.getSoftware()} />
        </Col>
        <Col xs={24} xl={12} style={{ marginBottom: 8 }}>
          <FirmwareInfosCard infos={infos?.getFirmware()} />
        </Col>
        <Col xs={24} xl={12} style={{ marginBottom: 8 }}>
          <ControllerStatsCard stats={infos?.getControllerStats()} />
        </Col>
      </Row>
    </>
  );
}

export default Home;
