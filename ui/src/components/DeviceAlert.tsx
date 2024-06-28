import { StopOutlined } from "@ant-design/icons";
import {
  DeviceAlertType,
  DeviceAlert as gDeviceAlert,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import { Alert, Button } from "antd";
import React from "react";
import { useNavigate } from "react-router-dom";

interface DeviceAlertProps {
  alert?: gDeviceAlert;
  navigateToController?: string;
  closable?: boolean;
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
            Go to controller
          </Button>
        )
      }
      style={{
        marginBottom: 10,
      }}
    />
  );
}

export default DeviceAlert;
