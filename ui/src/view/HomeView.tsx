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
  CoproDevice,
  CoproDevicesList,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_copro_pb";
import coproStore from "../store/CoproStore";
import BleDeviceMetricsWidget from "../components/BleDeviceMetricsWidget";
import { AppContext } from "../App";
import { HomeOutlined, DashboardOutlined, FireOutlined, UploadOutlined } from "@ant-design/icons";
import { SiGrafana, SiPrometheus, SiHomeassistant } from "react-icons/si";
import { Badge } from "antd";
import uiConfigStore, { ShortcutConfig } from "../store/UIConfigStore";
import { useSyncExternalStore } from "react";

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

  const [bleDeviceLinky, setBleDeviceLinky] = useState<CoproDevice | undefined>(undefined);
  const [bleDeviceLinkyLoading, setBleDeviceLinkyLoading] = useState(true);

  const [coproAlert, setCoproAlert] = useState<CoproAlert | undefined>(undefined);

  const [time, setTime] = useState(Date.now());

  const navigate = useNavigate();

  // Build base host/protocol for external services (assumes same host as controller)
  const proto = window.location.protocol; // e.g. http: or https:
  const host = window.location.hostname; // hostname only

  // subscribe to UI config to reuse dynamic shortcuts
  const uiConfigState = useSyncExternalStore(
    (cb) => uiConfigStore.subscribe(cb),
    () => uiConfigStore.getState(),
    () => uiConfigStore.getState()
  );
  useEffect(() => {
    uiConfigStore.fetch();
  }, []);

  const mapIcon = (sc?: ShortcutConfig) => {
    if (!sc?.icon) return undefined;
    const name = sc.icon.toLowerCase();
    let base: React.ReactNode | undefined;
    switch (name) {
      case "grafana": base = <SiGrafana />; break;
      case "prometheus": base = <SiPrometheus />; break;
      case "homeassistant":
      case "home_assistant": base = <SiHomeassistant />; break;
      case "swupdate": base = <UploadOutlined />; break;
      default: return undefined;
    }
    const wantsBadge = sc.badge ?? ["grafana","prometheus","homeassistant","home_assistant","swupdate"].includes(name);
    if (!wantsBadge) return base;
    const color = sc.badge_color || (name === "swupdate" ? "blue" : "red");
    return <Badge dot color={color}>{base}</Badge>;
  };

  const shortcuts = uiConfigState.data?.shortcut || [];
  const orderedShortcuts = shortcuts.slice().sort((a,b) => (a.order||0)-(b.order||0));

  useEffect(() => {
    let did = new DeviceId();
    did.setDid(0);

    setInfosLoading(true);
    setHeatersLoading(true);
    setOutdoorAlarmsLoading(true);
    setGarageLoading(true);
    setDevicesWithAlertLoading(true);
    setBleDevicesLoading(true);
    if (uiHomeBLEDevices) {
      setBleDevicesLoading(true);
    } else {
      setBleDeviceLinkyLoading(true);
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
    } else {
      coproStore.getDeviceByName("Linky TIC", (resp: CoproDevice) => {
        setBleDeviceLinky(resp);
        setBleDeviceLinkyLoading(false);
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
      {bleDeviceLinky && (
        <Col xs={24} md={12} xl={6} style={{ marginBottom: 8 }} key={bleDeviceLinky.getMac()}>
          <BleDeviceMetricsWidget
            title={bleDeviceLinky.getName()}
            device={bleDeviceLinky}
            loading={bleDeviceLinkyLoading}
            small={appContext.isMobile}
            debug={appContext.uiDebugMode}
            navigateTo="/ble"
          />
        </Col>
      )}

      <Col xs={24} md={8} xl={6} style={{ marginBottom: 8 }}>
        {outdoorAlarmsMetricsWidget}
      </Col>
      <Col xs={24} md={8} xl={6} style={{ marginBottom: 8 }}>
        <>
          <LoadableCard
            title="Message"
            loading={infosLoading}
            bordered={false}
            isMobile={appContext.isMobile}
          >
            <p style={{ marginBottom: 12, marginTop:0 }}>
              {uiConfigState.data?.message || "Bienvenue sur votre contrôleur Caniot."}
            </p>
            {/* External services quick links */}
            <style>
              {`
                .external-services-list { 
                  display: flex; 
                  flex-direction: column; 
                  gap: 8px; 
                  width: 100%;
                }
                .external-service-link { 
                  background: #f5f5f5; 
                  border-radius: 10px; 
                  padding: 8px 12px; 
                  display: flex; 
                  align-items: center; 
                  gap: 8px; 
                  justify-content: space-between;
                  width: 100%;
                  cursor: pointer; 
                  text-decoration: none; 
                  color: #222; 
                  font-size: 14px; 
                  line-height: 1.2; 
                  box-shadow: 0 1px 2px rgba(0,0,0,0.06);
                  transition: background .18s ease, box-shadow .18s ease, transform .18s ease;
                  border: 1px solid #e5e5e5;
                }
                .external-service-link:hover { 
                  background: #ffffff; 
                  box-shadow: 0 2px 6px rgba(0,0,0,0.12); 
                  transform: translateY(-2px);
                  text-decoration: none;
                }
                .external-service-icon { font-size: 18px; display: flex; align-items: center; }
                .external-service-text { font-weight: 500; }
                .external-service-main { display: flex; align-items: center; gap: 8px; }
                .external-service-url { 
                  margin-left: 12px; 
                  font-size: 12px; 
                  color: #7b7b7b; 
                  font-weight: 400; 
                  white-space: nowrap; 
                }
                .external-service-link:hover .external-service-url { color: #555; }
              `}
            </style>
            <div className="external-services-list">
              {orderedShortcuts.map((s) => {
                // reuse same render style as menu, only show external or internal all? show all shortcuts for now
                const external = s.target_blank || /^https?:\/\//i.test(s.url);
                const href = s.url.startsWith("/") && !external ? s.url : s.url; // keep as-is
                return (
                  <a
                    key={s.name}
                    className="external-service-link"
                    href={href}
                    {...(external ? { target: "_blank", rel: "noopener noreferrer" } : {})}
                  >
                    <span className="external-service-main">
                      <span className="external-service-icon">{mapIcon(s)}</span>
                      <span className="external-service-text">{s.name}</span>
                    </span>
                    <span className="external-service-url">{s.url}</span>
                  </a>
                );
              })}
              {orderedShortcuts.length === 0 && <span style={{fontSize:12,color:'#777'}}>Aucun raccourci configuré</span>}
            </div>
          </LoadableCard>
        </>
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
