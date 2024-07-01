import { Menu as AntMenu, Badge } from "antd";
import { Link } from "react-router-dom";
import type { MenuProps } from "antd";
import {
  HomeOutlined,
  PercentageOutlined,
  ControlOutlined,
  SettingFilled,
  PlusOutlined,
} from "@ant-design/icons";
import { Settings } from "@caniot-controller/caniot-api-grpc-web/api/ng_internal_pb";
import { LuSiren } from "react-icons/lu";
import { MdOutlineGarage } from "react-icons/md";
import { LiaTemperatureLowSolid } from "react-icons/lia";

interface IMenuProps {
  isMobile?: boolean;
  uiDebugMode?: boolean;
}

function AppMenu({ isMobile = false, uiDebugMode = false }: IMenuProps) {
  // Disable the tooltip for now
  const titleTooltipEnable = "";

  const menuItems: MenuProps["items"] = [
    {
      key: "home",
      icon: <HomeOutlined />,
      label: <Link to="/">Aperçu</Link>,
      title: titleTooltipEnable,
    },
    {
      key: "garage",
      label: <Link to="/devices/garage">Portes de garage</Link>,
      icon: <MdOutlineGarage />,
      title: titleTooltipEnable,
    },
    {
      key: "alarms",
      label: <Link to="/devices/alarms">Alarmes</Link>,
      icon: <LuSiren />,
      title: titleTooltipEnable,
    },
    {
      key: "heaters",
      label: <Link to="/devices/heaters">Chauffages</Link>,
      icon: <LiaTemperatureLowSolid />,
      title: titleTooltipEnable,
    },
    {
      key: "settings",
      label: <Link to="/settings">Configuration</Link>,
      icon: <SettingFilled />,
      title: titleTooltipEnable,
    },
  ];

  if (uiDebugMode) {
    menuItems.push({
      key: "misc",
      icon: <PlusOutlined />,
      label: "Misc",
      children: [
        { key: "devices", label: <Link to="/devices">Appareils</Link>, title: titleTooltipEnable },

        {
          key: "debug",
          label: <Link to="/debug">Debug</Link>,
          icon: <ControlOutlined />,
          title: titleTooltipEnable,
        },
        { key: "demo", label: <Link to="/demo">Ant Demos</Link>, title: titleTooltipEnable },
        {
          key: "measures",
          icon: <PercentageOutlined />,
          label: <Link to="/sensors">Mesures</Link>,
          title: titleTooltipEnable,
        },
      ],
    });
  }

  // No menu open by default on mobile
  const defaultOpenKeys = isMobile ? [] : ["misc"];

  return (
    <AntMenu
      mode="inline"
      defaultSelectedKeys={["home"]} // make this dynamic depending on the current route
      defaultOpenKeys={defaultOpenKeys}
      style={{ height: "100%" }}
      items={menuItems}
    />
  );
}

export default AppMenu;
