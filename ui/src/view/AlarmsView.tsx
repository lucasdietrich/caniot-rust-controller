import { Alert, Badge, Button, Col, List, Row, Tag } from "antd";
import React, { useEffect, useState } from "react";
import DeviceDetailsCard from "../components/DeviceDetailsCard";
import { Device } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import devicesStore from "../store/DevicesStore";
import LoadableCard from "../components/LoadableCard";
import ListGridItem from "../components/ListGridItem";
import TwoStatesSelector, { TwoStateCommand } from "../components/TwoStatesSelector";
import {
  OutdoorAlarmCommand,
  OutdoorAlarmLightsCommand,
  OutdoorAlarmState,
  SirenAction,
  TwoStates as gTwoStates,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_alarms_pb";
import alarmsStore from "../store/AlarmsStore";
import { MinusCircleOutlined, SyncOutlined } from "@ant-design/icons";
import DeviceAlert from "../components/DeviceAlert";
import AlarmDiagWidget from "../components/AlarmDiagWidget";

interface IAlarmsViewProps {
  refreshInterval?: number;
  isMobile?: boolean;
  uiDebugMode?: boolean;
}

function AlarmsView({
  refreshInterval = 5000,
  isMobile = false,
  uiDebugMode = false,
}: IAlarmsViewProps) {
  const [outdoorAlarmDevice, setOutdoorAlarmDevice] = useState<Device | undefined>(undefined);
  const [outdoorAlarmState, setOutdoorAlarmState] = useState<OutdoorAlarmState | undefined>(
    undefined
  );
  const [sirenForceOffRequested, setSirenForceOffRequested] = useState<boolean>(false);
  const [loading, setLoading] = useState(true);
  const [time, setTime] = useState(Date.now());

  useEffect(() => {
    setLoading(true);
    devicesStore.getOutdoorAlarmDevice((resp: Device) => {
      setOutdoorAlarmDevice(resp);
      alarmsStore.getOutdoorAlarmState((resp: OutdoorAlarmState) => {
        setOutdoorAlarmState(resp);
        setLoading(false);
      });
    });

    const interval = setInterval(() => setTime(Date.now()), refreshInterval);
    return () => {
      clearInterval(interval);
    };
  }, [time]);

  enum Light {
    All = 0,
    South = 1,
    East = 2,
  }

  const sendAlarmCommand = (command: OutdoorAlarmCommand) => {
    alarmsStore.sendOutdoorAlarmCommand(
      command,
      (resp) => {
        setOutdoorAlarmState(resp);
        devicesStore.getOutdoorAlarmDevice((resp: Device) => {
          setOutdoorAlarmDevice(resp);
          setLoading(false);
          setSirenForceOffRequested(false);
        });
      },
      (err) => {
        setLoading(false);
      }
    );
  };

  const handleLightAction = (light: Light, tsCmd: TwoStateCommand) => {
    setLoading(true);

    let ts = gTwoStates.NONE;
    if (tsCmd === TwoStateCommand.ON) {
      ts = gTwoStates.ON;
    } else if (tsCmd === TwoStateCommand.OFF) {
      ts = gTwoStates.OFF;
    } else if (tsCmd === TwoStateCommand.TOGGLE) {
      ts = gTwoStates.TOGGLE;
    }

    let lightsCommand: OutdoorAlarmLightsCommand = new OutdoorAlarmLightsCommand();
    if (light === Light.South) {
      lightsCommand.setSouthLight(ts);
    } else if (light === Light.East) {
      lightsCommand.setEastLight(ts);
    } else if (light === Light.All) {
      lightsCommand.setSouthLight(ts);
      lightsCommand.setEastLight(ts);
    }

    let command = new OutdoorAlarmCommand();
    command.setLights(lightsCommand);

    sendAlarmCommand(command);
  };

  const handleAlarmAction = (tsCmd: TwoStateCommand) => {
    setLoading(true);

    let command = new OutdoorAlarmCommand();
    if (tsCmd === TwoStateCommand.ON) {
      command.setOutdoorAlarmEnable(gTwoStates.ON);
    } else if (tsCmd === TwoStateCommand.OFF) {
      command.setOutdoorAlarmEnable(gTwoStates.OFF);
    }

    sendAlarmCommand(command);
  };

  const handleSirenAction = (saCmd: SirenAction) => {
    setLoading(true);
    setSirenForceOffRequested(saCmd === SirenAction.FORCE_OFF);

    console.log("Siren action: " + saCmd);

    let command = new OutdoorAlarmCommand();
    command.setOutdoorAlarmSirenDirectAction(saCmd);

    sendAlarmCommand(command);
  };

  const onSouthLightChange = (ts: TwoStateCommand) => handleLightAction(Light.South, ts);
  const onEastLightChange = (ts: TwoStateCommand) => handleLightAction(Light.East, ts);
  const onAllLightChange = (ts: TwoStateCommand) => handleLightAction(Light.All, ts);

  const onAlarmChange = (ts: TwoStateCommand) => handleAlarmAction(ts);
  const onSirenForceOff = () => handleSirenAction(SirenAction.FORCE_OFF);

  const outdoorAlarmEnabled = outdoorAlarmState?.getEnabled();
  const outdoorSirenActive = outdoorAlarmState?.getDevice()?.getSirenActive();

  const outdoorAlarmEastLight = outdoorAlarmState?.getDevice()?.getEastLight();
  const outdoorAlarmSouthLight = outdoorAlarmState?.getDevice()?.getSouthLight();

  return (
    <Row gutter={16}>
      <Col xl={14} xs={24} style={{ marginBottom: 16 }}>
        <LoadableCard
          title="Alarme extérieure"
          status={outdoorAlarmState !== undefined}
          loading={loading}
          onRefresh={() => {
            setLoading(true);
            setTime(Date.now());
          }}
        >
          <DeviceAlert alert={outdoorAlarmDevice?.getActiveAlert()} />

          <List>
            <List.Item>
              <span style={{ fontWeight: "bold" }}>Alarme</span>
            </List.Item>

            <ListGridItem
              label={
                <Badge
                  status={outdoorAlarmEnabled ? "success" : "default"}
                  text="Alarme extérieure"
                />
              }
              description="Active/désactive l'alarme extérieure"
              isMobile={isMobile}
            >
              <TwoStatesSelector
                value={outdoorAlarmEnabled}
                toggleButton={false}
                onStateChange={onAlarmChange}
              />
            </ListGridItem>

            <ListGridItem
              label="Sirène extérieure"
              description="Sirène extérieure active"
              isMobile={isMobile}
            >
              {outdoorSirenActive ? (
                <Button
                  type="default"
                  loading={outdoorSirenActive && sirenForceOffRequested}
                  danger
                  onClick={onSirenForceOff}
                >
                  Désactiver
                </Button>
              ) : (
                <Tag icon={<MinusCircleOutlined />} color="default">
                  Sirène inactive
                </Tag>
              )}
            </ListGridItem>

            <List.Item>
              <span style={{ fontWeight: "bold" }}>Lumières</span>
            </List.Item>

            <ListGridItem label="Lumières" description="Lumières extérieures" isMobile={isMobile}>
              <TwoStatesSelector
                disabledIfValueUndefined={false}
                onStateChange={onAllLightChange}
              />
            </ListGridItem>

            <ListGridItem
              label={
                <Badge status={outdoorAlarmSouthLight ? "success" : "default"} text="Lum ext Sud" />
              }
              description="Lumière extérieure Sud"
              isMobile={isMobile}
            >
              <TwoStatesSelector
                value={outdoorAlarmSouthLight}
                onStateChange={onSouthLightChange}
              />
            </ListGridItem>

            <ListGridItem
              label={
                <Badge status={outdoorAlarmEastLight ? "success" : "default"} text="Lum ext Est" />
              }
              description="Lumière extérieure Est"
              isMobile={isMobile}
            >
              <TwoStatesSelector value={outdoorAlarmEastLight} onStateChange={onEastLightChange} />
            </ListGridItem>

            {uiDebugMode && (
              <>
                <List.Item>
                  <span style={{ fontWeight: "bold" }}>Alarme programmée</span>
                </List.Item>

                <ListGridItem
                  label="Auto activation"
                  description="Activation l'alarme extérieure à une heure programmée"
                  isMobile={isMobile}
                >
                  <Badge
                    status={outdoorAlarmState?.getAlarmAutoEnabled() ? "success" : "default"}
                  />
                </ListGridItem>

                <ListGridItem
                  label="Activation"
                  description="Heure d'activation de l'alarme programmée"
                  isMobile={isMobile}
                >
                  H {outdoorAlarmState?.getAlarmAutoEnableTime()}
                </ListGridItem>
                <ListGridItem
                  label="Désactivation"
                  description="Heure de désactivation de l'alarme programmée"
                  isMobile={isMobile}
                >
                  H {outdoorAlarmState?.getAlarmAutoDisableTime()}
                </ListGridItem>
                <ListGridItem
                  label="Délai sirènes consécutives"
                  description="Délai minimum entre deux activations de la sirène"
                  isMobile={isMobile}
                >
                  {outdoorAlarmState?.getAlarmSirenMinimumIntervalSeconds()} secondes
                </ListGridItem>

                <List.Item>
                  <span style={{ fontWeight: "bold" }}>Eclairage automatique</span>
                </List.Item>

                <ListGridItem
                  label="Auto activation"
                  description="Activation automatique des lumières la nuit"
                  isMobile={isMobile}
                >
                  <Badge
                    status={outdoorAlarmState?.getLightsAutoEnabled() ? "success" : "default"}
                  />
                </ListGridItem>

                <ListGridItem
                  label="Activation"
                  description="Heure début des lumières automatiques"
                  isMobile={isMobile}
                >
                  H {outdoorAlarmState?.getLightsAutoEnableTime()}
                </ListGridItem>
                <ListGridItem
                  label="Désactivation"
                  description="Heure de fin des lumières automatiques"
                  isMobile={isMobile}
                >
                  H {outdoorAlarmState?.getLightsAutoDisableTime()}
                </ListGridItem>

                {/* <List.Item>
                  <span style={{ fontWeight: "bold" }}>Statistiques</span>
                </List.Item>

                <ListGridItem
                  label="Détections sud"
                  description="Numbre de détections de mouvement côté sud"
                  isMobile={isMobile}
                >
                  {outdoorAlarmState?.getSouthDetectorTriggeredCount()}
                </ListGridItem>

                <ListGridItem
                  label="Détections est"
                  description="Numbre de détections de mouvement côté est"
                  isMobile={isMobile}
                >
                  {outdoorAlarmState?.getEastDetectorTriggeredCount()}
                </ListGridItem>

                <ListGridItem
                  label="Détections sabotage"
                  description="Numbre de détections de sabotage"
                  isMobile={isMobile}
                >
                  {outdoorAlarmState?.getSabotageTriggeredCount()}
                </ListGridItem>

                <ListGridItem
                  label="Total détections"
                  description="Numbre total de détections (mouvement + sabotage)"
                  isMobile={isMobile}
                >
                  {outdoorAlarmState?.getSignalsTotalCount()}
                </ListGridItem>

                <ListGridItem
                  label="Sirènes déclenchées"
                  description="Numbre total de déclenchements de la sirène"
                  isMobile={isMobile}
                >
                  {outdoorAlarmState?.getSirensTriggeredCount()}
                </ListGridItem> */}
              </>
            )}
          </List>
        </LoadableCard>
      </Col>
      {uiDebugMode && (
        <>
          <Col xl={10} xs={24}>
            <AlarmDiagWidget
              title="Diagnostique alarme"
              alarm={outdoorAlarmState}
              loading={loading}
              navigateTo="/devices/alarms"
              isMobile={isMobile}
            />
          </Col>
          <Col xl={10} xs={24}>
            <DeviceDetailsCard title="Alarme extérieure" device={outdoorAlarmDevice} />
          </Col>
        </>
      )}
    </Row>
  );
}

export default AlarmsView;
