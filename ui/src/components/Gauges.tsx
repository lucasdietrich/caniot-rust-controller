import { Statistic } from "antd";
import React from "react";
import {
  BsBatteryFull,
  BsThermometer,
  BsThermometerHalf,
  BsThermometerHigh,
  BsThermometerLow,
  BsThermometerSnow,
  BsThermometerSun,
} from "react-icons/bs";
import {
  FaBatteryEmpty,
  FaBatteryFull,
  FaBatteryHalf,
  FaBatteryQuarter,
  FaBatteryThreeQuarters,
  FaDroplet,
  FaDropletSlash,
  FaTemperatureEmpty,
  FaTemperatureHalf,
} from "react-icons/fa6";
import { LuBluetoothConnected } from "react-icons/lu";
import { MdSignalCellularConnectedNoInternet4Bar } from "react-icons/md";

const ROUND_PRECISION = 1;

// Yeaye ! Thx chatgpt, i don't understand anything in this function but it works :D
/**
 * Interpolates between two hex colors and returns the resulting color as a hex string.
 * @param colorFrom - The starting color in hex format, e.g. '#ff0000'
 * @param colorTo - The ending color in hex format, e.g. '#00ff00'
 * @param weight - A number between 0 and 1 representing how far along the gradient to go.
 *                 0 will return colorFrom, 1 will return colorTo, 0.5 will return the midpoint, etc.
 * @returns A hex string representing the interpolated color, e.g. '#808000'
 */
export function interpolateColor(colorFrom: string, colorTo: string, weight: number): string {
  // Ensure weight is clamped between 0 and 1
  const w = Math.min(Math.max(weight, 0), 1);

  // Remove '#' if present and parse to integer values
  const fromInt = parseInt(colorFrom.replace("#", ""), 16);
  const toInt = parseInt(colorTo.replace("#", ""), 16);

  // Extract RGB components from the integer values
  const fromR = (fromInt >> 16) & 0xff;
  const fromG = (fromInt >> 8) & 0xff;
  const fromB = fromInt & 0xff;

  const toR = (toInt >> 16) & 0xff;
  const toG = (toInt >> 8) & 0xff;
  const toB = toInt & 0xff;

  // Interpolate each component based on the weight
  const r = Math.round(fromR + (toR - fromR) * w);
  const g = Math.round(fromG + (toG - fromG) * w);
  const b = Math.round(fromB + (toB - fromB) * w);

  // Convert back to a hex string
  const rr = r.toString(16).padStart(2, "0");
  const gg = g.toString(16).padStart(2, "0");
  const bb = b.toString(16).padStart(2, "0");

  return `#${rr}${gg}${bb}`;
}

export function GetTemperatureColor(temp: number) {
  if (temp < 0) {
    return "#AAF7FF";
  } else if (temp < 14) {
    return "#3FA0FF";
  } else if (temp < 22) {
    return "#2cde73";
  } else if (temp < 30) {
    return "#ffad72";
  } else if (temp < 40) {
    return "#f76d5e";
  } else {
    return "#D82632";
  }
}

export function GetTemperatureIcon(temp: number) {
  if (temp < 0) {
    return <BsThermometerSnow />;
  } else if (temp < 14) {
    return <BsThermometerLow />;
  } else if (temp < 22) {
    return <BsThermometerHalf />;
  } else if (temp < 30) {
    return <BsThermometerHigh />;
  } else {
    return <BsThermometerSun />;
  }
}

interface TemperatureGaugeProps {
  title?: string;
  temperature?: number;
  showIcon?: boolean;
}

function TemperatureGaugeStatistic({ title, temperature, showIcon = true }: TemperatureGaugeProps) {
  return temperature !== undefined ? (
    <Statistic
      title={title}
      value={temperature}
      precision={ROUND_PRECISION}
      valueStyle={{ color: GetTemperatureColor(temperature) }}
      prefix={showIcon && GetTemperatureIcon(temperature)}
      suffix="°C"
    ></Statistic>
  ) : (
    <Statistic
      title={title}
      value="N/A"
      valueStyle={{ color: "gray" }}
      prefix={<FaTemperatureEmpty />}
      suffix="°C"
    ></Statistic>
  );
}

function TemperatureGaugeText({ temperature, showIcon = true }: TemperatureGaugeProps) {
  return temperature ? (
    <span>
      {showIcon && GetTemperatureIcon(temperature)} {Math.round(temperature * 10) / 10} °C
    </span>
  ) : (
    <span>{showIcon && <FaTemperatureEmpty />} N/A °C</span>
  );
}

interface HumidityGaugeProps {
  title?: string;
  humidity?: number;
  showIcon?: boolean;
}

export function GetHumidityIcon(humidity: number) {
  return <FaDroplet />;
}

function HumidityGaugeText({ humidity, showIcon = true }: HumidityGaugeProps) {
  return humidity ? (
    <span>
      {showIcon && GetHumidityIcon(humidity)} {Math.round(humidity)} %
    </span>
  ) : (
    <span>{showIcon && <FaDropletSlash />} N/A %</span>
  );
}

export function GetHumidityColor(humidity: number) {
  return interpolateColor("#faf3ca", "#119af5", humidity / 100);
}

function HumidityGaugeStatistic({ title, humidity, showIcon = true }: HumidityGaugeProps) {
  return humidity !== undefined ? (
    <Statistic
      title={title}
      value={Math.round(humidity * 10) / 10}
      valueStyle={{ color: GetHumidityColor(humidity) }}
      prefix={showIcon && GetHumidityIcon(humidity)}
      suffix="%"
    ></Statistic>
  ) : (
    <Statistic
      title={title}
      value="N/A"
      valueStyle={{ color: "gray" }}
      prefix={<FaDropletSlash />}
      suffix="%"
    ></Statistic>
  );
}

interface BatteryGaugeProps {
  title?: string;
  battery_level?: number;
  battery_voltage?: number;
  showIcon?: boolean;
}

export function GetBatteryIcon(battery_level: number) {
  if (battery_level < 12.5) {
    return <FaBatteryEmpty />;
  } else if (battery_level < 25 + 12.5) {
    return <FaBatteryQuarter />;
  } else if (battery_level < 50 + 12.5) {
    return <FaBatteryHalf />;
  } else if (battery_level < 75 + 12.5) {
    return <FaBatteryThreeQuarters />;
  } else {
    return <FaBatteryFull />;
  }
}

function BatteryGaugeText({ battery_level, battery_voltage, showIcon = true }: BatteryGaugeProps) {
  return battery_level !== undefined && battery_voltage ? (
    <span title={Math.round(battery_voltage * 1000) / 1000 + "V"}>
      {showIcon && GetBatteryIcon(battery_level)} {battery_level} %
    </span>
  ) : (
    <span>{showIcon && <FaBatteryEmpty />} N/A %</span>
  );
}

function BleStatisticsText({
  rssi,
  rx,
  showIcon,
}: {
  rssi: number | undefined;
  rx: number | undefined;
  showIcon: boolean;
}) {
  return rssi ? (
    <span>
      {showIcon && <LuBluetoothConnected />}
      {rssi} dBm / {rx}p
    </span>
  ) : (
    <span>
      {showIcon && <MdSignalCellularConnectedNoInternet4Bar />}
      N/A dBm / {rx}p
    </span>
  );
}

export default TemperatureGaugeStatistic;

export {
  HumidityGaugeStatistic,
  TemperatureGaugeText,
  HumidityGaugeText,
  BatteryGaugeText,
  BleStatisticsText,
};
