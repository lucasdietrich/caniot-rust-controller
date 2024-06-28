import { StopOutlined } from "@ant-design/icons";
import {
  DeviceAlertType,
  DeviceAlert as gDeviceAlert,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import { Alert } from "antd";
import React from "react";

interface DeviceAlertProps {
  alert?: gDeviceAlert;
}

const alertTypeMapping: { [key in DeviceAlertType]?: "info" | "success" | "warning" | "error" } = {
  [DeviceAlertType.OK]: "success",
  [DeviceAlertType.NOTIFICATION]: "info",
  [DeviceAlertType.WARNING]: "warning",
  [DeviceAlertType.INHIBITTED]: "warning",
  [DeviceAlertType.INERROR]: "error",
};

function DeviceAlert({ alert }: DeviceAlertProps) {
  const gAlertType = alert?.getAlertType();
  const alertType = gAlertType !== undefined ? alertTypeMapping[gAlertType] : undefined;

  let icon = undefined;
  if (gAlertType === DeviceAlertType.INHIBITTED) {
    icon = <StopOutlined />;
  }

  if (!alert || !alertType) {
    return null;
  }

  return <Alert message={alert.getMessage()} type={alertType} showIcon closable icon={icon} />;
}

export default DeviceAlert;
