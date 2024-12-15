import {
  CoproAlert,
  CoproDevicesList,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_copro_pb";
import { Col, Row } from "antd";
import React, { useEffect, useState } from "react";
import coproStore from "../store/CoproStore";
import BleDeviceMetricsWidget from "../components/BleDeviceMetricsWidget";
import LoadableCard from "../components/LoadableCard";
import DeviceAlert from "../components/DeviceAlert";
import { DeviceAlertType } from "@caniot-controller/caniot-api-grpc-web/api/common_pb";

interface IBleDevicesViewProps {
  refreshInterval?: number;
  isMobile?: boolean;
  uiDebugMode?: boolean;
}

function BleDevicesView({
  refreshInterval = 5000,
  isMobile = false,
  uiDebugMode = false,
}: IBleDevicesViewProps) {
  const [bleDevicesList, setBleDevicesList] = useState<CoproDevicesList | undefined>(undefined);
  const [bleDevicesLoading, setBleDevicesLoading] = useState(true);

  const [coproAlert, setCoproAlert] = useState<CoproAlert | undefined>(undefined);

  const [time, setTime] = useState(Date.now());

  useEffect(() => {
    setBleDevicesLoading(true);

    coproStore.getList((resp: CoproDevicesList) => {
      setBleDevicesList(resp);
      setBleDevicesLoading(false);
    });

    coproStore.getCoproAlert((resp: CoproAlert) => {
      setCoproAlert(resp);
    });

    const intervalRefresh = setInterval(() => setTime(Date.now()), refreshInterval);
    return () => {
      clearInterval(intervalRefresh);
    };
  }, [time]);

  let hasCoproAlertActive = false;
  if (
    coproAlert?.hasActiveAlert() &&
    (coproAlert?.getActiveAlert()?.getAlertType() == DeviceAlertType.OK ||
      coproAlert?.getActiveAlert()?.getAlertType() == DeviceAlertType.NOTIFICATION)
  ) {
    hasCoproAlertActive = uiDebugMode;
  }

  // const hasCoproAlertActive = coproAlert?.hasActiveAlert() ?? false;
  const bleDevicesWithAlerts = bleDevicesList
    ? bleDevicesList.getDevicesList().filter((device) => device.hasActiveAlert())
    : [];
  const hasBleDevicesAlertsActive = bleDevicesWithAlerts.length > 0;

  const bleDevicesActiveAlerts = (
    <LoadableCard
      title="Alertes BLE actives"
      loading={bleDevicesLoading}
      bordered={false}
      isMobile={isMobile}
    >
      {hasCoproAlertActive || hasBleDevicesAlertsActive ? (
        <>
          {hasCoproAlertActive && (
            <DeviceAlert
              key="coproAlert"
              alert={coproAlert?.getActiveAlert()}
              closable={false}
              isMobile={isMobile}
            />
          )}
          {hasBleDevicesAlertsActive &&
            bleDevicesWithAlerts.map((device) => (
              <DeviceAlert
                key={device.getName()}
                alert={device.getActiveAlert()}
                closable={false}
                isMobile={isMobile}
              />
            ))}
        </>
      ) : (
        <p>Aucune alerte active</p>
      )}
    </LoadableCard>
  );

  return (
    <>
      <Row gutter={16}>
        <Col span={24} style={{ marginBottom: 8 }}>
          {(hasCoproAlertActive || hasBleDevicesAlertsActive) && bleDevicesActiveAlerts}
        </Col>
        {/* each BLE device gets its own card */}
        {bleDevicesList &&
          bleDevicesList.getDevicesList().map((device) => (
            <Col xs={24} md={12} xl={6} style={{ marginBottom: 8 }} key={device.getMac()}>
              <BleDeviceMetricsWidget
                title={device.getName()}
                device={device}
                loading={bleDevicesLoading}
                small={isMobile}
                debug={uiDebugMode}
              />
            </Col>
          ))}
      </Row>
    </>
  );
}

export default BleDevicesView;
