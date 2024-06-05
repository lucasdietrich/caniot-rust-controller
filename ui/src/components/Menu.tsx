import { Menu as AntMenu } from "antd";
import { Link } from "react-router-dom";
import type { MenuProps } from "antd";
import {
  HomeOutlined,
  PercentageOutlined,
  ControlOutlined,
  SettingFilled,
} from "@ant-design/icons";

const menuItems: MenuProps["items"] = [
  {
    key: "home",
    icon: <HomeOutlined />,
    label: "Home",
    children: [
      { key: "1", label: <Link to="/">Aperçu</Link> },
      { key: "2", label: <Link to="/devices">Appareils</Link> },
      { key: "3", label: <Link to="/debug">Debug</Link> },
    ],
  },
  {
    key: "devices",
    icon: <ControlOutlined />,
    label: "Contrôleurs",
    children: [
      { key: "10", label: <Link to="/devices/heaters">Chauffages</Link> },
      { key: "11", label: <Link to="/devices/garage">Portes de garage</Link> },
      { key: "12", label: <Link to="/devices/alarms">Alarmes</Link> },
    ],
  },
  {
    key: "measures",
    icon: <PercentageOutlined />,
    label: "Mesures",
    children: [{ key: "6", label: <Link to="/sensors">Capteurs</Link> }],
  },
  {
    key: "misc",
    icon: <SettingFilled />,
    label: "Autres",
    children: [
      { key: "20", label: <Link to="/settings">Configuration</Link> },
      { key: "21", label: <Link to="/about">Infos</Link> },
      { key: "22", label: <Link to="/demo">Ant Demos</Link> },
    ],
  },
];

function AppMenu() {
  return (
    <AntMenu
      mode="inline"
      defaultSelectedKeys={["1"]}
      defaultOpenKeys={["home", "devices"]}
      style={{ height: "100%" }}
      items={menuItems}
    />
  );
}

export default AppMenu;
