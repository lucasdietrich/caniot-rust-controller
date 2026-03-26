import EventEmitter from "events";
import { HandleError, HandleSuccess, getApiUrl } from "./helpers";

import { Empty } from "google-protobuf/google/protobuf/empty_pb";
// import google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb.js";

import { CoproServiceClient } from "@caniot-controller/caniot-api-grpc-web/api/Ng_coproServiceClientPb";
import {
  BleDevicesList,
  CoproAlert,
  CoproDevice,
  CoproDevicesList,
  GetListParams,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_copro_pb";

class CoproStore extends EventEmitter {
  client: CoproServiceClient;

  constructor() {
    super();
    this.client = new CoproServiceClient(getApiUrl());
  }

  getDeviceByName = (name: string, callbackFunc: (resp: CoproDevice) => void) => {
    let params = new GetListParams();
    params.setName(name);
    this.client.getList(params, null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("CoproStore::GetDevice succeeded");

      if (resp.getDevicesList().length === 1) {
        callbackFunc(resp.getDevicesList()[0]);
      } else {
        // notification.error({
        //   message: "CoproStore::GetDevice",
        //   description: `Expected 1 device, got ${resp.getDevicesList().length}`,
        // });
        console.error(`CoproStore::GetDevice: Expected 1 device, got ${resp.getDevicesList().length}`);
      }
    });
  };

  getList = (callbackFunc: (resp: CoproDevicesList) => void) => {
    let params = new GetListParams();
    params.setAll(new Empty());
    this.client.getList(params, null, (err, resp) => {
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

  getBleDevices = (callbackFunc: (resp: BleDevicesList) => void) => {
    this.client.getBleDevices(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }
      callbackFunc(resp);
    });
  };

  bleRemoveBonds = (callbackFunc: () => void) => {
    this.client.bleRemoveBonds(new Empty(), null, (err, _resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }
      HandleSuccess("CoproStore::BleRemoveBonds succeeded");
      callbackFunc();
    });
  };
}

const coproStore = new CoproStore();
export default coproStore;
