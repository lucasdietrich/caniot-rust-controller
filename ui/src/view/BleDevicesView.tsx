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
import { AppContext } from "../App";

interface IBleDevicesViewProps {
  refreshInterval?: number;
  appContext: AppContext;
}

function BleDevicesView({ refreshInterval = 5000, appContext }: IBleDevicesViewProps) {
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

  // Determine if the copro device has an active alert that should be displayed
  let hasCoproAlertActive = false;
  if (coproAlert?.hasActiveAlert()) {
    const alertType = coproAlert.getActiveAlert()?.getAlertType();
    // Show copro alert if in debug mode or if alert is not OK/NOTIFICATION
    hasCoproAlertActive =
      appContext.uiDebugMode ||
      (alertType !== DeviceAlertType.OK && alertType !== DeviceAlertType.NOTIFICATION);
  }

  // Filter BLE devices to only those with active alerts
  const bleDevicesWithAlerts = bleDevicesList
    ? bleDevicesList.getDevicesList().filter((device) => device.hasActiveAlert())
    : [];

  // Filter out OK and NOTIFICATION alerts, unless in debug mode
  const bleDevicesWithNonOkAlerts = bleDevicesWithAlerts.filter((device) => {
    const alertType = device.getActiveAlert()?.getAlertType();
    return (
      appContext.uiDebugMode ||
      (alertType !== DeviceAlertType.OK && alertType !== DeviceAlertType.NOTIFICATION)
    );
  });

  // Determine if there are any BLE alerts to display
  const hasBleDevicesAlertsActive = bleDevicesWithNonOkAlerts.length > 0;

  const bleDevicesActiveAlerts = (
    <LoadableCard
      title="Alertes BLE actives"
      loading={bleDevicesLoading}
      bordered={false}
      isMobile={appContext.isMobile}
    >
      {hasCoproAlertActive || hasBleDevicesAlertsActive ? (
        <>
          {hasCoproAlertActive && coproAlert?.getActiveAlert() && (
            <DeviceAlert
              key="coproAlert"
              alert={coproAlert.getActiveAlert()}
              closable={false}
              isMobile={appContext.isMobile}
            />
          )}
          {hasBleDevicesAlertsActive &&
            bleDevicesWithNonOkAlerts.map((device) => (
              <DeviceAlert
                key={device.getName()}
                alert={device.getActiveAlert()}
                closable={false}
                isMobile={appContext.isMobile}
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
                small={appContext.isMobile}
                debug={appContext.uiDebugMode}
                isSummer={appContext.isSummer}
              />
            </Col>
          ))}
      </Row>
    </>
  );
}

export default BleDevicesView;
