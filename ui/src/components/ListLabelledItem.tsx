import { List, Typography } from "antd";
import React, { PropsWithChildren, ReactNode } from "react";

interface IListLabelledItemProps {
  label?: string;
  labelAlignTop?: boolean;
}

function ListLabelledItem({
  label,
  labelAlignTop = false,
  children,
}: PropsWithChildren<IListLabelledItemProps>) {
  return (
    <List.Item style={{ display: "flex", alignItems: labelAlignTop ? "flex-start" : "center" }}>
      {label && (
        <div style={{ width: 175, flexShrink: 0 }}>
          <Typography.Text strong>{label}</Typography.Text>{" "}
        </div>
      )}
      <div style={{ flexGrow: 1 }}>{children}</div>
    </List.Item>
  );
}

export default ListLabelledItem;
