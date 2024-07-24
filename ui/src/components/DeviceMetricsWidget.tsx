import LoadableCard from "./LoadableCard";
import { Device } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import { useNavigate } from "react-router-dom";
import { Col, Divider, Row } from "antd";
import TemperatureGaugeStatistic, { TemperatureGaugeText } from "./TemperatureGauges";
import LastSeenBadge from "./LastSeenBadge";
import { TbCpu } from "react-icons/tb";

interface DeviceMetricsWidgetProps {
  title?: string;
  device?: Device;
  loading: boolean;
  navigateTo: string;
}

function DeviceMetricsWidget({ title, device, loading, navigateTo }: DeviceMetricsWidgetProps) {
  const navigate = useNavigate();

  return (
    <LoadableCard
      title={title}
      onGoto={() => navigate(navigateTo)}
      loading={loading}
      status={device !== undefined}
      bordered={false}
    >
      <Row gutter={2}>
        <Col span={24}>
          <TemperatureGaugeStatistic
            title="Température ext"
            temperature={device?.hasOutsideTemp() ? device.getOutsideTemp() : undefined}
          />
        </Col>

        <Divider style={{ margin: 5 }} />

        <Col span={12}>
          <>
            {/* <HiOutlineCpuChip />  is bugged, i.e. console error */}
            <TbCpu />
            <TemperatureGaugeText temperature={device?.getBoardTemp()} showIcon={false} />
          </>
        </Col>

        <Col span={12}>
          <LastSeenBadge
            lastSeenDate={device?.getLastseen()?.toDate()}
            lastSeenValue={device?.getLastseenfromnow() || 0}
            minimalDisplay={true}
          ></LastSeenBadge>
        </Col>
      </Row>
    </LoadableCard>
  );
}

export default DeviceMetricsWidget;
