import {
  HelloResponse,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_internal_pb";
import { List } from "antd";
import { PropsWithChildren } from "react";
import ListLabelledItem from "./ListLabelledItem";

interface IHelloPropsComponent {
  resp: HelloResponse | undefined;
}

function Hello({ resp, children }: PropsWithChildren<IHelloPropsComponent>) {
  if (resp === undefined) {
    return undefined;
  }

  const datas = [
    ["Message", resp.getMessage()],
    ["Timestamp", resp.getTimestamp()?.toDate().toString()],
    ["Map", "count: " + Object.keys(resp.getMapMap()).length],
    ["Strings", resp.getStringsList().join(", ")],
    ["Bytes", resp.getBytes_asB64()],
  ];

  return (
    <List
      dataSource={datas}
      renderItem={(item) => <ListLabelledItem label={item[0]}>{item[1]}</ListLabelledItem>}
    >
      {children}
    </List>
  );
}

export default Hello;
