import React from "react";
import { Layout, theme } from "antd";

import { Routes, Route } from "react-router-dom";
import Home from "./view/Home";
import About from "./view/About";
import Devices from "./view/Devices";
import AppMenu from "./components/Menu";
import Heaters from "./view/Heaters";
import GarageDoors from "./view/GarageDoors";
import Alarms from "./view/Alarms";
import Settings from "./view/Settings";
import NoMatch from "./view/NoMatch";
import Debug from "./view/Debug";

const { Content, Sider } = Layout;

const App: React.FC = () => {
  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();

  return (
    <Layout>
      <Sider width={200} style={{ background: colorBgContainer }}>
        <AppMenu />
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
            <Route path="/devices" element={<Devices />} />
            <Route path="/about" element={<About />} />
            <Route path="/devices/heaters" element={<Heaters />} />
            <Route path="/devices/garage" element={<GarageDoors />} />
            <Route path="/devices/alarms" element={<Alarms />} />
            <Route path="/settings" element={<Settings />} />
            <Route path="/debug" element={<Debug />} />
            <Route path="*" element={<NoMatch />} />
          </Routes>
        </Content>
      </Layout>
    </Layout>
  );
};

export default App;
