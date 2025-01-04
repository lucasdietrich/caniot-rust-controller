import LoadableCard from "./LoadableCard";
import { Device } from "@caniot-controller/caniot-api-grpc-web/api/ng_devices_pb";
import { useNavigate } from "react-router-dom";
import { Col, Divider, Row } from "antd";
import TemperatureGaugeStatistic, { TemperatureGaugeText } from "./Gauges";
import LastSeenBadge from "./LastSeenBadge";
import { TbCpu } from "react-icons/tb";
import { AppContext } from "../App";

interface DeviceMetricsWidgetProps {
  title?: string;
  device?: Device;
  loading: boolean;
  navigateTo: string;
  appContext: AppContext;
}

function DeviceMetricsWidget({
  title,
  device,
  loading,
  navigateTo,
  appContext,
}: DeviceMetricsWidgetProps) {
  const navigate = useNavigate();

  const showMinMaxColor = true;

  return (
    <LoadableCard
      title={title}
      onGoto={() => navigate(navigateTo)}
      loading={loading}
      status={device !== undefined}
      bordered={false}
      isMobile={appContext.isMobile}
    >
      <Row gutter={2}>
        <Col span={24}>
          <TemperatureGaugeStatistic
            title="Température ext"
            temperature={device?.hasOutsideTemp() ? device.getOutsideTemp() : undefined}
            indoor={false}
            summer={appContext.isSummer}
          />
        </Col>
      </Row>
      <Row gutter={2}>
        <Col span={12}>
          <TemperatureGaugeStatistic
            title="Min"
            temperature={device?.hasOutsideTempMin() ? device.getOutsideTempMin() : undefined}
            indoor={false}
            summer={appContext.isSummer}
            showColor={showMinMaxColor}
            small
          />
        </Col>
        <Col span={12}>
          <TemperatureGaugeStatistic
            title="Max"
            temperature={device?.hasOutsideTempMax() ? device.getOutsideTempMax() : undefined}
            indoor={false}
            summer={appContext.isSummer}
            showColor={showMinMaxColor}
            small
          />
        </Col>
      </Row>
      <Divider style={{ margin: 5 }} />
      <Row gutter={2}>
        <Col span={12}>
          <>
            {/* <HiOutlineCpuChip />  is bugged, i.e. console error */}
            <TbCpu />
            <TemperatureGaugeText
              temperature={device?.getBoardTemp()}
              showIcon={false}
              indoor={true}
            />
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
