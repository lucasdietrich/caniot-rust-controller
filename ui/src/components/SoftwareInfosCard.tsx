import { Card, List, Typography } from "antd";
import React from "react";
import ListLabelledItem from "./ListLabelledItem";
import { SoftwareInfos } from "@caniot-controller/caniot-api-grpc-web/api/ng_internal_pb";
import LoadableCard from "./LoadableCard";

interface SoftwareInfosCardProps {
  infos?: SoftwareInfos;
}

function SoftwareInfosCard({ infos }: SoftwareInfosCardProps) {
  return (
    <LoadableCard loading={!infos} title="Logiciel" bordered={false}>
      <List>
        <ListLabelledItem label="Status">
          <Typography.Text type="success">Running</Typography.Text>
        </ListLabelledItem>
        <ListLabelledItem label="Version">
          {infos?.getBuild()?.getVersion() || "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="Build date">
          {infos?.getBuild()?.getBuildDate()?.toDate().toLocaleString() || "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="Build commit">
          {infos?.getBuild()?.getCommit() || "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="Dernier lancement">
          {infos?.getStartDate()?.toDate().toLocaleString() || "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="Derrière mise à jour">
          {infos?.getUpdateDate()?.toDate().toLocaleString() || "Jamais"}
        </ListLabelledItem>
      </List>
    </LoadableCard>
  );
}

export default SoftwareInfosCard;
