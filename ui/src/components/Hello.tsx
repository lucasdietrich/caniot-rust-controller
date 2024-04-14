import internalStore from '../store/InternalStore'
import { HelloRequest, HelloResponse } from '@caniot-controller/caniot-api-grpc-web/api2/ng_pb';


function Hello() {
  let req = new HelloRequest();
  req.setName("Lucas");

  console.log("Hello component");

  internalStore.hello(req, (resp: HelloResponse) => {
    console.log(resp.getMessage());
  });

  return (
    <div>Hello sqd</div>
  )
}

export default Hello