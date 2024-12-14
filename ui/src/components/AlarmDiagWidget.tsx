import React from "react";
import LoadableCard from "./LoadableCard";
import { Col, Divider, Row } from "antd";
import { useNavigate } from "react-router-dom";
import { OutdoorAlarmState } from "@caniot-controller/caniot-api-grpc-web/api/ng_alarms_pb";
import CounterGauge from "./CounterGauge";
import { LuEye, LuSiren } from "react-icons/lu";
import LastSeenSecondsCounter from "./LastSeenSecondsCounter";

interface AlarmDiagWidgetProps {
  title?: string;
  alarm?: OutdoorAlarmState;
  loading: boolean;
  navigateTo: string;
  isMobile?: boolean;
}

function AlarmDiagWidget({
  title,
  alarm,
  loading,
  navigateTo,
  isMobile = false,
}: AlarmDiagWidgetProps) {
  const navigate = useNavigate();

  return (
    <LoadableCard
      title={title}
      onGoto={() => navigate(navigateTo)}
      loading={loading}
      status={alarm !== undefined}
      bordered={false}
      isMobile={isMobile}
    >
      <Row gutter={2}>
        <Col span={6}>
          <CounterGauge title="sud" counter={alarm?.getSouthDetectorTriggeredCount()} />
        </Col>
        <Col span={6}>
          <CounterGauge title="est" counter={alarm?.getEastDetectorTriggeredCount()} />
        </Col>
        <Col span={6}>
          <CounterGauge title="sab" counter={alarm?.getSabotageTriggeredCount()} />
        </Col>
        <Col span={6}>
          <CounterGauge title="total " counter={alarm?.getSignalsTotalCount()} />
        </Col>

        <Col span={24}>
          <LuEye />
          <span
            style={{
              fontStyle: "italic",
              color: "darkgray",
              marginLeft: 5,
            }}
          >
            {!isMobile && alarm?.getLastSignal()?.toDate().toLocaleString()}
            <LastSeenSecondsCounter
              lastSeenValue={
                alarm?.hasLastSignalFromNowSeconds()
                  ? alarm?.getLastSignalFromNowSeconds()
                  : undefined
              }
              refreshIntervalMs={1000}
              minimalDisplay={isMobile}
              prefix=""
            />
          </span>
        </Col>

        <Divider style={{ margin: 5 }} />

        <Col span={24}>
          <CounterGauge
            title="activations de la sirène"
            counter={alarm?.getSirensTriggeredCount()}
          />
        </Col>

        <Col span={24}>
          <LuSiren />
          <span
            style={{
              fontStyle: "italic",
              color: "darkgray",
              marginLeft: 5,
            }}
          >
            {!isMobile && alarm?.getLastSiren()?.toDate().toLocaleString()}
            <LastSeenSecondsCounter
              lastSeenValue={
                alarm?.hasLastSirenFromNowSeconds()
                  ? alarm?.getLastSirenFromNowSeconds()
                  : undefined
              }
              refreshIntervalMs={1000}
              minimalDisplay={isMobile}
              prefix=""
            />
          </span>
        </Col>
      </Row>
    </LoadableCard>
  );
}

export default AlarmDiagWidget;
