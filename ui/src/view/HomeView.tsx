import { Row, Col, Statistic } from "antd";
import { useEffect, useState } from "react";
import { Device, DevicesList } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import devicesStore from "../store/DevicesStore";
import { DeviceAlertType, DeviceId } from "@caniot-controller/caniot-api-grpc-web/api/common_pb";
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
import AlarmDiagWidget from "../components/AlarmDiagWidget";
import { OutdoorAlarmState } from "@caniot-controller/caniot-api-grpc-web/api/ng_alarms_pb";
import alarmsStore from "../store/AlarmsStore";
import {
  CoproAlert,
  CoproDevicesList,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_copro_pb";
import coproStore from "../store/CoproStore";
import BleDeviceMetricsWidget from "../components/BleDeviceMetricsWidget";
import { AppContext } from "../App";

const { Countdown } = Statistic;

interface HomeProps {
  appContext: AppContext;
  refreshInterval?: number;
  uiHomeBLEDevices?: boolean;
}

function HomeView({ appContext, refreshInterval = 5000, uiHomeBLEDevices = false }: HomeProps) {
  const [infosLoading, setInfosLoading] = useState(true);
  const [infos, setInfos] = useState<Infos | undefined>(undefined);

  const [devicesWithAlert, setDevicesWithAlert] = useState<DevicesList | undefined>(undefined);
  const [devicesWithAlertLoading, setDevicesWithAlertLoading] = useState(true);

  const [garageDevice, setGarageDevice] = useState<Device | undefined>(undefined);
  const [garageState, setGarageState] = useState<GarageStatus | undefined>(undefined);
  const [garageLoading, setGarageLoading] = useState(true);

  const [heatersDevice, setHeatersDevice] = useState<Device | undefined>(undefined);
  const [heatersLoading, setHeatersLoading] = useState(true);

  const [outdoorAlarmState, setOutdoorAlarmState] = useState<OutdoorAlarmState | undefined>(
    undefined
  );
  const [outdoorAlarmsDevice, setOutdoorAlarmsDevice] = useState<Device | undefined>(undefined);
  const [outdoorAlarmsLoading, setOutdoorAlarmsLoading] = useState(true);

  const [bleDevicesList, setBleDevicesList] = useState<CoproDevicesList | undefined>(undefined);
  const [bleDevicesLoading, setBleDevicesLoading] = useState(true);

  const [coproAlert, setCoproAlert] = useState<CoproAlert | undefined>(undefined);

  const [time, setTime] = useState(Date.now());

  const navigate = useNavigate();

  useEffect(() => {
    let did = new DeviceId();
    did.setDid(0);

    setInfosLoading(true);
    setHeatersLoading(true);
    setOutdoorAlarmsLoading(true);
    setGarageLoading(true);
    setDevicesWithAlertLoading(true);
    if (uiHomeBLEDevices) {
      setBleDevicesLoading(true);
    }

    devicesStore.getDevicesWithActiveAlert((devices: DevicesList) => {
      setDevicesWithAlert(devices);
      setDevicesWithAlertLoading(false);
    });

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
      alarmsStore.getOutdoorAlarmState((resp: OutdoorAlarmState) => {
        setOutdoorAlarmState(resp);
        setOutdoorAlarmsLoading(false);
      });
    });

    if (uiHomeBLEDevices) {
      coproStore.getList((resp: CoproDevicesList) => {
        setBleDevicesList(resp);
        setBleDevicesLoading(false);
      });
    }

    coproStore.getCoproAlert((resp: CoproAlert) => {
      setCoproAlert(resp);
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
      isMobile={appContext.isMobile}
      className="no-vertical-padding"
    >
      <GarageDoorsStatus height="100px" garageState={garageState} isMobile={appContext.isMobile} />
    </LoadableCard>
  );

  const garageDoorsMetricsWidget = (
    <DeviceMetricsWidget
      title="Garage"
      loading={garageLoading}
      device={garageDevice}
      navigateTo="/devices/garage"
      appContext={appContext}
    />
  );

  const heatersMetricsWidget = (
    <DeviceMetricsWidget
      title="Chauffage"
      loading={heatersLoading}
      device={heatersDevice}
      navigateTo="/devices/heaters"
      appContext={appContext}
    />
  );

  const outdoorAlarmsGenericMetricsWidget = (
    <DeviceMetricsWidget
      title="Alarme extérieure"
      loading={outdoorAlarmsLoading}
      device={outdoorAlarmsDevice}
      navigateTo="/devices/alarms"
      appContext={appContext}
    />
  );

  const outdoorAlarmsMetricsWidget = (
    <AlarmDiagWidget
      title="Diagnostique alarme"
      alarm={outdoorAlarmState}
      loading={outdoorAlarmsLoading}
      navigateTo="/devices/alarms"
      isMobile={appContext.isMobile}
    />
  );

  let hasCoproAlertActive = false;
  if (
    coproAlert?.hasActiveAlert() &&
    (coproAlert?.getActiveAlert()?.getAlertType() == DeviceAlertType.OK ||
      coproAlert?.getActiveAlert()?.getAlertType() == DeviceAlertType.NOTIFICATION)
  ) {
    hasCoproAlertActive = appContext.uiDebugMode;
  }

  const hasDevicesAlertsActive = devicesWithAlert && devicesWithAlert.getDevicesList().length > 0;
  const hasAlertsActive = hasCoproAlertActive || hasDevicesAlertsActive;

  const devicesActiveAlerts = (
    <LoadableCard
      title="Alertes actives"
      loading={devicesWithAlertLoading}
      bordered={false}
      isMobile={appContext.isMobile}
    >
      {hasAlertsActive ? (
        <>
          {hasCoproAlertActive && (
            <DeviceAlert
              key="coproAlert"
              alert={coproAlert?.getActiveAlert()}
              navigateToController="ble"
              closable={false}
              isMobile={appContext.isMobile}
            />
          )}
          {hasDevicesAlertsActive &&
            devicesWithAlert
              .getDevicesList()
              .map((device) => (
                <DeviceAlert
                  key={device.getDid()?.getDid()}
                  alert={device.getActiveAlert()}
                  navigateToController={`devices/${device.getUiViewName()}`}
                  closable={false}
                  isMobile={appContext.isMobile}
                />
              ))}
        </>
      ) : (
        <p>Aucune alerte active</p>
      )}
    </LoadableCard>
  );

  return (
    <Row gutter={16}>
      {hasAlertsActive && (
        <Col xs={24} md={24} xl={12} style={{ marginBottom: 8 }}>
          {devicesActiveAlerts}
        </Col>
      )}
      <Col xs={12} md={8} xl={6} style={{ marginBottom: 8 }}>
        {garageDoorsStatusWidget}
      </Col>
      <Col xs={12} sm={12} md={8} xl={6} style={{ marginBottom: 8 }}>
        {garageDoorsMetricsWidget}
      </Col>
      <Col xs={12} md={8} xl={6} style={{ marginBottom: 8 }}>
        {outdoorAlarmsGenericMetricsWidget}
      </Col>
      <Col xs={12} md={8} xl={6} style={{ marginBottom: 8 }}>
        {heatersMetricsWidget}
      </Col>

      {/* each BLE device gets its own card */}
      {bleDevicesList &&
        bleDevicesList.getDevicesList().map((device) => (
          <Col xs={24} md={12} xl={6} style={{ marginBottom: 8 }} key={device.getMac()}>
            <BleDeviceMetricsWidget
              title={device.getName()}
              device={device}
              loading={bleDevicesLoading}
              small={appContext.isMobile}
              debug={appContext.uiDebugMode}
              navigateTo="/ble"
            />
          </Col>
        ))}

      <Col xs={24} md={8} xl={6} style={{ marginBottom: 8 }}>
        {outdoorAlarmsMetricsWidget}
      </Col>

      {appContext.uiDebugMode && (
        <>
          <Col xs={24} xl={12} style={{ marginBottom: 8 }}>
            <SoftwareInfosCard infos={infos?.getSoftware()} isMobile={appContext.isMobile} />
          </Col>
          <Col xs={24} xl={12} style={{ marginBottom: 8 }}>
            <FirmwareInfosCard infos={infos?.getFirmware()} isMobile={appContext.isMobile} />
          </Col>
          <Col xs={24} xl={12} style={{ marginBottom: 8 }}>
            <ControllerStatsCard
              stats={infos?.getControllerStats()}
              isMobile={appContext.isMobile}
            />
          </Col>
        </>
      )}
    </Row>
  );
}

export default HomeView;
