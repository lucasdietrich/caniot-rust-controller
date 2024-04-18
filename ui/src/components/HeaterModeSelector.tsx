import {
  HomeOutlined,
  MinusOutlined,
  MoonOutlined,
  PoweroffOutlined,
  SunFilled,
  SunOutlined,
  ThunderboltFilled,
  ThunderboltOutlined,
} from "@ant-design/icons";
import { Form, Radio, Select, Space } from "antd";
import React from "react";
import { LuLeaf, LuThermometerSnowflake } from "react-icons/lu";
import { PiSnowflakeLight, PiSnowflakeThin } from "react-icons/pi";
import { TbSnowflake } from "react-icons/tb";

interface IProps {
  name: string;
  disabled?: boolean;
}

function HeaterModeSelector({ name, disabled = false }: IProps) {
  const [form] = Form.useForm();

  return (
    // is the name required ?
    // name="heaters" ?
    <Form form={form}>
      <Form.Item
        label={name}
        name="heater_mode"
        initialValue={!disabled && "off"}
      >
        <Radio.Group value="off" disabled={disabled} buttonStyle="solid">
          <Radio.Button value="off">
            <PoweroffOutlined /> Arrêt
          </Radio.Button>
          <Radio.Button value="eco">
            <LuLeaf /> Eco
            {/* <ThunderboltOutlined /> Eco */}
          </Radio.Button>
          <Radio.Button value="comfort">
            <HomeOutlined /> Comfort
          </Radio.Button>
          <Radio.Button value="comfort_m1">Comfort -1°C</Radio.Button>
          <Radio.Button value="comfort_m2">
            <MoonOutlined /> Comfort -2°C
          </Radio.Button>
          <Radio.Button value="frost_free">
            <LuThermometerSnowflake /> Hors-gel
          </Radio.Button>
        </Radio.Group>
      </Form.Item>
    </Form>
  );
}

export default HeaterModeSelector;
