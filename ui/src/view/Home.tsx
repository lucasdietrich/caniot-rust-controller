import { Row, Col, Card, Button, List, Typography } from "antd";
import Hello from "../components/Hello";
import { ReloadOutlined } from "@ant-design/icons";
import HelloCard from "./HelloCard";
import ListLabelledItem from "../components/ListLabelledItem";

function Home() {
  return (
    <>
      <Row gutter={16}>
        <Col span={12}>
          <HelloCard user_name="Lucas" />
        </Col>
        <Col span={12}>
          <Card title="Firmware" bordered={false}>
            <List>
              <ListLabelledItem label="Firmware version">
                v0.1.0-beta
              </ListLabelledItem>
              <ListLabelledItem label="Firmware data">
                04/10/2021 12:00:00
              </ListLabelledItem>
              <ListLabelledItem label="Firmware status">
                <Typography.Text type="success">Running</Typography.Text>
              </ListLabelledItem>
            </List>
          </Card>
        </Col>
      </Row>
      <Row gutter={16} style={{ paddingTop: 20 }}>
        <Col span={12}>
          <HelloCard user_name="Tom" />
        </Col>
      </Row>
    </>
  );
}

export default Home;
