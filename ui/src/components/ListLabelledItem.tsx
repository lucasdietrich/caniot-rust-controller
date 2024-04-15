import { List, Typography } from "antd";
import React, { PropsWithChildren, ReactNode } from "react";

interface IProps {
  label: string;
}

function ListLabelledItem({ label, children }: PropsWithChildren<IProps>) {
  return (
    <List.Item>
      <Typography.Text
        strong
        style={{
          width: "175px", // 175px / 40%
          display: "block",
          float: "left",
        }}
      >
        {label}
      </Typography.Text>{" "}
      {children}
    </List.Item>
  );
}

export default ListLabelledItem;
