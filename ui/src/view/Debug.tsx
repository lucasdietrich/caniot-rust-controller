import { Row, Col, Card, Button, List, Typography, Space } from "antd";
import Hello from "../components/Hello";
import { ReloadOutlined } from "@ant-design/icons";
import HelloCard from "./HelloCard";
import ListLabelledItem from "../components/ListLabelledItem";
import CaniotFramesTable from "../components/CaniotFramesTable";
import CaniotQueryForm from "../components/CaniotQueryForm";

function Debug() {
  return (
    <>
      <Space direction="vertical" size="middle" style={{ display: "flex" }}>
        <Card title="Query CANIOT">
          <CaniotQueryForm></CaniotQueryForm>
        </Card>
        <Card>
          <CaniotFramesTable></CaniotFramesTable>
        </Card>
      </Space>
    </>
  );
}

export default Debug;
