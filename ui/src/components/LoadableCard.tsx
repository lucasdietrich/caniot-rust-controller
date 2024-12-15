import {
  CaretRightFilled,
  CaretRightOutlined,
  InfoCircleOutlined,
  LoadingOutlined,
  ReloadOutlined,
  RightSquareOutlined,
  SettingFilled,
  SettingOutlined,
} from "@ant-design/icons";
import { Badge, Button, Card, Progress, Space, Spin } from "antd";
import React, { PropsWithChildren } from "react";
import { LiaHandPaper } from "react-icons/lia";

interface ICardLoadableProps {
  title?: React.ReactNode;
  status?: boolean;
  loading?: boolean;
  onRefresh?: () => void;
  progress?: number;

  // Button to go to a specific page
  onGoto?: () => void;

  bordered?: boolean;
  isMobile?: boolean;

  className?: string;

  cardStyle?: React.CSSProperties;
}

function LoadableCard({
  title = undefined,
  status = undefined,
  loading = undefined,
  onRefresh = undefined,
  progress = undefined,
  onGoto = undefined,
  cardStyle = undefined,
  bordered = true,
  isMobile = false,
  className = undefined,
  children,
}: PropsWithChildren<ICardLoadableProps>) {
  let titleComponent = (
    <Space size="middle">
      {title}
      <Spin spinning={loading} indicator={<LoadingOutlined spin />} />
    </Space>
  );

  // Build refresh button
  const refreshButton = onRefresh ? (
    <Button
      disabled={loading}
      onClick={onRefresh}
      icon={
        progress !== undefined ? (
          <Progress type="circle" percent={progress} size={20} />
        ) : (
          <ReloadOutlined />
        )
      }
    />
  ) : progress !== undefined ? (
    <Progress
      type="circle"
      percent={progress}
      size={20}
      style={{ marginLeft: 10, verticalAlign: "middle" }}
    />
  ) : undefined;

  // Build go to button
  const gotoButton = onGoto ? (
    <Button
      type="text"
      onClick={onGoto}
      icon={<LiaHandPaper />} // <RightSquareOutlined /> <LiaHandPaper /> <CaretRightOutlined /> <SettingOutlined />
      style={{ marginLeft: 10 }}
    />
  ) : undefined;

  // Build extra
  const extra = (
    <>
      {refreshButton}
      {gotoButton}
    </>
  );

  return (
    <Card
      bordered={bordered}
      title={
        title &&
        (status === undefined ? (
          <span>{titleComponent}</span>
        ) : (
          <Badge status={status ? "success" : "error"} text={titleComponent} />
        ))
      }
      extra={extra}
      style={cardStyle}
      size={isMobile ? "small" : "default"}
      className={className + " loadable-card"}
    >
      {children}
    </Card>
  );
}

export default LoadableCard;
