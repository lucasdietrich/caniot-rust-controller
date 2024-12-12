import LoadableCard from "./LoadableCard";
import { Device } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import { useNavigate } from "react-router-dom";
import { Col, Divider, Row } from "antd";
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
}

function BleDeviceMetricsWidget({
  title,
  device,
  loading,
  navigateTo,
}: BleDeviceMetricsWidgetProps) {
  const navigate = useNavigate();

  return (
    <LoadableCard
      title={title}
      onGoto={navigateTo ? () => navigate(navigateTo) : undefined}
      loading={loading}
      status={device !== undefined}
      bordered={false}
      cardStyle={{
        opacity: (device?.getLastseenfromnow() ?? 0) > SECONDS_TO_CONSIDER_ONLINE_BLE ? 0.5 : 1,
      }}
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

        <Col span={6}>
          <>
            <BatteryGaugeText
              battery_level={device?.getBatteryLevel()}
              battery_voltage={device?.getBatteryVoltage()}
              showIcon={true}
            />
          </>
        </Col>

        <Col span={12}>
          <>
            <BleStatisticsText
              rssi={device?.getRssi()}
              rx={device?.getStats()?.getRx()}
              showIcon={true}
            />
          </>
        </Col>

        <Col span={6}>
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
