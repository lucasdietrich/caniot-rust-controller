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

import { CaniotDevicesServiceClient } from "@caniot-controller/caniot-api-grpc-web/api/Ng_devicesServiceClientPb";
import { DeviceId } from "@caniot-controller/caniot-api-grpc-web/api/common_pb";

class DevicesStore extends EventEmitter {
  client: CaniotDevicesServiceClient;

  constructor() {
    super();
    this.client = new CaniotDevicesServiceClient(getApiUrl());
  }

  getList = (callbackFunc: (resp: DevicesList) => void) => {
    this.client.getList(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("DevicesStore::GetList succeeded");

      callbackFunc(resp);
    });
  };

  get = (req: DeviceId, callbackFunc: (resp: Device) => void) => {
    this.client.get(req, null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("DevicesStore::Get succeeded");

      callbackFunc(resp);
    });
  };

  getHeatersDevice = (callbackFunc: (resp: Device) => void) => {
    this.client.getHeatersDevice(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("DevicesStore::GetHeatersDevice succeeded");

      callbackFunc(resp);
    });
  };

  getGarageDevice = (callbackFunc: (resp: Device) => void) => {
    this.client.getGarageDevice(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("DevicesStore::GetGarageDevice succeeded");

      callbackFunc(resp);
    });
  };

  getOutdoorAlarmDevice = (callbackFunc: (resp: Device) => void) => {
    this.client.getOutdoorAlarmDevice(new Empty(), null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("DevicesStore::GetOutdoorAlarmDevice succeeded");

      callbackFunc(resp);
    });
  };

  performAction = (action: Action, callbackFunc: (resp: ActionResult) => void) => {
    this.client.performAction(action, null, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      HandleSuccess("DevicesStore::PerformAction succeeded");

      callbackFunc(resp);
    });
  };
}

const devicesStore = new DevicesStore();
export default devicesStore;
