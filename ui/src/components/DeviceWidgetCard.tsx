import { Card } from "antd";
import React, { PropsWithChildren } from "react";
import GarageDoorsView from "../view/GarageDoorsView";
import GarageDoorsStatus from "./GarageDoorsStatus";
import LoadableCard from "./LoadableCard";

interface IDeviceWidgetCardProps {
  title?: string;
}

function DeviceWidgetCard({
  title = undefined,
  children,
}: PropsWithChildren<IDeviceWidgetCardProps>) {
  return (
    <LoadableCard title={title} onGoto={() => {}} progress={undefined} loading={false}>
      {children}
    </LoadableCard>
  );
}

export default DeviceWidgetCard;
