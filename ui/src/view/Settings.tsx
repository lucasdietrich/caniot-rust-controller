import { Card, Divider, List, Switch } from "antd";
import React from "react";
import ListLabelledItem from "../components/ListLabelledItem";

interface ISettingsProps {
  darkMode: boolean;
  debugMode: boolean;
  setDarkMode: (darkMode: boolean) => void;
  setDebugMode: (debugMode: boolean) => void;
}

function Settings({
  darkMode,
  debugMode = false,
  setDarkMode,
  setDebugMode = () => {},
}: ISettingsProps) {
  return (
    <Card title="Settings">
      <List>
        <ListLabelledItem label="Debug">
          <Switch defaultChecked={debugMode} onChange={(checked) => setDebugMode(checked)} />
        </ListLabelledItem>
        <ListLabelledItem label="Dark Mode">
          <Switch defaultChecked={darkMode} onChange={(checked) => setDarkMode(checked)} />
        </ListLabelledItem>
      </List>
    </Card>
  );
}

export default Settings;
