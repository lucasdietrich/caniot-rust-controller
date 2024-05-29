import { LoadingOutlined, ReloadOutlined } from "@ant-design/icons";
import { Badge, Button, Card, Progress, Space, Spin } from "antd";
import React, { PropsWithChildren } from "react";

interface ICardLoadableProps {
  title?: React.ReactNode;
  status?: boolean;
  loading?: boolean;
  onRefresh?: () => void;
  progress?: number;
}

function LoadableCard({
  title,
  status,
  loading,
  onRefresh,
  progress: progressToNextRefresh,
  children,
}: PropsWithChildren<ICardLoadableProps>) {
  let titleComponent = (
    <Space size="middle">
      {title}
      <Spin spinning={loading} indicator={<LoadingOutlined spin />} />
    </Space>
  );

  let extraComponent;

  if (progressToNextRefresh !== undefined && onRefresh !== undefined) {
    extraComponent = (
      <Button
        onClick={onRefresh}
        icon={<Progress type="circle" percent={progressToNextRefresh} size={20} />}
      />
    );
  } else if (onRefresh !== undefined) {
    extraComponent = <Button onClick={onRefresh} icon={<ReloadOutlined />} />;
  } else if (progressToNextRefresh !== undefined) {
    extraComponent = (
      <Progress
        type="circle"
        percent={progressToNextRefresh}
        size={20}
        style={{
          marginLeft: 10,
          verticalAlign: "middle",
        }}
      />
    );
  } else {
    extraComponent = undefined;
  }

  return (
    <Card
      title={
        status === undefined ? (
          <span>{titleComponent}</span>
        ) : (
          <Badge status={status ? "success" : "error"} text={titleComponent} />
        )
      }
      extra={extraComponent}
    >
      {children}
    </Card>
  );
}

export default LoadableCard;