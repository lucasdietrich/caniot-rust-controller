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
      { key: "1", label: <Link to="/">Overview</Link> },
      { key: "2", label: <Link to="/devices">Devices</Link> },
    ],
  },
  {
    key: "devices",
    icon: <ControlOutlined />,
    label: "Devices",
    children: [
      { key: "3", label: <Link to="/devices/heaters">Heaters</Link> },
      { key: "4", label: <Link to="/devices/garage">Garage Doors</Link> },
      { key: "5", label: <Link to="/devices/alarms">Alarms</Link> },
    ],
  },
  {
    key: "measures",
    icon: <PercentageOutlined />,
    label: "Measures",
    children: [{ key: "6", label: <Link to="/sensors">Sensors</Link> }],
  },
  {
    key: "misc",
    icon: <SettingFilled />,
    label: "Misc",
    children: [
      { key: "7", label: <Link to="/settings">Settings</Link> },
      { key: "8", label: <Link to="/about">About</Link> },
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
