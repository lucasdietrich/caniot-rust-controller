import { notification } from "antd";
import EventEmitter from "events";
import { HandleError, HandleSuccess } from "./helpers";

import { Empty } from "google-protobuf/google/protobuf/empty_pb";
// import google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb.js";

import {
  Status,
  Command,
  CommandMessage,
  DoorState,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_garage_pb";

import { GarageServiceClient } from "@caniot-controller/caniot-api-grpc-web/api/Ng_garageServiceClientPb";

class GarageStore extends EventEmitter {
  client: GarageServiceClient;

  constructor() {
    super();
    this.client = new GarageServiceClient("http://localhost:50051"); // http://192.168.10.53:50051
  }

  getState = (req: Empty, callbackFunc: (resp: Status) => void) => {
    this.client.getState(req, null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("GarageStore::GetState succeeded");

      callbackFunc(resp);
    });
  };

  setState = (req: CommandMessage, callbackFunc: (resp: Status) => void) => {
    this.client.setState(req, null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("GarageStore::SetState succeeded");

      callbackFunc(resp);
    });
  };
}

const garageStore = new GarageStore();
export default garageStore;
