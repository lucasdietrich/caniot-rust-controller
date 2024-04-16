import { DevicesList } from "@caniot-controller/caniot-api-grpc-web/api/ng_pb";
import { Table } from "antd";
import React from "react";

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
  ];

  const dataSource = devicesList.getDevicesList().map((device, index) => {
    return {
      key: index,
      did: device.getDid()?.getCls() + " / " + device.getDid()?.getSid(),
      last_seen: device.getLastseen()?.toDate().toString() || "",
    };
  });

  return <Table dataSource={dataSource} columns={columns} />;
}

export default DevicesTable;
