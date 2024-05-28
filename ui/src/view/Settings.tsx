import { Card, Divider, List, Switch } from "antd";
import React from "react";
import ListLabelledItem from "../components/ListLabelledItem";

interface IProps {
  darkMode: boolean;
  setDarkMode: (darkMode: boolean) => void;
}

function Settings({ darkMode, setDarkMode }: IProps) {
  return (
    <Card title="Settings">
      <List>
        <ListLabelledItem label="Dark Mode">
          <Switch defaultChecked={darkMode} onChange={(checked) => setDarkMode(checked)} />
        </ListLabelledItem>
      </List>
    </Card>
  );
}

export default Settings;
