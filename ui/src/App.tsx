import React, { useEffect, useState } from "react";
import { ConfigProvider, FloatButton, Layout, Switch, theme } from "antd";

import { Routes, Route } from "react-router-dom";
import Home from "./view/Home";
import About from "./view/About";
import DevicesView from "./view/DevicesView";
import AppMenu from "./components/Menu";
import HeatersView from "./view/HeatersView";
import GarageDoorsView from "./view/GarageDoorsView";
import AlarmsView from "./view/AlarmsView";
import SettingsView from "./view/SettingsView";
import NoMatch from "./view/NoMatch";
import Debug from "./view/Debug";
import DemoView from "./view/DemoView";

import "./App.css";
import {
  PartialSettings,
  Settings,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_internal_pb";
import internalStore from "./store/InternalStore";

const { Content, Sider } = Layout;

const App: React.FC = () => {
  const [settings, setSettings] = useState<Settings | undefined>(undefined);
  const [darkMode, setDarkMode] = useState(true);

  useEffect(() => {
    internalStore.getSettings((resp) => {
      setSettings(resp);
      setDarkMode(resp.getDarkMode());
      console.log("Loaded settings", resp.toObject());
    });
  }, []);

  const onDarkModeChange = (checked: boolean) => {
    let newSettings = new PartialSettings();
    newSettings.setDarkMode(checked);
    internalStore.setSettings(newSettings, (resp) => {
      setSettings(resp);
      setDarkMode(resp.getDarkMode());
    });
  };

  const onDebugModeChange = (checked: boolean) => {
    let newSettings = new PartialSettings();
    newSettings.setDebugMode(checked);
    internalStore.setSettings(newSettings, (resp) => {
      setSettings(resp);
    });
  };

  const onSettingsReset = () => {
    internalStore.resetSettings((resp) => {
      setSettings(resp);
      setDarkMode(resp.getDarkMode());
    });
  };

  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();

  return (
    <ConfigProvider
      theme={{
        algorithm: darkMode ? theme.darkAlgorithm : theme.defaultAlgorithm,
      }}
    >
      <Layout>
        <Sider width={200} style={{ background: colorBgContainer }}>
          <AppMenu settings={settings} />
        </Sider>
        <Layout style={{ padding: "24px 24px 24px" }}>
          <Content
            style={{
              margin: 0,
              // padding: 24,
              minHeight: 280,
              // background: colorBgContainer,
              borderRadius: borderRadiusLG,
            }}
          >
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/devices" element={<DevicesView />} />
              <Route path="/about" element={<About />} />
              {settings?.getDebugMode() && <Route path="/debug" element={<Debug />} />}
              <Route path="/devices/heaters" element={<HeatersView />} />
              <Route path="/devices/garage" element={<GarageDoorsView refreshInterval={1000} />} />
              <Route path="/devices/alarms" element={<AlarmsView />} />
              <Route
                path="/settings"
                element={
                  <SettingsView
                    settings={settings}
                    setDarkMode={onDarkModeChange}
                    setDebugMode={onDebugModeChange}
                    setSettingsReset={onSettingsReset}
                  />
                }
              />
              <Route path="/demo" element={<DemoView />} />
              <Route path="*" element={<NoMatch />} />
            </Routes>
          </Content>
        </Layout>
      </Layout>
    </ConfigProvider>
  );
};

export default App;
