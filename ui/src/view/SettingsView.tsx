import { Button, Card, Col, Divider, List, Row, Slider, Switch } from "antd";
import React, { useEffect, useState } from "react";
import ListLabelledItem from "../components/ListLabelledItem";
import {
  HelloRequest,
  HelloResponse,
  Settings,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_internal_pb";
import internalStore from "../store/InternalStore";
import LoadableCard from "../components/LoadableCard";
import ListGridItem from "../components/ListGridItem";

interface ISettingsProps {
  settings?: Settings;

  setDarkMode?: (darkMode: boolean) => void;
  setDebugMode?: (debugMode: boolean) => void;
  setSettingsReset?: () => void;
}

function SettingsView({
  settings,
  setDarkMode = () => {},
  setDebugMode = () => {},
  setSettingsReset = () => {},
}: ISettingsProps) {
  const loading = settings === undefined;

  const onDebugModeChange = (checked: boolean) => {
    // let newSettings = new Settings();
    // newSettings.setDebugMode(checked);
    // updateSettings(newSettings);
    setDebugMode(checked);
  };

  const onDarkModeChange = (checked: boolean) => {
    // let newSettings = new Settings();
    // newSettings.setDarkMode(checked);
    // updateSettings(newSettings);
    setDarkMode(checked);
  };

  return (
    <Row gutter={24}>
      <Col span={16}>
        {" "}
        <LoadableCard title="Settings" loading={loading}>
          <List>
            <List.Item>
              <span style={{ fontWeight: "bold" }}>Général</span>
            </List.Item>
            <ListGridItem label="Debug" description="Active le mode de debug">
              <Switch checked={settings?.getDebugMode()} onChange={onDebugModeChange} />
            </ListGridItem>
            <ListGridItem label="Mode nuit" description="Active le dark mode">
              <Switch checked={settings?.getDarkMode()} onChange={onDarkModeChange} />
            </ListGridItem>
            <ListGridItem
              label="Actif si"
              description="Temps jusqu'auquel un capteur est considéré actif"
            >
              <Slider
                defaultValue={60}
                style={{
                  width: "80%",
                }}
                tooltip={{
                  formatter: (value) => `${value} s`,
                }}
                step={10}
                max={300}
                min={10}
                marks={{
                  30: "30s",
                  60: "1m",
                  120: "2m",
                  300: "5m",
                }}
              />
            </ListGridItem>

            <List.Item>
              <span style={{ fontWeight: "bold" }}>Actions</span>
            </List.Item>
            <ListGridItem
              label="Réinitialiser "
              description="Réinitialiser les paramètres aux valeurs usine"
            >
              <Button type="primary" danger onClick={setSettingsReset} disabled={loading}>
                Réinitialiser les paramètres
              </Button>
            </ListGridItem>
          </List>
        </LoadableCard>
      </Col>
      <Col span={12}></Col>
    </Row>
  );
}

export default SettingsView;
