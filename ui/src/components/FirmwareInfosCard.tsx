import { Card, List, Typography } from "antd";
import React from "react";
import ListLabelledItem from "./ListLabelledItem";
import { FirmwareInfos } from "@caniot-controller/caniot-api-grpc-web/api/ng_internal_pb";
import LoadableCard from "./LoadableCard";

interface FirmwareInfosCardProps {
  infos?: FirmwareInfos;
}

function FirmwareInfosCard({ infos }: FirmwareInfosCardProps) {
  return (
    <LoadableCard loading={!infos} title="Firmware" bordered={false}>
      <List>
        <ListLabelledItem label="Distribution">
          {infos?.getBuild()?.getDistro() || "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="Version de distribution">
          {infos?.getBuild()?.getDistroVersion() || "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="Date de build">
          {infos?.getBuild()?.getBuildDate()?.toDate().toLocaleString() || "N/A"}
        </ListLabelledItem>
      </List>
    </LoadableCard>
  );
}

export default FirmwareInfosCard;
