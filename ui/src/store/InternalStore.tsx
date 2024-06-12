import { notification } from "antd";
import EventEmitter from "events";
import { HandleError, HandleSuccess, getApiUrl } from "./helpers";
import {
  HelloRequest,
  HelloResponse,
  PartialSettings,
  Settings,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_internal_pb";

import { Empty } from "google-protobuf/google/protobuf/empty_pb";
import { InternalServiceClient } from "@caniot-controller/caniot-api-grpc-web/api/Ng_internalServiceClientPb";

class InternalStore extends EventEmitter {
  client: InternalServiceClient;

  constructor() {
    super();
    this.client = new InternalServiceClient(getApiUrl());
  }

  hello = (req: HelloRequest, callbackFunc: (resp: HelloResponse) => void) => {
    this.client.hello(req, null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("InternalStore::Hello succeeded");

      callbackFunc(resp);
    });
  };

  getSettings = (callbackFunc: (resp: Settings) => void) => {
    this.client.getSettings(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("InternalStore::GetSettings succeeded");

      callbackFunc(resp);
    });
  };

  setSettings = (req: PartialSettings, callbackFunc: (resp: Settings) => void) => {
    this.client.setSettings(req, null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("InternalStore::SetSettings succeeded");

      callbackFunc(resp);
    });
  };
}

const internalStore = new InternalStore();
export default internalStore;
