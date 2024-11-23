import { Card, Select, Slider, SliderSingleProps, Switch, Table, TimePicker } from "antd";
import { State } from "@caniot-controller/caniot-api-grpc-web/api/ng_heaters_pb";
import React from "react";
import dayjs from "dayjs";
import { MoonOutlined } from "@ant-design/icons";
import { getHeaterModeLabel } from "./HeaterModeSelector";

const timeFormat = "HH:mm";

const setpointMarks: SliderSingleProps["marks"] = {
  15: "15°C",
  18: "18°C",
  20: "20°C",
  21: "21°C",
  25: {
    style: {
      color: "#f50",
    },
    label: <strong>25</strong>,
  },
};

const timeMarks: SliderSingleProps["marks"] = {
  0: "00 H",
  6: "06 H",
  8: "8 H",
  12: "midi",
  16: "16 H",
  18: "18 H",
  22: "22 H",
  24: "minuit",
};

const setpointsOptions = [
  { value: 15, label: "15°C" },
  { value: 16, label: "16°C" },
  { value: 17, label: "17°C" },
  { value: 18, label: "18°C" },
  { value: 19, label: "19°C" },
  { value: 20, label: "20°C" },
  { value: 21, label: "21°C" },
  { value: 22, label: "22°C" },
  { value: 23, label: "23°C" },
  { value: 24, label: "24°C" },
  { value: 25, label: "25°C" },
];

interface IHeatersSchedulingCardProps {
  isMobile?: boolean;
}

function HeatersSchedulingCard({ isMobile = false }: IHeatersSchedulingCardProps) {
  // List of days in French
  const weekdaysLongs = ["Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi", "Dimanche"];
  const weekdaysShorts = ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"];

  const weekdays = isMobile ? weekdaysShorts : weekdaysLongs;

  const columns2 = [
    {
      title: "Jour",
      dataIndex: "day",
      key: "day",
      width: isMobile ? "20px" : "60px",
    },
    {
      title: "Mode de chauffe",
      dataIndex: "heating-mode",
      key: "heating-mode",
      width: isMobile ? "40px" : "150px",
    },
    {
      title: "Température",
      dataIndex: "setpoint",
      key: "setpoint",
      width: isMobile ? "40px" : "80px",
    },
    {
      title: "Plage horaire",
      dataIndex: "time",
      key: "time",
    },
  ];

  const modesOptions = [
    {
      value: "off",
      label: getHeaterModeLabel(State.OFF, isMobile),
    },
    { value: "eco", label: getHeaterModeLabel(State.COMFORT_ENERGY_SAVING, isMobile) },
    { value: "comfort", label: getHeaterModeLabel(State.COMFORT, isMobile) },
    { value: "comfort-1", label: getHeaterModeLabel(State.COMFORT_MIN_1, isMobile) },
    { value: "comfort-2", label: getHeaterModeLabel(State.COMFORT_MIN_2, isMobile) },
    { value: "frost-free", label: getHeaterModeLabel(State.FROST_FREE, isMobile) },
  ];

  const generateDataSource2 = () => {
    return weekdays.map((day, index) => ({
      key: index.toString(),
      day,
      "heating-mode": (
        <Select
          style={{ width: "100%" }}
          showSearch={!isMobile}
          placeholder="Mode"
          options={modesOptions}
          defaultValue={"comfort"}
        />
      ),
      setpoint: (
        <Select
          showSearch={!isMobile}
          placeholder="Setpoint"
          style={{ width: "100%" }}
          options={setpointsOptions}
          defaultValue={20}
        />
      ),
      time: (
        <Slider
          marks={timeMarks}
          step={1}
          defaultValue={[6, 22]}
          range
          min={0}
          max={24}
          style={{}}
          tooltip={{
            placement: "top",
            visible: true,
            formatter: (value) => `${value} h`,
          }}
        />
      ),
    }));
  };

  const dataSource2 = generateDataSource2();

  return (
    <Card title={"Programmation"}>
      <Switch defaultChecked />
      <TimePicker
        defaultValue={dayjs("12:08", timeFormat)}
        format={timeFormat}
        minuteStep={15}
        size="large"
      />
      Mode de veille
      <Select showSearch placeholder="Mode" options={modesOptions} />
      <Table dataSource={dataSource2} columns={columns2} size={isMobile ? "small" : "middle"} />;
    </Card>
  );
}

export default HeatersSchedulingCard;
