// Copied and adapted from chirpstack
// MIT License
// https://github.com/chirpstack/chirpstack/blob/master/LICENSE

import { notification } from "antd";
import { RpcError, StatusCode } from "grpc-web";

const MAX_NOTIFICATIONS = 6;
const SUCCESS_DURATION = 2;
const ERROR_DURATION = 5;
const SHOW_SUCCESS = false;
const SHOW_RPC_UNKNOWN_ERROR = false;

let notificationCount = 0;

export function HandleError(e: RpcError) {
  console.log("API error: ", e);
  console.log("API: ", e.code, e.message, e.metadata);

  if (!SHOW_RPC_UNKNOWN_ERROR && e.code == StatusCode.UNKNOWN) {
    return;
  }

  if (notificationCount < MAX_NOTIFICATIONS) {
    notificationCount++;
    notification.error({
      message: e.name,
      description: e.message,
      duration: ERROR_DURATION,
      showProgress: true,
      onClose: () => {
        notificationCount--;
      },
    });
  }
}

export function HandleSuccess(message: string) {
  if (SHOW_SUCCESS) {
    if (notificationCount < MAX_NOTIFICATIONS) {
      notificationCount++;
      notification.success({
        message: message,
        duration: SUCCESS_DURATION,
        showProgress: true,
        onClose: () => {
          notificationCount--;
        },
      });
    }
  }
}

// ant-notification-notice ant-notification-notice-success ant-notification-notice-closable

// http://localhost:50051
// http://192.168.10.53:50051

export function getApiUrl(): string {
  const port = import.meta.env.VITE_API_PORT;
  const host = window.location.hostname;
  return `http://${host}:${port}`;
}
