import { Card, List, Typography } from "antd";
import React from "react";
import ListLabelledItem from "./ListLabelledItem";
import { SoftwareInfos } from "@caniot-controller/caniot-api-grpc-web/api/ng_internal_pb";
import LoadableCard from "./LoadableCard";

interface SoftwareInfosCardProps {
  infos?: SoftwareInfos;
  isMobile?: boolean;
}

function SoftwareInfosCard({ infos, isMobile = false }: SoftwareInfosCardProps) {
  return (
    <LoadableCard
      loading={infos === undefined}
      title="Logiciel"
      bordered={false}
      isMobile={isMobile}
    >
      <List>
        <ListLabelledItem label="Status">
          <Typography.Text type="success">Running</Typography.Text>
        </ListLabelledItem>
        <ListLabelledItem label="Version">
          {infos?.getBuild()?.getVersion() ?? "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="Build date">
          {infos?.getBuild()?.getBuildDate()?.toDate().toLocaleString() ?? "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="Build commit">
          {infos?.getBuild()?.getCommit() ?? "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="Derrière mise à jour">
          {infos?.getUpdateDate()?.toDate().toLocaleString() ?? "Jamais"}
        </ListLabelledItem>
        <ListLabelledItem label="Dernier lancement">
          {infos?.getRuntime()?.getStartTime()?.toDate().toLocaleString() ?? "N/A"}
        </ListLabelledItem>
        <ListLabelledItem label="Heure système">
          {infos?.getRuntime()?.getSystemTime()?.toDate().toLocaleString() ?? "N/A"}
        </ListLabelledItem>
      </List>
    </LoadableCard>
  );
}

export default SoftwareInfosCard;
