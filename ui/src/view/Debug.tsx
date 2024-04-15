import { Row, Col, Card, Button, List, Typography } from "antd";
import Hello from "../components/Hello";
import { ReloadOutlined } from "@ant-design/icons";
import HelloCard from "./HelloCard";
import ListLabelledItem from "../components/ListLabelledItem";

function Debug() {
  return (
    <>
      <Row gutter={16}>
        <Col span={12}>
          <HelloCard user_name="Lucas" />
        </Col>
      </Row>
      <Row gutter={16} style={{ paddingTop: 20 }}></Row>
    </>
  );
}

export default Debug;
