import LoadableCard from "./LoadableCard";
import { Device } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import { useNavigate } from "react-router-dom";
import { Col, Divider, Row, Tooltip } from "antd";
import TemperatureGaugeStatistic, {
  BatteryGaugeText,
  HumidityGaugeStatistic,
  HumidityGaugeText,
  BleStatisticsText,
  TemperatureGaugeText,
} from "./Gauges";
import LastSeenBadge from "./LastSeenBadge";
import { TbCpu } from "react-icons/tb";
import { CoproDevice } from "@caniot-controller/caniot-api-grpc-web/api/ng_copro_pb";
import { SECONDS_TO_CONSIDER_ONLINE_BLE } from "../constants";

interface BleDeviceMetricsWidgetProps {
  title?: string;
  device?: CoproDevice;
  loading: boolean;
  navigateTo?: string;
  small?: boolean;
  debug?: boolean;
}

function BleDeviceMetricsWidget({
  title,
  device,
  loading,
  navigateTo,
  small = false,
  debug = false,
}: BleDeviceMetricsWidgetProps) {
  const navigate = useNavigate();

  let width_edges;
  let width_center;

  // (span_edge, span_center) = (12, 6) is a tuple
  if (debug) {
    width_edges = 6;
    width_center = 12;
  } else {
    width_edges = 8;
    width_center = 8;
  }

  return (
    <LoadableCard
      title={title}
      extraLabel={
        <Tooltip title={device?.getType() + " " + device?.getMac()}>
          <span style={{ color: "#777777", fontStyle: "italic" }}>
            {device?.getType() + " " + device?.getMac().trimStart().slice(9)}
          </span>
        </Tooltip>
      }
      onGoto={navigateTo ? () => navigate(navigateTo) : undefined}
      loading={loading}
      status={device !== undefined}
      bordered={false}
      cardStyle={{
        opacity: (device?.getLastseenfromnow() ?? 0) > SECONDS_TO_CONSIDER_ONLINE_BLE ? 0.5 : 1,
      }}
      isMobile={small}
    >
      <Row gutter={2}>
        <Col span={12}>
          <TemperatureGaugeStatistic
            title="Température"
            temperature={device?.hasTemperature() ? device.getTemperature() : undefined}
          />
        </Col>

        <Col span={12}>
          <HumidityGaugeStatistic
            title="Humidity"
            humidity={device?.hasHumidity() ? device.getHumidity() : undefined}
          />
        </Col>

        <Divider style={{ margin: 5 }} />

        <Col span={width_edges}>
          <BatteryGaugeText
            battery_level={device?.getBatteryLevel()}
            battery_voltage={device?.getBatteryVoltage()}
            showIcon={true}
          />
        </Col>

        <Col span={width_center}>
          <BleStatisticsText
            rssi={device?.getRssi()}
            rx={debug ? device?.getStats()?.getRx() : undefined}
            showIcon={true}
          />
        </Col>

        <Col span={width_edges}>
          <LastSeenBadge
            lastSeenDate={device?.getLastseen()?.toDate()}
            lastSeenValue={device?.getLastseenfromnow() || 0}
            secondsToConsiderOnline={SECONDS_TO_CONSIDER_ONLINE_BLE}
            minimalDisplay={true}
          ></LastSeenBadge>
        </Col>
      </Row>
    </LoadableCard>
  );
}

export default BleDeviceMetricsWidget;
