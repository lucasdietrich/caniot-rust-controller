import {
  HelloRequest,
  HelloResponse,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_pb";
import internalStore from "../store/InternalStore";
import { Card, Divider, List, Typography, Button } from "antd";
import { useEffect, useState } from "react";
import { ReloadOutlined } from "@ant-design/icons";
import Hello from "../components/Hello";

interface IProps {
  user_name: string;
}

function HelloCard({ user_name }: IProps) {
  const [resp, setHelloResp] = useState<HelloResponse | undefined>(undefined);
  const [refreshData, setRefreshData] = useState<boolean>(false);

  useEffect(() => {
    const req = new HelloRequest();
    req.setName(user_name);

    internalStore.hello(req, (resp: HelloResponse) => {
      setHelloResp(resp);
    });
  }, [refreshData]);

  return (
    <Card title={"Hello " + user_name} bordered={false}>
      <Hello resp={resp}>
        {" "}
        <Button
          type="primary"
          icon={<ReloadOutlined />}
          onClick={() => setRefreshData(!refreshData)}
        >
          Refresh
        </Button>
      </Hello>
    </Card>
  );
}

export default HelloCard;
