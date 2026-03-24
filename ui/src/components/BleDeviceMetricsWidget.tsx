import LoadableCard from "./LoadableCard";
import { useNavigate } from "react-router-dom";
import { Col, Divider, Row, Tooltip } from "antd";
import TemperatureGaugeStatistic, {
  BatteryGaugeText,
  HumidityGaugeStatistic,
  BleStatisticsText,
  PowerGaugeStatistic,
  EnergyGaugeStatistic,
  CurrentGaugeStatistic,
} from "./Gauges";
import LastSeenBadge from "./LastSeenBadge";
import { TbPlugConnected } from "react-icons/tb";
import { CoproDevice } from "@caniot-controller/caniot-api-grpc-web/api/ng_copro_pb";
import { SECONDS_TO_CONSIDER_ONLINE_BLE } from "../constants";

interface BleDeviceMetricsWidgetProps {
  title?: string;
  device?: CoproDevice;
  loading: boolean;
  navigateTo?: string;
  small?: boolean;
  debug?: boolean;
  isSummer?: boolean;
}

function BleDeviceMetricsWidget({
  title,
  device,
  loading,
  navigateTo,
  small = false,
  debug = false,
  isSummer = false,
}: BleDeviceMetricsWidgetProps) {
  const navigate = useNavigate();

  const showMinMaxColor = true;

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
      {(() => {
        const env = device?.getEnvironemental();
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const valueExists = (v: any) => v !== undefined && v !== null;

        const tempVal = env?.getTemperature();
        const humVal = env?.getHumidity();
        const hasTemp = env?.hasTemperature ? env.hasTemperature() : valueExists(tempVal);
        const hasHum = env?.hasHumidity ? env.hasHumidity() : valueExists(humVal);

        const hasTempMin = env?.hasTemperatureMin ? env.hasTemperatureMin() : valueExists(env?.getTemperatureMin());
        const hasTempMax = env?.hasTemperatureMax ? env.hasTemperatureMax() : valueExists(env?.getTemperatureMax());
        const hasHumMin = env?.hasHumidityMin ? env.hasHumidityMin() : valueExists(env?.getHumidityMin());
        const hasHumMax = env?.hasHumidityMax ? env.hasHumidityMax() : valueExists(env?.getHumidityMax());

        const showEnvRow = hasTemp || hasHum;
        const showEnvMinMaxRow = hasTempMin || hasTempMax || hasHumMin || hasHumMax;

        const energy = device?.getEnergyMeter();
        const powerVal = energy?.getPower();
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const currentVal = energy?.getCurrent?.() ?? (energy as any)?.getCurrent?.();
        const energyVal = energy?.getEnergy();
        const hasPower = valueExists(powerVal);
  const hasCurrent = valueExists(currentVal);
        const hasEnergy = valueExists(energyVal);
  const showEnergyRow = hasPower || hasCurrent || hasEnergy;

  // Min / Max for power & current (optional presence)
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const powerMin = (energy as any)?.getPowerMin?.();
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const powerMax = (energy as any)?.getPowerMax?.();
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const currentMin = (energy as any)?.getCurrentMin?.();
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const currentMax = (energy as any)?.getCurrentMax?.();

  const hasPowerMin = valueExists(powerMin);
  const hasPowerMax = valueExists(powerMax);
  const hasCurrentMin = valueExists(currentMin);
  const hasCurrentMax = valueExists(currentMax);

  const showEnergyMinMaxRow = hasPowerMin || hasPowerMax || hasCurrentMin || hasCurrentMax;

        return (
          <>
            {showEnvRow && (
              <Row gutter={2}>
                {hasTemp && (
                  <Col span={12}>
                    <TemperatureGaugeStatistic
                      title="Température"
                      temperature={tempVal}
                      indoor={true}
                      summer={isSummer}
                    />
                  </Col>
                )}
                {hasHum && (
                  <Col span={hasTemp ? 12 : 24}>
                    <HumidityGaugeStatistic
                      title="Humidité"
                      humidity={humVal}
                    />
                  </Col>
                )}
              </Row>
            )}
            {showEnvMinMaxRow && (
              <Row gutter={2}>
                {hasTempMin && (
                  <Col span={6}>
                    <TemperatureGaugeStatistic
                      title="Min"
                      temperature={env?.getTemperatureMin()}
                      indoor={true}
                      showColor={showMinMaxColor}
                      small
                    />
                  </Col>
                )}
                {hasTempMax && (
                  <Col span={6}>
                    <TemperatureGaugeStatistic
                      title="Max"
                      temperature={env?.getTemperatureMax()}
                      indoor={true}
                      showColor={showMinMaxColor}
                      small
                    />
                  </Col>
                )}
                {hasHumMin && (
                  <Col span={6}>
                    <HumidityGaugeStatistic
                      title="Min"
                      humidity={env?.getHumidityMin()}
                      showColor={showMinMaxColor}
                      small
                    />
                  </Col>
                )}
                {hasHumMax && (
                  <Col span={6}>
                    <HumidityGaugeStatistic
                      title="Max"
                      humidity={env?.getHumidityMax()}
                      showColor={showMinMaxColor}
                      small
                    />
                  </Col>
                )}
              </Row>
            )}
            {(showEnvRow || showEnvMinMaxRow) && showEnergyRow && <Divider style={{ margin: 5 }} />}
            {showEnergyRow && (
              <>
                <Row gutter={2}>
                  {hasPower && (
                    <Col span={hasCurrent || hasEnergy ? 8 : 24}>
                      <PowerGaugeStatistic
                        title="Puissance (W)"
                        power_w={powerVal}
                      />
                    </Col>
                  )}
                  {hasCurrent && (
                    <Col span={hasPower && hasEnergy ? 8 : hasPower || hasEnergy ? 12 : 24}>
                      <CurrentGaugeStatistic
                        title="Courant (A)"
                        current_a={currentVal}
                      />
                    </Col>
                  )}
                  {hasEnergy && (
                    <Col span={hasPower || hasCurrent ? (hasPower && hasCurrent ? 8 : 12) : 24}>
                      <EnergyGaugeStatistic
                        title="Index (Wh)"
                        energy_wh={energyVal}
                      />
                    </Col>
                  )}
                </Row>
                {showEnergyMinMaxRow && (
                  <Row gutter={2} style={{ marginTop: 4 }}>
                    {hasPowerMin && (
                      <Col span={6}>
                        <PowerGaugeStatistic
                          title="Min"
                          power_w={powerMin}
                          small
                        />
                      </Col>
                    )}
                    {hasPowerMax && (
                      <Col span={6}>
                        <PowerGaugeStatistic
                          title="Max"
                          power_w={powerMax}
                          small
                        />
                      </Col>
                    )}
                    {hasCurrentMin && (
                      <Col span={6}>
                        <CurrentGaugeStatistic
                          title="Min"
                          current_a={currentMin}
                          small
                        />
                      </Col>
                    )}
                    {hasCurrentMax && (
                      <Col span={6}>
                        <CurrentGaugeStatistic
                          title="Max"
                          current_a={currentMax}
                          small
                        />
                      </Col>
                    )}
                  </Row>
                )}
                <Divider style={{ margin: 5 }} />
              </>
            )}
          </>
        );
      })()}
      <Row gutter={2}>
        {(() => {
          const battery_level = device?.getBatteryLevel();
          const battery_voltage = device?.getBatteryVoltage();
          const showBattery = device?.hasBatteryLevel() || device?.hasBatteryVoltage(); // hide if both missing
          return (
            <Col span={width_edges} style={{ display: "flex", alignItems: "center" }}>
              {showBattery ? (
                <BatteryGaugeText
                  battery_level={battery_level}
                  battery_voltage={battery_voltage}
                  showIcon={true}
                />
              ) : (
                <Tooltip title="Alimenté par le secteur">
                  <span style={{ color: "#555", fontSize: 12, display: "flex", alignItems: "center" }}>
                    <TbPlugConnected size={18} style={{ marginRight: 4 }} />
                    Secteur
                  </span>
                </Tooltip>
              )}
            </Col>
          );
        })()}

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
