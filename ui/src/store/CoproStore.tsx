import { notification } from "antd";
import EventEmitter from "events";
import { HandleError, HandleSuccess, getApiUrl } from "./helpers";

import { Empty } from "google-protobuf/google/protobuf/empty_pb";
// import google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb.js";

import {
  Action,
  ActionResult,
  Device,
  DevicesList,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";

import { CoproServiceClient } from "@caniot-controller/caniot-api-grpc-web/api/Ng_coproServiceClientPb";
import { DeviceId } from "@caniot-controller/caniot-api-grpc-web/api/common_pb";
import {
  CoproAlert,
  CoproDevicesList,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_copro_pb";

class CoproStore extends EventEmitter {
  client: CoproServiceClient;

  constructor() {
    super();
    this.client = new CoproServiceClient(getApiUrl());
  }

  getList = (callbackFunc: (resp: CoproDevicesList) => void) => {
    this.client.getList(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("CoproStore::GetList succeeded");

      callbackFunc(resp);
    });
  };

  getCoproAlert = (callbackFunc: (resp: CoproAlert) => void) => {
    this.client.getCoproAlert(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("CoproStore::GetCoproAlert succeeded");

      callbackFunc(resp);
    });
  };
}

const coproStore = new CoproStore();
export default coproStore;
