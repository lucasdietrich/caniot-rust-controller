import { Statistic } from "antd";
import React from "react";
import {
  BsThermometer,
  BsThermometerHalf,
  BsThermometerHigh,
  BsThermometerLow,
  BsThermometerSnow,
  BsThermometerSun,
} from "react-icons/bs";
import { FaTemperatureEmpty, FaTemperatureHalf } from "react-icons/fa6";

const ROUND_PRECISION = 1;

export function GetTemperatureColor(temp: number) {
  if (temp < 0) {
    return "#3FA0FF";
  } else if (temp < 14) {
    return "#AAF7FF";
  } else if (temp < 22) {
    return "#FFFFBF";
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
  return temperature ? (
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

export default TemperatureGaugeStatistic;

function TemperatureGaugeText({ temperature, showIcon = true }: TemperatureGaugeProps) {
  return temperature ? (
    <span>
      {showIcon && GetTemperatureIcon(temperature)} {Math.round(temperature * 10) / 10} °C
    </span>
  ) : (
    <span>{showIcon && <FaTemperatureEmpty />} N/A °C</span>
  );
}

export { TemperatureGaugeText };
