// @ts-check

import { DevicesList } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";
import { Table } from "antd";

interface IProps {
  devicesList: DevicesList | undefined;
}

function DevicesTable({ devicesList }: IProps) {
  if (devicesList === undefined) {
    return undefined;
  }

  const columns = [
    {
      title: "Device ID",
      dataIndex: "did",
      key: "did",
    },
    {
      title: "Last seen",
      dataIndex: "last_seen",
      key: "last_seen",
    },
    {
      title: "Temperature",
      dataIndex: "temp_in",
      key: "temp_in",
    },
  ];

  const dataSource = devicesList.getDevicesList().map((device, index) => {
    let lastseen: Timestamp | undefined = device.getLastseen();
    let lastseen_fmt = lastseen?.toDate().toLocaleString();

    let temp = device.getClass1()?.getIntTemp().toFixed(2) || "N/A";

    return {
      key: index,
      did: device.getDid()?.getCls() + " / " + device.getDid()?.getSid(),
      last_seen: lastseen_fmt || "",
      temp_in: temp + " °C",
    };
  });

  return <Table dataSource={dataSource} columns={columns} />;
}

export default DevicesTable;
