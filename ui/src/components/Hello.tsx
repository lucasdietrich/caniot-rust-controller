import { HelloRequest, HelloResponse } from '@caniot-controller/caniot-api-grpc-web/api/ng_pb'
import internalStore from '../store/InternalStore';

function Hello() {
  const req = new HelloRequest();
  req.setName("Lucas");

  console.log("Hello component");

  internalStore.hello(req, (resp: HelloResponse) => {
    console.log(resp.getMessage(), resp.getTimestamp()?.toDate());
  });

  return (
    <div>Hello sqd</div>
  )
}

export default Hello