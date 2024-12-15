import React, { Fragment, useEffect, useState } from "react";
import { ConfigProvider, FloatButton, Layout, Switch, theme } from "antd";

import { Routes, Route } from "react-router-dom";
import HomeView from "./view/HomeView";
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
import Media from "react-media";

import "./App.css";
import {
  PartialSettings,
  Settings,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_internal_pb";
import internalStore from "./store/InternalStore";
import sessionStore from "./store/SessionStore";
import EmulationView from "./view/EmulationView";
import BleDevicesView from "./view/BleDevicesView";

const { Content, Sider } = Layout;

const MobileMaxSize = 700;

const App: React.FC = () => {
  const [settings, setSettings] = useState<Settings | undefined>(undefined);
  const [width, setWidth] = useState<number>(window.innerWidth);
  const [UIDarkMode, setUIDarkMode] = useState(false);
  const [UIDebugMode, setUIDebugMode] = useState(false);
  const [UIHomeBLEDevices, setUIHomeBLEDevices] = useState(false);

  function handleWindowSizeChange() {
    setWidth(window.innerWidth);
  }

  const isMobile = width <= MobileMaxSize;

  useEffect(() => {
    // set page title
    document.title = "CANIOT Controller";

    window.addEventListener("resize", handleWindowSizeChange);

    setUIDarkMode(sessionStore.getUIDarkMode() || false);
    setUIDebugMode(sessionStore.getUIDebugMode() || false);
    setUIHomeBLEDevices(sessionStore.getUIHomeBLEDevices() || false);

    internalStore.getSettings((resp) => {
      setSettings(resp);
    });

    return () => {
      window.removeEventListener("resize", handleWindowSizeChange);
    };
  }, []);

  const onDarkModeChange = (checked: boolean) => {
    setUIDarkMode(checked);
    sessionStore.setUIDarkMode(checked);
  };

  const onDebugModeChange = (checked: boolean) => {
    setUIDebugMode(checked);
    sessionStore.setUIDebugMode(checked);
  };

  const onHomeBLEDevicesChange = (checked: boolean) => {
    setUIHomeBLEDevices(checked);
    sessionStore.setUIHomeBLEDevices(checked);
  };

  const onSettingsReset = () => {
    internalStore.resetSettings((resp) => {
      setSettings(resp);
    });
  };

  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();

  const SiderWidth = 250;
  const SiderMobileWidth = 50;

  return (
    <ConfigProvider
      // change sizeXS
      theme={{
        algorithm: UIDarkMode ? theme.darkAlgorithm : theme.defaultAlgorithm,
        token: {
          // sizeXS: SizeXS,
        },
      }}
    >
      <Layout>
        <Sider
          style={{
            background: colorBgContainer,
            position: "fixed",
            left: 0,
            overflow: "auto",
            height: "100vh",
          }}
          collapsed={isMobile}
          collapsedWidth={SiderMobileWidth}
          width={SiderWidth}
        >
          <AppMenu isMobile={isMobile} uiDebugMode={UIDebugMode} />
        </Sider>
        <Layout
          style={{
            padding: isMobile ? "8px 8px 8px" : "24px 24px 24px",
            marginLeft: isMobile ? SiderMobileWidth : SiderWidth,
          }}
        >
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
              <Route
                path="/"
                element={
                  <HomeView
                    isMobile={isMobile}
                    uiDebugMode={UIDebugMode}
                    uiHomeBLEDevices={UIHomeBLEDevices}
                  />
                }
              />
              <Route path="/devices" element={<DevicesView />} />
              <Route path="/about" element={<About />} />
              {UIDebugMode && <Route path="/debug" element={<Debug />} />}
              <Route
                path="/devices/heaters"
                element={<HeatersView isMobile={isMobile} uiDebugMode={UIDebugMode} />}
              />
              <Route
                path="/devices/garage"
                element={
                  <GarageDoorsView
                    isMobile={isMobile}
                    uiDebugMode={UIDebugMode}
                    refreshInterval={1000}
                  />
                }
              />
              <Route
                path="/garage"
                element={
                  <GarageDoorsView
                    isMobile={isMobile}
                    uiDebugMode={UIDebugMode}
                    refreshInterval={1000}
                  />
                }
              />{" "}
              {/* Alias to keep compat with caniot-pycontroller */}
              <Route
                path="/devices/alarms"
                element={<AlarmsView isMobile={isMobile} uiDebugMode={UIDebugMode} />}
              />
              <Route
                path="/ble"
                element={
                  <BleDevicesView
                    isMobile={isMobile}
                    uiDebugMode={UIDebugMode}
                    refreshInterval={5000}
                  />
                }
              />
              <Route
                path="/settings"
                element={
                  <SettingsView
                    settings={settings}
                    UIDarkMode={UIDarkMode}
                    UIDebugMode={UIDebugMode}
                    UIHomeBLEDevices={UIHomeBLEDevices}
                    setDarkMode={onDarkModeChange}
                    setDebugMode={onDebugModeChange}
                    setSettingsReset={onSettingsReset}
                    setUIHomeBLEDevices={onHomeBLEDevicesChange}
                    isMobile={isMobile}
                  />
                }
              />
              <Route path="/demo" element={<DemoView />} />
              <Route path="/emulation" element={<EmulationView isMobile={isMobile} />} />
              <Route path="*" element={<NoMatch />} />
            </Routes>
          </Content>
        </Layout>
      </Layout>
    </ConfigProvider>
  );
};

export default App;
export { MobileMaxSize };
