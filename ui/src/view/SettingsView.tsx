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

  UIDarkMode?: boolean;
  UIDebugMode?: boolean;

  setDarkMode?: (darkMode: boolean) => void;
  setDebugMode?: (debugMode: boolean) => void;
  setSettingsReset?: () => void;
}

function SettingsView({
  settings,
  UIDarkMode = false,
  UIDebugMode = false,
  setDarkMode = () => {},
  setDebugMode = () => {},
  setSettingsReset = () => {},
}: ISettingsProps) {
  const loading = settings === undefined;

  return (
    <Row gutter={24}>
      <Col xl={14} xs={24}>
        {" "}
        <LoadableCard title="Settings" loading={loading}>
          <List>
            <List.Item>
              <span style={{ fontWeight: "bold" }}>Général (UI)</span>
            </List.Item>
            <ListGridItem label="Debug" description="Active le mode de debug">
              <Switch checked={UIDebugMode} onChange={setDebugMode} />
            </ListGridItem>
            <ListGridItem label="Mode nuit" description="Active le dark mode">
              <Switch checked={UIDarkMode} onChange={setDarkMode} />
            </ListGridItem>
            {UIDebugMode && (
              <>
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
              </>
            )}
          </List>
        </LoadableCard>
      </Col>
      <Col xl={14} xs={24}></Col>
    </Row>
  );
}

export default SettingsView;
