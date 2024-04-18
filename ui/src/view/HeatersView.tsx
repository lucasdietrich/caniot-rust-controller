import { Badge, Card, Col, Form, Result, Row } from "antd";
import React from "react";
import HeaterModeSelector from "../components/HeaterModeSelector";
import { CheckCircleFilled, CheckCircleOutlined } from "@ant-design/icons";

function HeatersView() {
  return (
    <>
      <Row gutter={16}>
        <Col span={16}>
          <Card title={<Badge status="success" text="Chauffages" />}>
            <HeaterModeSelector name="Chauffage 1"></HeaterModeSelector>
            <HeaterModeSelector name="Chauffage 2"></HeaterModeSelector>
            <HeaterModeSelector
              name="Chauffage 3"
              disabled
            ></HeaterModeSelector>
            <HeaterModeSelector
              name="Chauffage 4"
              disabled
            ></HeaterModeSelector>
          </Card>
        </Col>
        <Col span={8}>
          <Card title="Test"></Card>
        </Col>
      </Row>
    </>
  );
}

export default HeatersView;
