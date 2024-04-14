import { notification } from 'antd';
import EventEmitter from 'events'
import { HandleError } from './helpers';
import { HelloRequest, HelloResponse,  } from '@caniot-controller/caniot-api-grpc-web/api/ng_pb';

import { CaniotControllerClient } from '@caniot-controller/caniot-api-grpc-web/api/NgServiceClientPb';


class InternalStore extends EventEmitter {
  client: CaniotControllerClient;

  constructor() {
    super();
    this.client = new CaniotControllerClient("http://localhost:50051");
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