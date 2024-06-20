import { Col, List, Row } from "antd";
import React, { PropsWithChildren } from "react";

interface IListGridItemProps {
  label: string | React.ReactNode;
  description?: string;
  isMobile?: boolean;
}

function ListGridItem({
  label,
  description,
  isMobile = false,
  children,
}: PropsWithChildren<IListGridItemProps>) {
  description = isMobile ? undefined : description;

  const childrenSpan = isMobile ? 14 : 10;
  const labelSpan = isMobile ? 10 : 6;
  const descriptionSpan = 8;

  return (
    <List.Item>
      <Row style={{ width: "100%" }}>
        <Col span={labelSpan}>
          <span>{label}</span>
        </Col>
        <Col span={childrenSpan}>{children}</Col>
        {description && (
          <Col span={descriptionSpan}>
            <p style={{ margin: 0 }}>{description}</p>
          </Col>
        )}
      </Row>
    </List.Item>
  );
}

export default ListGridItem;
