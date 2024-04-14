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