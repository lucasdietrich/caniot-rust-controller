import { notification } from "antd";
import EventEmitter from "events";
import { HandleError, HandleSuccess, getApiUrl } from "./helpers";

import { Empty } from "google-protobuf/google/protobuf/empty_pb";
import { AlarmsServiceClient } from "@caniot-controller/caniot-api-grpc-web/api/Ng_alarmsServiceClientPb";
import {
  OutdoorAlarmCommand,
  OutdoorAlarmState,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_alarms_pb";
import { RpcError } from "grpc-web";

class AlarmsStore extends EventEmitter {
  client: AlarmsServiceClient;

  constructor() {
    super();
    this.client = new AlarmsServiceClient(getApiUrl());
  }

  getOutdoorAlarmState = (callbackFunc: (resp: OutdoorAlarmState) => void) => {
    this.client.getOutdoorAlarmState(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("AlarmsStore::GetOutdoorAlarmState succeeded");

      callbackFunc(resp);
    });
  };

  sendOutdoorAlarmCommand = (
    req: OutdoorAlarmCommand,
    callbackFunc: (resp: OutdoorAlarmState) => void,
    errCallbackFunc: (err: RpcError) => void
  ) => {
    this.client.sendOutdoorAlarmCommand(req, null, (err, resp) => {
      if (err !== null) {
        errCallbackFunc(err);
        HandleError(err);
        return;
      }

      HandleSuccess("AlarmsStore::SendOutdoorAlarmCommand succeeded");

      callbackFunc(resp);
    });
  };
}

const alarmsStore = new AlarmsStore();
export default alarmsStore;
