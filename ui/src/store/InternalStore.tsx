import { notification } from "antd";
import EventEmitter from "events";
import { HandleError, HandleSuccess } from "./helpers";
import {
  HelloRequest,
  HelloResponse,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_internal_pb";

import { InternalServiceClient } from "@caniot-controller/caniot-api-grpc-web/api/Ng_internalServiceClientPb";

class InternalStore extends EventEmitter {
  client: InternalServiceClient;

  constructor() {
    super();
    this.client = new InternalServiceClient("http://localhost:50051"); // http://192.168.10.53:50051
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
}

const internalStore = new InternalStore();
export default internalStore;
