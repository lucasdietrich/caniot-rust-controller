import { Card, Space } from "antd";
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
