import { DevicesList } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";
import { Table, TableProps, Button, Space, Tag } from "antd";

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
        <span>
          {last_seen ? last_seen.timestamp.toDate().toLocaleString() : "never"} (
          {last_seen ? last_seen.secondsFromNow : ""} ago)
        </span>
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
          <Button type="primary" size="small">
            Reset
          </Button>
          <Button type="primary" size="small">
            Reset settings
          </Button>
          <Button type="primary" size="small">
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
    };
  });

  return <Table columns={columns} dataSource={data} />;
}

export default DevicesTable;
