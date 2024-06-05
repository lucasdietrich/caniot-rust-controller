import { Device } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import { Badge, Card, List, Progress, Space, Table } from "antd";
import React, { useEffect, useState } from "react";
import ListLabelledItem from "./ListLabelledItem";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";
import LoadableCard from "./LoadableCard";
import LastSeenBadge from "./LastSeenBadge";

interface IDeviceStatusCardProps {
  title?: string;
  device: Device | undefined;
  progress?: number;
}

const SECONDS_TO_CONSIDER_ONLINE = 60;

function DeviceStatusCard({ title, device, progress }: IDeviceStatusCardProps) {
  if (device === undefined) {
    return undefined;
  }

  const isOnline = device.getLastseenfromnow() < SECONDS_TO_CONSIDER_ONLINE;

  return (
    <LoadableCard title={title} status={isOnline} loading={device === undefined}>
      <DeviceStatusCardContent device={device} />
    </LoadableCard>
  );
}

interface IDeviceCardContentProps {
  device: Device;
}

function DeviceStatusCardContent({ device: resp }: IDeviceCardContentProps) {
  let tempIn = "N/A",
    tempExt0 = "N/A",
    tempExt1 = "N/A",
    tempExt2 = "N/A",
    hasTempExt0 = false,
    hasTempExt1 = false,
    hasTempExt2 = false,
    inputs,
    ios_count = 0;

  let tempBoard = resp.hasBoardTemp() ? resp.getBoardTemp().toFixed(2) : "N/A";

  if (resp.hasClass0()) {
    const c0 = resp.getClass0();

    hasTempExt0 = c0?.hasExtTemp0() || false;
    hasTempExt1 = c0?.hasExtTemp1() || false;
    hasTempExt2 = c0?.hasExtTemp2() || false;

    tempIn = c0?.hasIntTemp() ? c0.getIntTemp().toFixed(2) : "N/A";
    tempExt0 = c0?.hasExtTemp0() ? c0.getExtTemp0()?.toFixed(2) : "N/A";
    tempExt1 = c0?.hasExtTemp1() ? c0.getExtTemp1()?.toFixed(2) : "N/A";
    tempExt2 = c0?.hasExtTemp2() ? c0.getExtTemp2()?.toFixed(2) : "N/A";
  } else if (resp.hasClass1()) {
    const c1 = resp.getClass1();

    hasTempExt0 = c1?.hasExtTemp0() || false;
    hasTempExt1 = c1?.hasExtTemp1() || false;
    hasTempExt2 = c1?.hasExtTemp2() || false;

    tempIn = c1?.hasIntTemp() ? c1.getIntTemp().toFixed(2) : "N/A";
    tempExt0 = c1?.hasExtTemp0() ? c1.getExtTemp0()?.toFixed(2) : "N/A";
    tempExt1 = c1?.hasExtTemp1() ? c1.getExtTemp1()?.toFixed(2) : "N/A";
    tempExt2 = c1?.hasExtTemp2() ? c1.getExtTemp2()?.toFixed(2) : "N/A";

    inputs = c1?.getIosList().map((io, index) => (
      <Space key={index} style={{ marginBottom: 8 }}>
        {" "}
        <Badge status={io ? "success" : "error"} />
      </Space>
    ));

    ios_count = c1?.getIosList().length || 0;
  }

  let stats = resp.getStats();
  const statsDataRoot: Map<string, number> = new Map<string, number>([
    ["rx", stats?.getRx() || 0],
    ["tx", stats?.getTx() || 0],
    ["telemetry_tx", stats?.getTelemetryTx() || 0],
    ["telemetry_rx", stats?.getTelemetryRx() || 0],
    ["command_tx", stats?.getCommandTx() || 0],
    ["err", stats?.getErrRx() || 0],
    ["attribute_rx", stats?.getAttributeRx() || 0],
    ["attribute_tx", stats?.getAttributeTx() || 0],
  ]);

  const statsData = Array.from(statsDataRoot).map(([metric, value]) => ({
    key: metric,
    metric,
    value,
  }));

  const statsColumns = [
    {
      title: "Metric",
      dataIndex: "metric",
      key: "metric",
    },
    {
      title: "Value",
      dataIndex: "value",
      key: "value",
    },
  ];

  return (
    <List size="small">
      <ListLabelledItem label="Status">
        <LastSeenBadge
          lastSeenDate={resp.getLastseen()?.toDate()}
          lastSeenValue={resp.getLastseenfromnow()}
          secondsToConsiderOnline={SECONDS_TO_CONSIDER_ONLINE}
        />
      </ListLabelledItem>
      <ListLabelledItem label="Contrôleur">
        {resp?.getControllerAttached() ? resp.getControllerName() : "N/A"}
      </ListLabelledItem>
      <ListLabelledItem label="Temp carte">{tempBoard} °C</ListLabelledItem>
      <ListLabelledItem label="Temp extérieure (sens 0)">{tempExt0} °C</ListLabelledItem>
      {hasTempExt1 && (
        <ListLabelledItem label="Temp extérieure (sens 1)">{tempExt1} °C</ListLabelledItem>
      )}
      {hasTempExt2 && (
        <ListLabelledItem label="Temp extérieure (sens 1)">{tempExt2} °C</ListLabelledItem>
      )}
      {resp?.hasClass0() && (
        <>
          <ListLabelledItem label={"Entrées (4)"}>To Implement</ListLabelledItem>
          <ListLabelledItem label={"Sorties (4)"}>To Implement</ListLabelledItem>
        </>
      )}
      {resp?.hasClass1() && (
        <>
          <ListLabelledItem label={"Entrées/Sorties  (" + ios_count + ")"}>
            {inputs}
          </ListLabelledItem>
        </>
      )}
      <ListLabelledItem label="Statistiques" labelAlignTop={true}>
        <Table
          dataSource={statsData}
          columns={statsColumns}
          showHeader={false}
          pagination={false}
          size="small"
          style={{ maxWidth: 300 }}
        />
      </ListLabelledItem>
    </List>
  );
}

export default DeviceStatusCard;
export { DeviceStatusCardContent };
