import { Menu as AntMenu, Badge } from "antd";
import { Link } from "react-router-dom";
import type { MenuProps } from "antd";
import { HomeOutlined, SettingFilled, PlusOutlined, UploadOutlined } from "@ant-design/icons";
import { LuBluetooth, LuSiren } from "react-icons/lu";
import { MdOutlineGarage } from "react-icons/md";
import { LiaTemperatureLowSolid } from "react-icons/lia";
import { SiGrafana, SiPrometheus, SiHomeassistant } from "react-icons/si";
import { useEffect, useSyncExternalStore } from "react";
import uiConfigStore from "../store/UIConfigStore"; // dynamic UI config
import type { ShortcutConfig } from "../store/UIConfigStore";

interface IMenuProps {
  isMobile?: boolean;
  uiDebugMode?: boolean;
}

function AppMenu({ isMobile = false, uiDebugMode = false }: IMenuProps) {
  // Disable the tooltip for now
  const titleTooltipEnable = "";

  const uiConfigState = useSyncExternalStore(
    (cb) => uiConfigStore.subscribe(cb),
    () => uiConfigStore.getState(),
    () => uiConfigStore.getState()
  );

  useEffect(() => {
    uiConfigStore.fetch();
  }, []);

  // Use last part of the URL as the current route
  const currentRoute = window.location.pathname.split("/").pop();

  // Map icon keyword to node and optionally wrap with badge per shortcut config
  const mapIcon = (sc?: ShortcutConfig) => {
    if (!sc?.icon) return undefined;
    const name = sc.icon.toLowerCase();
    let base: React.ReactNode | undefined;
    switch (name) {
      case "grafana":
        base = <SiGrafana />; break;
      case "prometheus":
        base = <SiPrometheus />; break;
      case "homeassistant":
      case "home_assistant":
        base = <SiHomeassistant />; break;
      case "swupdate":
        base = <UploadOutlined />; break;
      default:
        return undefined;
    }
    const wantsBadge = sc.badge ?? ["grafana", "prometheus", "homeassistant", "home_assistant", "swupdate"].includes(name);
    if (!wantsBadge) return base;
    const color = sc.badge_color || (name === "swupdate" ? "blue" : "red");
    return <Badge dot color={color}>{base}</Badge>;
  };

  const createLink = (s: ShortcutConfig) => (
    <a href={s.url} {...(s.target_blank ? { target: "_blank", rel: "noopener noreferrer" } : {})}>{s.name}</a>
  );
  const menuItems: MenuProps["items"] = [
    {
      key: "home",
      icon: <HomeOutlined />,
      label: <Link to="/">Aperçu</Link>,
      title: titleTooltipEnable,
    },
    {
      key: "ble",
      label: <Link to="/ble">capteurs BLE</Link>,
      icon: <LuBluetooth />,
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

  // integrate dynamic shortcuts from backend with explicit badge control
  if (uiConfigState.data?.shortcut) {
    const afterHome: MenuProps["items"] = [];
    const beforeSettings: MenuProps["items"] = [];
    const end: MenuProps["items"] = [];

    uiConfigState.data.shortcut.slice().sort((a, b) => (a.order || 0) - (b.order || 0)).forEach((s) => {
      if (menuItems.find((mi) => mi && mi.key === s.name)) return; // avoid duplicates
      const target = s.menu_order || "before_settings";
      const item: NonNullable<MenuProps["items"]>[number] = {
        key: s.name,
        label: createLink(s),
        icon: mapIcon(s),
        title: titleTooltipEnable,
      };
      switch (target) {
        case "after_home": afterHome.push(item); break;
        case "before_settings": beforeSettings.push(item); break;
        case "end": default: end.push(item); break;
      }
    });

    if (afterHome.length) {
      const homeIdx = menuItems.findIndex((i) => i?.key === "home");
      if (homeIdx >= 0) menuItems.splice(homeIdx + 1, 0, ...afterHome);
      else menuItems.unshift(...afterHome);
    }
    if (beforeSettings.length) {
      const settingsIdx = menuItems.findIndex((i) => i?.key === "settings");
      if (settingsIdx >= 0) menuItems.splice(settingsIdx, 0, ...beforeSettings);
      else menuItems.push(...beforeSettings);
    }
    if (end.length) menuItems.push(...end);
  }

  if (uiDebugMode) {
    menuItems.push({
      key: "misc",
      icon: <PlusOutlined />,
      label: "Misc",
      children: [
        {
          key: "devices",
          label: <Link to="/devices">Appareils</Link>,
          title: titleTooltipEnable,
        },
        {
          key: "demo",
          label: <Link to="/demo">Démo</Link>,
          title: titleTooltipEnable,
        },
      ],
    });
  }

  const defaultOpenKeys = isMobile ? [] : ["misc"];

  return (
    <AntMenu
      mode="inline"
      defaultSelectedKeys={[currentRoute || "home"]} // make this dynamic depending on the current route
      defaultOpenKeys={defaultOpenKeys}
      style={{ height: "100%" }}
      items={menuItems}
    />
  );
}

export default AppMenu;
