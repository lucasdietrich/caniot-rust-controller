import { Card, Space } from "antd";
import React, { useEffect, useState } from "react";
import DevicesTable from "../components/DevicesArray";
import { Empty } from "google-protobuf/google/protobuf/empty_pb";
import { DevicesList } from "@caniot-controller/caniot-api-grpc-web/api/ng_pb";
import devicesStore from "../store/DevicesStore";

function DevicesView() {
  const [devicesList, setDevicesList] = useState<DevicesList | undefined>(
    undefined
  );
  const [refreshData, setRefreshData] = useState<boolean>(false);

  useEffect(() => {
    devicesStore.getList(new Empty(), (resp: DevicesList) => {
      setDevicesList(resp);
    });
  }, [refreshData]);

  return (
    <Space direction="vertical" size="middle" style={{ display: "flex" }}>
      <Card
        title={"Devices (" + devicesList?.getDevicesList().length + ")"}
        size="small"
      >
        <DevicesTable devicesList={devicesList} />
      </Card>
    </Space>
  );
}

export default DevicesView;
