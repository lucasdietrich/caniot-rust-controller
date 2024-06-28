import { notification } from "antd";
import EventEmitter from "events";
import { HandleError, HandleSuccess, getApiUrl } from "./helpers";
import {
  ControllerStats,
  FirmwareInfos,
  HelloRequest,
  HelloResponse,
  Infos,
  PartialSettings,
  Settings,
  SoftwareInfos,
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

  resetSettings = (callbackFunc: (resp: Settings) => void) => {
    this.client.resetSettings(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("InternalStore::ResetSettings succeeded");

      callbackFunc(resp);
    });
  };

  getSoftwareInfos = (callbackFunc: (resp: SoftwareInfos) => void) => {
    this.client.getSoftwareInfos(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("InternalStore::GetSoftwareInfos succeeded");

      callbackFunc(resp);
    });
  };

  getFirmwareInfos = (callbackFunc: (resp: FirmwareInfos) => void) => {
    this.client.getFirmwareInfos(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("InternalStore::GetFirmwareInfos succeeded");

      callbackFunc(resp);
    });
  };

  getControllerStats = (callbackFunc: (resp: ControllerStats) => void) => {
    this.client.getControllerStats(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("InternalStore::GetControllerStats succeeded");

      callbackFunc(resp);
    });
  };

  getInfos = (callbackFunc: (resp: Infos) => void) => {
    this.client.getInfos(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("InternalStore::GetInfos succeeded");

      callbackFunc(resp);
    });
  };
}

const internalStore = new InternalStore();
export default internalStore;
