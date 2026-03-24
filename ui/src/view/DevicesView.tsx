import { Space } from "antd";
import React, { useEffect, useState } from "react";
import DevicesTable from "../components/DevicesTable";
import { DevicesList } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import devicesStore from "../store/DevicesStore";
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
