import { Card, List, Typography } from "antd";
import React from "react";
import ListLabelledItem from "./ListLabelledItem";
import { ControllerStats } from "@caniot-controller/caniot-api-grpc-web/api/ng_internal_pb";
import LoadableCard from "./LoadableCard";

interface ControllerStatsSardProps {
  stats?: ControllerStats;
}

function ControllerStatsCard({ stats }: ControllerStatsSardProps) {
  return (
    <LoadableCard loading={!stats} title="Statistiques du contrôleur" bordered={false}>
      <List>
        <ListLabelledItem label="CAN Interface RX">{stats?.getIfaceRx() ?? "N/A"}</ListLabelledItem>
        <ListLabelledItem label="CAN Interface TX">{stats?.getIfaceTx() ?? "N/A"}</ListLabelledItem>
        <ListLabelledItem label="CAN Interface Errors">
          {stats?.getIfaceErr() ?? "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="CAN Interface Malformed">
          {stats?.getIfaceMalformed() ?? "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="Pending queries pushed">
          {stats?.getPqPushed() ?? "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="Pending queries answered">
          {stats?.getPqAnswered() ?? "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="Pending queries timeout">
          {stats?.getPqTimeout() ?? "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="Duplicate pending queries dropped">
          {stats?.getPqDuplicateDropped() ?? "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="Internal API calls">{stats?.getApiRx() ?? "N/A"}</ListLabelledItem>
        <ListLabelledItem label="Loop runs count">{stats?.getLoopRuns() ?? "N/A"}</ListLabelledItem>
      </List>
    </LoadableCard>
  );
}

export default ControllerStatsCard;
