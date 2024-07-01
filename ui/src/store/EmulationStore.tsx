import { notification } from "antd";
import EventEmitter from "events";
import { HandleError, HandleSuccess, getApiUrl } from "./helpers";

import { Empty } from "google-protobuf/google/protobuf/empty_pb";
import { EmulationServiceClient } from "@caniot-controller/caniot-api-grpc-web/api/Ng_emulationServiceClientPb";
import { Req, Status } from "@caniot-controller/caniot-api-grpc-web/api/ng_emulation_pb";

class EmulationStore extends EventEmitter {
  client: EmulationServiceClient;

  constructor() {
    super();
    this.client = new EmulationServiceClient(getApiUrl());
  }

  get = (callbackFunc: (resp: Status) => void) => {
    this.client.get(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("EmulationStore::Get succeeded");

      callbackFunc(resp);
    });
  };

  set = (req: Req, callbackFunc: (resp: Status) => void) => {
    this.client.set(req, null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("EmulationStore::Set succeeded");

      callbackFunc(resp);
    });
  };
}

const emulationStore = new EmulationStore();
export default emulationStore;
