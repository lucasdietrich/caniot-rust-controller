import { Device } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import { Badge, Card, List, Space } from "antd";
import React, { useEffect, useState } from "react";
import ListLabelledItem from "./ListLabelledItem";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";

interface IProps {
  title: string;
  resp: Device | undefined;
}

const SECONDS_TO_CONSIDER_ONLINE = 60;

function DeviceStatusCard({ title, resp }: IProps) {
  if (resp === undefined) {
    return undefined;
  }

  let lastseen: Timestamp | undefined = resp.getLastseen();
  let lastseen_fmt = lastseen?.toDate().toLocaleString();

  // make sure getLastseen is less than 10 seconds
  const isOnline = resp.getLastseenfromnow() < SECONDS_TO_CONSIDER_ONLINE;

  let tempIn = "N/A",
    tempExt0 = "N/A",
    tempExt1 = "N/A",
    tempExt2 = "N/A",
    hasTempExt0 = false,
    hasTempExt1 = false,
    hasTempExt2 = false,
    inputs,
    ios_count = 0;

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

    // add a "Badge" for each input/output
    inputs = c1?.getIosList().map((io, index) => (
      <Space key={index} style={{ marginBottom: 8 }}>
        {" "}
        <Badge status={io ? "success" : "error"} />
      </Space>
    ));

    ios_count = c1?.getIosList().length || 0;
  }

  return (
    <Card title={<Badge status={isOnline ? "success" : "error"} text={title} />}>
      <List>
        <ListLabelledItem label="Date">
          {lastseen_fmt + " (actif il y a " + resp.getLastseenfromnow() + "s)"}
        </ListLabelledItem>
        <ListLabelledItem label="Temp carte">{tempIn} °C</ListLabelledItem>
        <ListLabelledItem label="Temp extérieure (sens 0)">{tempExt0} °C</ListLabelledItem>
        {hasTempExt1 && (
          <ListLabelledItem label="Temp extérieure (sens 1)">{tempExt1} °C</ListLabelledItem>
        )}
        {hasTempExt2 && (
          <ListLabelledItem label="Temp extérieure (sens 1)">{tempExt2} °C</ListLabelledItem>
        )}
        {resp?.hasClass0() && (
          <>
            <ListLabelledItem label={"Entrées (4)"}>ToImplement</ListLabelledItem>
            <ListLabelledItem label={"Sorties (4)"}>ToImplement</ListLabelledItem>
          </>
        )}
        {resp?.hasClass1() && (
          <>
            <ListLabelledItem label={"Entrées/Sorties  (" + ios_count + ")"}>
              {inputs}
            </ListLabelledItem>
          </>
        )}
        {/* <ListLabelledItem label="Statistiques">
          <List></List>
        </ListLabelledItem> */}
        <ListLabelledItem label="rx">{resp.getStats()?.getRx()}</ListLabelledItem>
        <ListLabelledItem label="tx">{resp.getStats()?.getTx()}</ListLabelledItem>
        <ListLabelledItem label="telemetry_tx">
          {resp.getStats()?.getTelemetryTx()}
        </ListLabelledItem>
        <ListLabelledItem label="telemetry_rx">
          {resp.getStats()?.getTelemetryRx()}
        </ListLabelledItem>
        <ListLabelledItem label="command_tx">{resp.getStats()?.getCommandTx()}</ListLabelledItem>
        <ListLabelledItem label="err">{resp.getStats()?.getErrRx()}</ListLabelledItem>
        <ListLabelledItem label="attribute_rx">
          {resp.getStats()?.getAttributeRx()}
        </ListLabelledItem>
        <ListLabelledItem label="attribute_tx">
          {resp.getStats()?.getAttributeTx()}
        </ListLabelledItem>
      </List>
    </Card>
  );
}

export default DeviceStatusCard;
