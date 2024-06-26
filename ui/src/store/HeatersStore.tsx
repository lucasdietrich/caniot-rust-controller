import { notification } from "antd";
import EventEmitter from "events";
import { HandleError, HandleSuccess, getApiUrl } from "./helpers";

import { Empty } from "google-protobuf/google/protobuf/empty_pb";
// import google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb.js";

import { Status, Command } from "@caniot-controller/caniot-api-grpc-web/api/ng_heaters_pb";

import { HeatersServiceClient } from "@caniot-controller/caniot-api-grpc-web/api/Ng_heatersServiceClientPb";

class HeatersStore extends EventEmitter {
  client: HeatersServiceClient;

  constructor() {
    super();
    this.client = new HeatersServiceClient(getApiUrl());
  }

  getStatus = (callbackFunc: (resp: Status) => void) => {
    this.client.getState(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("HeatersStore::GetStatus succeeded");

      callbackFunc(resp);
    });
  };

  setStatus = (req: Command, callbackFunc: (resp: Status) => void) => {
    this.client.setState(req, null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("HeatersStore::SetStatus succeeded");

      callbackFunc(resp);
    });
  };
}

const heatersStore = new HeatersStore();
export default heatersStore;
