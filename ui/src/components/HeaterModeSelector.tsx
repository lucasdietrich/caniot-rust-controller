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
import React, { useEffect, useState } from "react";
import { LuLeaf, LuThermometerSnowflake } from "react-icons/lu";
import { PiSnowflakeLight, PiSnowflakeThin } from "react-icons/pi";
import { TbSnowflake } from "react-icons/tb";
import heatersStore from "../store/HeatersStore";
import useFormItemStatus from "antd/es/form/hooks/useFormItemStatus";

interface IProps {
  label: string;
  heaterIndex: number;
  initialMode?: State;
  onModeChange?: (heaterIndex: number, mode: State) => void;
}

function HeaterModeSelector({
  label,
  heaterIndex,
  initialMode = State.NONE,
  onModeChange = () => {},
}: IProps) {
  const [form] = Form.useForm();

  console.log("HeaterModeSelector", heaterIndex, initialMode);

  const disabled = initialMode === State.NONE;

  const onChange = (e: RadioChangeEvent) => {
    onModeChange(heaterIndex, e.target.value);
  };

  // Reseting the fields is required in order to have the initial values set correctly
  // https://github.com/ant-design/ant-design/issues/22372
  useEffect(() => form.resetFields(), [initialMode]);

  return (
    <Form form={form} initialValues={{ [heaterIndex]: initialMode }}>
      <Form.Item label={label} name={heaterIndex}>
        <Radio.Group
          disabled={disabled}
          buttonStyle="solid"
          onChange={onChange}
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
