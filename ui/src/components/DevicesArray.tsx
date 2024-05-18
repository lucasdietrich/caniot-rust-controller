import { Device, DevicesList } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";
import { Table, TableProps, Button, Space, Tag, Badge } from "antd";
import devicesStore from "../store/DevicesStore";
import { DeviceId } from "@caniot-controller/caniot-api-grpc-web/api/common_pb";
import { useState } from "react";
import DeviceStatusCard from "./DeviceStatusCard";
import { LoadingOutlined } from "@ant-design/icons";

const SECONDS_TO_CONSIDER_ONLINE = 60;

interface IProps {
  devicesList: DevicesList | undefined;
}

interface DeviceRow {
  key: string;
  did: number;
  tags: {
    sid: number;
    cls: number;
  };
  controller?: string;
  last_seen?: {
    timestamp: Timestamp;
    secondsFromNow: number;
  };
  temp_in?: number;
  device?: Device;
}

const classColor: { [key: number]: string } = {
  0: "red",
  1: "green",
  2: "purple",
  3: "orange",
  4: "cyan",
  5: "magenta",
  6: "geekblue",
  7: "orange",
};

function DevicesTable({ devicesList }: IProps) {
  const [nestedDevices, setNestedDevices] = useState<{ [key: number]: Device }>({});
  const [isLoading, setIsLoading] = useState<{ [key: number]: boolean }>({});

  if (devicesList === undefined) {
    return undefined;
  }

  const columns: TableProps<DeviceRow>["columns"] = [
    {
      title: "Device Id",
      dataIndex: "did",
      key: "did",
      render: (did) => <span>{did}</span>,
      sorter: (a, b) => a.did - b.did,
    },
    {
      title: "Tags",
      dataIndex: "tags",
      key: "tags",
      render: ({ cls, sid }, record) => (
        <span>
          <Tag color={classColor[cls]}>Class {cls}</Tag>
          <Tag color="blue">id {sid}</Tag>
        </span>
      ),
    },
    {
      title: "Controller",
      dataIndex: "controller",
      key: "controller",
      render: (controller) => <span>{controller || "N/A"}</span>,
    },
    {
      title: "Last seen",
      dataIndex: "last_seen",
      key: "last_seen",
      render: (last_seen) => (
        <Badge
          status={
            last_seen && last_seen.secondsFromNow < SECONDS_TO_CONSIDER_ONLINE ? "success" : "error"
          }
          text={
            <span>
              {last_seen ? last_seen.timestamp.toDate().toLocaleString() : "never"} (
              {last_seen ? last_seen.secondsFromNow : ""} ago)
            </span>
          }
        />
      ),
      sorter: (a, b) => {
        if (a.last_seen && b.last_seen) {
          return a.last_seen.secondsFromNow - b.last_seen.secondsFromNow;
        } else if (a.last_seen) {
          return -1;
        } else if (b.last_seen) {
          return 1;
        } else {
          return 0;
        }
      },
    },
    {
      title: "Temp In",
      dataIndex: "temp_in",
      key: "temp_in",
      render: (temp_in) => <span>{temp_in?.toFixed(2) || "N/A"} °C</span>,
    },
    {
      title: "Action",
      key: "action",
      render: (_, record) => (
        <Space size="small">
          <Button size="small" type="dashed">
            Ping
          </Button>
          <Button type="default" size="small">
            Reboot
          </Button>
          <Button size="small" type="primary">
            Reset settings
          </Button>
        </Space>
      ),
    },
  ];

  const data = devicesList.getDevicesList().map((device, index) => {
    return {
      key: index.toString(),
      did: device.getDid()?.getDid() || 0,
      tags: {
        cls: device.getDid()?.getCls() || 0,
        sid: device.getDid()?.getSid() || 0,
      },
      controller: device.getControllerAttached() ? device.getControllerName() : undefined,
      last_seen: device.getLastseen()
        ? {
            timestamp: device.getLastseen() || new Timestamp(),
            secondsFromNow: device.getLastseenfromnow(),
          }
        : undefined,
      temp_in: device.getClass1()?.getIntTemp(),
      device: device,
    };
  });

  // const onExpand = (_expanded: any, record: DeviceRow) => {
  //   setIsLoading({ ...isLoading, [record.did]: true });
  //   let did = new DeviceId();
  //   did.setDid(record.did);
  //   devicesStore.get(did, (resp) => {
  //     setNestedDevices({ ...nestedDevices, [record.did]: resp });
  //     setIsLoading({ [record.did]: false });
  //     console.log(resp.getLastseenfromnow());
  //   });
  // };

  const expandedRowRender = (record: DeviceRow) => {
    if (record.device !== undefined) {
      return <DeviceStatusCard device={record.device} />;
    } else {
      return <LoadingOutlined />;
    }
  };

  return (
    <Table
      columns={columns}
      dataSource={data}
      expandable={{
        expandedRowRender: expandedRowRender,
        rowExpandable: (record) => true,
        // onExpand: onExpand,
      }}
    />
  );
}

export default DevicesTable;
