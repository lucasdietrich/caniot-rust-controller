import { Button, Card, Space, Spin } from "antd";
import React, { useEffect, useState } from "react";
import DevicesTable from "../components/DevicesArray";
import { Empty } from "google-protobuf/google/protobuf/empty_pb";
import { DevicesList } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import devicesStore from "../store/DevicesStore";
import { LoadingOutlined, SyncOutlined } from "@ant-design/icons";

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
      <Card
        title={
          <>
            <span>
              Devices
              {devicesList?.getDevicesList().length
                ? " (" + devicesList?.getDevicesList().length + ")"
                : ""}
              <Spin spinning={loading} indicator={<LoadingOutlined spin />} />
            </span>

            <Button
              type="primary"
              icon={<SyncOutlined />}
              size="small"
              style={{
                position: "absolute",
                right: 20,
              }}
              onClick={() => {
                setLoading(true);
                setRefreshData(!refreshData);
              }}
            />
          </>
        }
        size="small"
      >
        <DevicesTable devicesList={devicesList} />
      </Card>
    </Space>
  );
}

export default DevicesView;
