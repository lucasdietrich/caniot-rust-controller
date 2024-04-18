import { notification } from "antd";
import EventEmitter from "events";
import { HandleError } from "./helpers";
import {
  HelloRequest,
  HelloResponse,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_internal_pb";

import { InternalServiceClient } from "@caniot-controller/caniot-api-grpc-web/api/Ng_internalServiceClientPb";

class InternalStore extends EventEmitter {
  client: InternalServiceClient;

  constructor() {
    super();
    this.client = new InternalServiceClient("http://localhost:50051");
  }

  hello = (req: HelloRequest, callbackFunc: (resp: HelloResponse) => void) => {
    this.client.hello(req, null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Hello succeeded",
        description: resp.getMessage(),
        duration: 3,
      });

      callbackFunc(resp);
    });
  };
}

const internalStore = new InternalStore();
export default internalStore;
