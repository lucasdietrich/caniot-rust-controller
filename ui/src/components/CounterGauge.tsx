import { Statistic } from "antd";
import React from "react";

interface CounterGaugeProps {
  title?: string;
  counter?: number;
}

const ROUND_PRECISION = 0;

function CounterGauge({ title, counter }: CounterGaugeProps) {
  return counter !== undefined ? (
    <Statistic title={title} value={counter} precision={ROUND_PRECISION}></Statistic>
  ) : (
    <Statistic title={title} value="N/A" valueStyle={{ color: "gray" }}></Statistic>
  );
}

export default CounterGauge;
