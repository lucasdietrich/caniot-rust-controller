import { StopOutlined } from "@ant-design/icons";
import {
  DeviceAlertType,
  DeviceAlert as gDeviceAlert,
} from "@caniot-controller/caniot-api-grpc-web/api/common_pb";
import { Alert, Button } from "antd";
import React from "react";
import { useNavigate } from "react-router-dom";

interface DeviceAlertProps {
  alert?: gDeviceAlert;
  navigateToController?: string;
  closable?: boolean;
  isMobile?: boolean;
}

const alertTypeMapping: { [key in DeviceAlertType]?: "info" | "success" | "warning" | "error" } = {
  [DeviceAlertType.OK]: "success",
  [DeviceAlertType.NOTIFICATION]: "info",
  [DeviceAlertType.WARNING]: "warning",
  [DeviceAlertType.INHIBITTED]: "warning",
  [DeviceAlertType.INERROR]: "error",
};

function DeviceAlert({
  alert,
  navigateToController = undefined,
  closable = true,
  isMobile = false,
}: DeviceAlertProps) {
  const navigate = useNavigate();

  const gAlertType = alert?.getAlertType();
  const alertType = gAlertType !== undefined ? alertTypeMapping[gAlertType] : undefined;

  let icon = undefined;
  if (gAlertType === DeviceAlertType.INHIBITTED) {
    icon = <StopOutlined />;
  }

  if (!alert || !alertType) {
    return null;
  }

  let marginBottom = 10;
  let paddingTopBottom = 8;

  if (isMobile) {
    marginBottom = 5;

    if (!alert.hasDescription()) {
      paddingTopBottom = 2;
    }
  }

  return (
    <Alert
      message={alert.getMessage()}
      description={alert.getDescription()}
      type={alertType}
      showIcon
      closable={closable}
      icon={icon}
      action={
        navigateToController && (
          <Button type="link" onClick={() => navigate(navigateToController)}>
            Intervenir
          </Button>
        )
      }
      style={{
        marginBottom,
        paddingTop: paddingTopBottom,
        paddingBottom: paddingTopBottom,
      }}
    />
  );
}

export default DeviceAlert;
