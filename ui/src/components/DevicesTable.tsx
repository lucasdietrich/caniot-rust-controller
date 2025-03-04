import {
  Action,
  Device,
  DevicesList,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";
import { Table, TableProps, Button, Space, Tag, Badge } from "antd";
import devicesStore from "../store/DevicesStore";
import { DeviceId, Endpoint } from "@caniot-controller/caniot-api-grpc-web/api/common_pb";
import { Empty as ProtobufEmpty } from "google-protobuf/google/protobuf/empty_pb";
import { useState } from "react";
import DeviceDetailsCard from "./DeviceDetailsCard";
import { LoadingOutlined } from "@ant-design/icons";
import LastSeenBadge from "./LastSeenBadge";
import { SECONDS_TO_CONSIDER_ONLINE_CANIOT } from "../constants";

interface IDevicesTableProps {
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

function DevicesTable({ devicesList }: IDevicesTableProps) {
  const [nestedDevices, setNestedDevices] = useState<{ [key: number]: Device }>({});
  const [isLoading, setIsLoading] = useState<{ [key: number]: boolean }>({});

  if (devicesList === undefined) {
    return undefined;
  }

  const handlePing = (did: DeviceId | undefined) => {
    return () => {
      let action = new Action();
      action.setDid(did);
      action.setPing(Endpoint.ENDPOINTBOARDLEVELCONTROL);
      devicesStore.performAction(action, (resp) => {
        console.log("[" + resp.getPong()?.getDid() + "] : " + resp.getPong()?.getPayloadList());
      });
    };
  };

  const handleReboot = (did: DeviceId | undefined) => {
    return () => {
      let action = new Action();
      action.setDid(did);
      action.setReboot(new ProtobufEmpty());
      devicesStore.performAction(action, (resp) => {
        console.log(resp);
      });
    };
  };

  const handleResetSettings = (did: DeviceId | undefined) => {
    return () => {
      let action = new Action();
      action.setDid(did);
      action.setResetSettings(new ProtobufEmpty());
      devicesStore.performAction(action, (resp) => {
        console.log(resp);
      });
    };
  };

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
      render: (controller) => <span>{controller ?? "N/A"}</span>,
    },
    {
      title: "Last seen",
      dataIndex: "last_seen",
      key: "last_seen",
      render: (last_seen) => (
        <LastSeenBadge
          lastSeenDate={last_seen?.timestamp.toDate()}
          lastSeenValue={last_seen.secondsFromNow}
          secondsToConsiderOnline={SECONDS_TO_CONSIDER_ONLINE_CANIOT}
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
      render: (temp_in) => <span>{temp_in?.toFixed(2) ?? "N/A"} °C</span>,
    },
    {
      title: "Action",
      key: "action",
      render: (_, record) => (
        <Space size="small">
          <Button
            size="small"
            type="dashed"
            disabled={record.device?.getDid()?.getObj() === undefined}
            onClick={handlePing(record.device?.getDid()?.getObj())}
          >
            Ping
          </Button>
          <Button
            size="small"
            type="primary"
            disabled={record.device?.getDid()?.getObj() === undefined}
            onClick={handleReboot(record.device?.getDid()?.getObj())}
          >
            Reboot
          </Button>
          <Button
            size="small"
            type="primary"
            danger
            disabled={record.device?.getDid()?.getObj() === undefined}
            onClick={handleResetSettings(record.device?.getDid()?.getObj())}
          >
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
      temp_in: device.hasBoardTemp() ? device.getBoardTemp() : undefined,
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
      return <DeviceDetailsCard device={record.device} />;
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
