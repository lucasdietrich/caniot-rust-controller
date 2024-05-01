import { notification } from "antd";
import EventEmitter from "events";
import { HandleError } from "./helpers";

import { Empty } from "google-protobuf/google/protobuf/empty_pb";
// import google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb.js";

import {
  Status,
  Command,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_heaters_pb";

import { HeatersServiceClient } from "@caniot-controller/caniot-api-grpc-web/api/Ng_heatersServiceClientPb";

class HeatersStore extends EventEmitter {
  client: HeatersServiceClient;

  constructor() {
    super();
    this.client = new HeatersServiceClient("http://localhost:50051");
  }

  getStatus = (req: Empty, callbackFunc: (resp: Status) => void) => {
    this.client.getState(req, null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "HeatersStore::GetStatus succeeded",
        duration: 3,
      });

      callbackFunc(resp);
    });
  };

  setStatus = (req: Command, callbackFunc: (resp: Status) => void) => {
    this.client.setState(req, null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "HeatersStore::SetStatus succeeded",
        duration: 3,
      });

      callbackFunc(resp);
    });
  };
}

const heatersStore = new HeatersStore();
export default heatersStore;
