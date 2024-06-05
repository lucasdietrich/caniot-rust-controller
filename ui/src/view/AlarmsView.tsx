import { Alert, Badge, Button, Col, List, Row, Tag } from "antd";
import React, { useEffect, useState } from "react";
import DeviceStatusCard from "../components/DeviceStatusCard";
import { Device } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import devicesStore from "../store/DevicesStore";
import LoadableCard from "../components/LoadableCard";
import ListGridItem from "../components/ListGridItem";
import TwoStatesSelector, { TwoStateCommand } from "../components/TwoStatesSelector";
import ListLabelledItem from "../components/ListLabelledItem";
import LastSeenSecondsCounter from "../components/LastSeenSecondsCounter";
import {
  OutdoorAlarmCommand,
  OutdoorAlarmLightsCommand,
  OutdoorAlarmState,
  TwoStates as gTwoStates,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_alarms_pb";
import alarmsStore from "../store/AlarmsStore";
import { MinusCircleOutlined, SyncOutlined } from "@ant-design/icons";

interface IAlarmsViewProps {
  refreshInterval?: number;
}

function AlarmsView({ refreshInterval = 5000 }: IAlarmsViewProps) {
  const [outdoorAlarmDevice, setOutdoorAlarmDevice] = useState<Device | undefined>(undefined);
  const [outdoorAlarmState, setOutdoorAlarmState] = useState<OutdoorAlarmState | undefined>(
    undefined
  );
  const [loading, setLoading] = useState(true);
  const [time, setTime] = useState(Date.now());

  useEffect(() => {
    setLoading(true);
    devicesStore.getOutdoorAlarmDevice((resp: Device) => {
      setOutdoorAlarmDevice(resp);
      alarmsStore.getOutdoorAlarmState((resp: OutdoorAlarmState) => {
        setOutdoorAlarmState(resp);
        console.log("Outdoor alarm state", resp.toObject());
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
    setLoading(true);
    alarmsStore.sendOutdoorAlarmCommand(command, (resp) => {
      setOutdoorAlarmState(resp);
      devicesStore.getOutdoorAlarmDevice((resp: Device) => {
        setOutdoorAlarmDevice(resp);
        setLoading(false);
      });
    });
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

    let lightsCommand = new OutdoorAlarmLightsCommand();
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

  const onSouthLightChange = (ts: TwoStateCommand) => handleLightAction(Light.South, ts);
  const onEastLightChange = (ts: TwoStateCommand) => handleLightAction(Light.East, ts);
  const onAllLightChange = (ts: TwoStateCommand) => handleLightAction(Light.All, ts);

  const onAlarmChange = (ts: TwoStateCommand) => handleAlarmAction(ts);

  const outdoorAlarmEnabled = outdoorAlarmState?.getEnabled();
  const outdoorSirenActive = outdoorAlarmState?.getDevice()?.getSirenActive();

  const outdoorAlarmEastLight = outdoorAlarmState?.getDevice()?.getEastLight();
  const outdoorAlarmSouthLight = outdoorAlarmState?.getDevice()?.getSouthLight();

  return (
    <>
      <Row gutter={16}>
        <Col span={14}>
          <LoadableCard
            title="Alarme extérieure"
            status={undefined}
            loading={loading}
            onRefresh={() => {
              setLoading(true);
              setTime(Date.now());
            }}
          >
            {outdoorSirenActive && (
              // todo list detectors active
              <Alert message={"Sirène extérieure active"} type="warning" showIcon />
            )}

            <List>
              <List.Item>
                <span style={{ fontWeight: "bold" }}>Lumières</span>
              </List.Item>

              <ListGridItem label="Lumières" description="Lumières extérieures">
                <TwoStatesSelector
                  disabledIfValueUndefined={false}
                  onStateChange={onAllLightChange}
                />
              </ListGridItem>

              <ListGridItem
                label={
                  <Badge
                    status={outdoorAlarmSouthLight ? "success" : "default"}
                    text="Lum ext Sud"
                  />
                }
                description="Lumière extérieure Sud"
              >
                <TwoStatesSelector
                  value={outdoorAlarmSouthLight}
                  onStateChange={onSouthLightChange}
                />
              </ListGridItem>

              <ListGridItem
                label={
                  <Badge
                    status={outdoorAlarmEastLight ? "success" : "default"}
                    text="Lum ext Est"
                  />
                }
                description="Lumière extérieure Est"
              >
                <TwoStatesSelector
                  value={outdoorAlarmEastLight}
                  onStateChange={onEastLightChange}
                />
              </ListGridItem>

              <List.Item>
                <span style={{ fontWeight: "bold" }}>Alarme</span>
              </List.Item>

              <ListGridItem label="Sirène extérieure" description="Sirène extérieure active">
                {outdoorSirenActive ? (
                  <Button type="default" loading danger onClick={() => {}}>
                    Désactiver
                  </Button>
                ) : (
                  <Tag icon={<MinusCircleOutlined />} color="default">
                    Sirène inactive
                  </Tag>
                )}
              </ListGridItem>

              <ListGridItem
                label={
                  <Badge
                    status={outdoorAlarmEnabled ? "success" : "default"}
                    text="Alarme extérieure"
                  />
                }
                description="Active/désactive l'alarme extérieure"
              >
                <TwoStatesSelector
                  value={outdoorAlarmEnabled}
                  toggleButton={false}
                  onStateChange={onAlarmChange}
                />
              </ListGridItem>
            </List>
          </LoadableCard>
        </Col>
        <Col span={10}>
          <DeviceStatusCard title="Alarme extérieure" device={outdoorAlarmDevice} />
        </Col>
      </Row>
    </>
  );
}

export default AlarmsView;
