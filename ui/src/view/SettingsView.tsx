import { Card, Col, Divider, List, Row, Switch } from "antd";
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
}

function SettingsView({
  settings,
  setDarkMode = () => {},
  setDebugMode = () => {},
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
      <Col span={12}>
        {" "}
        <LoadableCard title="Settings" loading={loading}>
          <List>
            <ListGridItem label="Debug" description="Active le mode de debug">
              <Switch checked={settings?.getDebugMode()} onChange={onDebugModeChange} />
            </ListGridItem>
            <ListGridItem label="Dark mode" description="Active le dark mode">
              <Switch checked={settings?.getDarkMode()} onChange={onDarkModeChange} />
            </ListGridItem>
          </List>
        </LoadableCard>
      </Col>
      <Col span={12}></Col>
    </Row>
  );
}

export default SettingsView;
