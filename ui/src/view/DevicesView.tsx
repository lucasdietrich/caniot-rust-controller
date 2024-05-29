import { Button, Card, Space, Spin } from "antd";
import React, { useEffect, useState } from "react";
import DevicesTable from "../components/DevicesArray";
import { Empty } from "google-protobuf/google/protobuf/empty_pb";
import { DevicesList } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import devicesStore from "../store/DevicesStore";
import { LoadingOutlined, SyncOutlined } from "@ant-design/icons";
import LoadableCard from "../components/LoadableCard";

function DevicesView() {
  const [devicesList, setDevicesList] = useState<DevicesList | undefined>(undefined);
  const [refreshData, setRefreshData] = useState<boolean>(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setLoading(true);
    devicesStore.getList((resp: DevicesList) => {
      setDevicesList(resp);
      setLoading(false);
    });
  }, [refreshData]);

  return (
    <Space direction="vertical" size="middle" style={{ display: "flex" }}>
      <LoadableCard
        title={
          <span>
            Devices
            {devicesList?.getDevicesList().length
              ? " (" + devicesList?.getDevicesList().length + ")"
              : ""}
          </span>
        }
        loading={loading}
        onRefresh={() => {
          setLoading(true);
          setRefreshData(!refreshData);
        }}
      >
        <DevicesTable devicesList={devicesList} />
      </LoadableCard>
    </Space>
  );
}

export default DevicesView;
