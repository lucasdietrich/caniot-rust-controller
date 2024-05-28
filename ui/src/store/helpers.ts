// Copied and adapted from chirpstack
// MIT License
// https://github.com/chirpstack/chirpstack/blob/master/LICENSE

import { notification } from "antd";
import { RpcError } from "grpc-web";

export function HandleError(e: RpcError) {
  console.log("API error: ", e);

  notification.error({
    message: "Error",
    description: e.message,
    duration: 3,
  });
}

export function HandleSuccess(message: string) {
  // notification.success({
  //   message: message,
  //   duration: 3,
  // });
}

// http://localhost:50051
// http://192.168.10.53:50051
export function getApiUrl(): string {
  const port = import.meta.env.VITE_API_PORT;
  const host = window.location.hostname;
  return `http://${host}:${port}`;
}
