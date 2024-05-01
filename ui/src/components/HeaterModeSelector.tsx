import {
  HomeOutlined,
  MinusOutlined,
  MoonOutlined,
  PoweroffOutlined,
  ReloadOutlined,
  SunFilled,
  SunOutlined,
  ThunderboltFilled,
  ThunderboltOutlined,
} from "@ant-design/icons";
import {
  Command,
  State,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_heaters_pb";
import {
  Button,
  Form,
  Radio,
  RadioChangeEvent,
  Select,
  Space,
  Spin,
} from "antd";
import React, { useState } from "react";
import { LuLeaf, LuThermometerSnowflake } from "react-icons/lu";
import { PiSnowflakeLight, PiSnowflakeThin } from "react-icons/pi";
import { TbSnowflake } from "react-icons/tb";
import heatersStore from "../store/HeatersStore";

interface IProps {
  label: string;
  heaterIndex: number;
  initialMode?: State;
  onModeChange?: (heaterIndex: number, mode: State) => void;
}

function HeaterModeSelector({
  label,
  heaterIndex: name,
  initialMode = State.NONE,
  onModeChange = () => {},
}: IProps) {
  const [form] = Form.useForm();

  console.log("HeaterModeSelector", name, initialMode);

  const disabled = initialMode === State.NONE;

  const onChange = (e: RadioChangeEvent) => {
    onModeChange(name, e.target.value);
  };

  return (
    <Form form={form}>
      <Form.Item label={label} name={name}>
        <Radio.Group
          disabled={disabled}
          buttonStyle="solid"
          onChange={onChange}
          value={initialMode}
        >
          <Radio.Button value={State.OFF}>
            <PoweroffOutlined /> Arrêt
          </Radio.Button>
          <Radio.Button value={State.COMFORT_ENERGY_SAVING}>
            <LuLeaf /> Eco
            {/* <ThunderboltOutlined /> Eco */}
          </Radio.Button>
          <Radio.Button value={State.COMFORT}>
            <HomeOutlined /> Comfort
          </Radio.Button>
          <Radio.Button value={State.COMFORT_MIN_1}>Comfort -1°C</Radio.Button>
          <Radio.Button value={State.COMFORT_MIN_2}>
            <MoonOutlined /> Comfort -2°C
          </Radio.Button>
          <Radio.Button value={State.FROST_FREE}>
            <LuThermometerSnowflake /> Hors-gel
          </Radio.Button>
        </Radio.Group>
      </Form.Item>
    </Form>
  );
}

export default HeaterModeSelector;
