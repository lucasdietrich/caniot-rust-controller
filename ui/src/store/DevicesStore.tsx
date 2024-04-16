import { notification } from "antd";
import EventEmitter from "events";
import { HandleError } from "./helpers";

import { Empty } from "google-protobuf/google/protobuf/empty_pb";
// import google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb.js";

import { DevicesList } from "@caniot-controller/caniot-api-grpc-web/api/ng_pb";

import { CaniotDevicesServiceClient } from "@caniot-controller/caniot-api-grpc-web/api/NgServiceClientPb";

class DevicesStore extends EventEmitter {
  client: CaniotDevicesServiceClient;

  constructor() {
    super();
    this.client = new CaniotDevicesServiceClient("http://localhost:50051");
  }

  getList = (req: Empty, callbackFunc: (resp: DevicesList) => void) => {
    this.client.getList(req, null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "DevicesStore::GetList succeeded",
        duration: 3,
      });

      callbackFunc(resp);
    });
  };
}

const devicesStore = new DevicesStore();
export default devicesStore;
