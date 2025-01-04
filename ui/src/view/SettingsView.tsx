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
import { Link } from "react-router-dom";

interface ISettingsProps {
  settings?: Settings;

  UIDarkMode?: boolean;
  UIDebugMode?: boolean;
  UIHomeBLEDevices?: boolean;

  setDarkMode?: (darkMode: boolean) => void;
  setDebugMode?: (debugMode: boolean) => void;
  setUIHomeBLEDevices?: (homeBLEDevices: boolean) => void;
  setSettingsReset?: () => void;
  setStatsMinMaxReset?: () => void;

  isMobile?: boolean;
}

function SettingsView({
  settings,
  UIDarkMode = false,
  UIDebugMode = false,
  UIHomeBLEDevices = false,
  setDarkMode = () => {},
  setDebugMode = () => {},
  setSettingsReset = () => {},
  setStatsMinMaxReset = () => {},
  setUIHomeBLEDevices = () => {},
  isMobile = false,
}: ISettingsProps) {
  const loading = settings === undefined;

  return (
    <Row gutter={24}>
      <Col xl={14} xs={24}>
        {" "}
        <LoadableCard title="Settings" loading={loading} isMobile={isMobile}>
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
            <ListGridItem
              label="Accueil BLE"
              description="Affiche les devices BLE sur la page d'accueil"
            >
              <Switch checked={UIHomeBLEDevices} onChange={setUIHomeBLEDevices} />
            </ListGridItem>

            <>
              <List.Item>
                <span style={{ fontWeight: "bold" }}>Actions</span>
              </List.Item>
              <ListGridItem label="Stats min/max" description="Remettre à zéro les stats min/max">
                <Button type="primary" danger onClick={setStatsMinMaxReset} disabled={loading}>
                  Remettre à zéro
                </Button>
              </ListGridItem>
              {UIDebugMode && (
                <ListGridItem
                  label="Réinitialiser "
                  description="Réinitialiser les paramètres aux valeurs usine"
                >
                  <Button type="primary" danger onClick={setSettingsReset} disabled={loading}>
                    Réinitialiser les paramètres
                  </Button>
                </ListGridItem>
              )}
            </>
          </List>
          <List.Item>
            <span style={{ fontWeight: "bold" }}>Misc</span>
          </List.Item>
          <ListGridItem label="Prometheus metrics" description="Lien vers les metrics Prometheus">
            <Link to="/metrics" target="_blank">
              /metrics
            </Link>
          </ListGridItem>
        </LoadableCard>
      </Col>
      <Col xl={14} xs={24}></Col>
    </Row>
  );
}

export default SettingsView;
