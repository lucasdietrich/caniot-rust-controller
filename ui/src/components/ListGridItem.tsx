import { Col, List, Row } from "antd";
import React, { PropsWithChildren } from "react";

interface IListGridItemProps {
  label: string | React.ReactNode;
  description?: string;
}

function ListGridItem({ label, description, children }: PropsWithChildren<IListGridItemProps>) {
  return (
    <List.Item>
      <Row style={{ width: "100%" }}>
        <Col span={6}>
          <span>{label}</span>
        </Col>
        <Col span={10}>{children}</Col>
        {description && (
          <Col span={8}>
            <p style={{ margin: 0 }}>{description}</p>
          </Col>
        )}
      </Row>
    </List.Item>
  );
}

export default ListGridItem;
