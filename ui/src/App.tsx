import React, { Fragment, useEffect, useState } from "react";
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
import Media from "react-media";

import "./App.css";
import {
  PartialSettings,
  Settings,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_internal_pb";
import internalStore from "./store/InternalStore";

const { Content, Sider } = Layout;

const MobileMaxSize = 700;

const App: React.FC = () => {
  const [width, setWidth] = useState<number>(window.innerWidth);
  const [settings, setSettings] = useState<Settings | undefined>(undefined);
  const [darkMode, setDarkMode] = useState(true);

  function handleWindowSizeChange() {
    setWidth(window.innerWidth);
  }

  const isMobile = width <= MobileMaxSize;

  useEffect(() => {
    window.addEventListener("resize", handleWindowSizeChange);

    internalStore.getSettings((resp) => {
      setSettings(resp);
      setDarkMode(resp.getDarkMode());
      console.log("Loaded settings", resp.toObject());
    });

    return () => {
      window.removeEventListener("resize", handleWindowSizeChange);
    };
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

  const SiderWidth = 250;
  const SiderMobileWidth = 50;

  return (
    <ConfigProvider
      // change sizeXS
      theme={{
        algorithm: darkMode ? theme.darkAlgorithm : theme.defaultAlgorithm,
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
          <AppMenu settings={settings} isMobile={isMobile} />
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
              <Route path="/" element={<Home isMobile={isMobile} />} />
              <Route path="/devices" element={<DevicesView />} />
              <Route path="/about" element={<About />} />
              {settings?.getDebugMode() && <Route path="/debug" element={<Debug />} />}
              <Route path="/devices/heaters" element={<HeatersView isMobile={isMobile} />} />
              <Route path="/devices/garage" element={<GarageDoorsView refreshInterval={1000} />} />
              <Route path="/devices/alarms" element={<AlarmsView isMobile={isMobile} />} />
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
export { MobileMaxSize };
