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
  settings?: Settings;
}

function AppMenu({ settings }: IMenuProps) {
  const menuItems: MenuProps["items"] = [
    {
      key: "home",
      icon: <HomeOutlined />,
      label: <Link to="/">Aperçu</Link>,
    },
    {
      key: "garage",
      label: <Link to="/devices/garage">Portes de garage</Link>,
      icon: <MdOutlineGarage />,
    },
    { key: "alarms", label: <Link to="/devices/alarms">Alarmes</Link>, icon: <LuSiren /> },
    {
      key: "heaters",
      label: <Link to="/devices/heaters">Chauffages</Link>,
      icon: <LiaTemperatureLowSolid />,
    },
    {
      key: "settings",
      label: <Link to="/settings">Configuration</Link>,
      icon: <SettingFilled />,
    },
  ];

  if (settings?.getDebugMode()) {
    menuItems.push({
      key: "misc",
      icon: <PlusOutlined />,
      label: "Misc",
      children: [
        { key: "devices", label: <Link to="/devices">Appareils</Link> },

        { key: "debug", label: <Link to="/debug">Debug</Link>, icon: <ControlOutlined /> },
        { key: "demo", label: <Link to="/demo">Ant Demos</Link> },
        {
          key: "measures",
          icon: <PercentageOutlined />,
          label: <Link to="/sensors">Mesures</Link>,
        },
      ],
    });
  }

  return (
    <AntMenu
      mode="inline"
      defaultSelectedKeys={["home"]} // make this dynamic depending on the current route
      defaultOpenKeys={["misc"]}
      style={{ height: "100%" }}
      items={menuItems}
    />
  );
}

export default AppMenu;
