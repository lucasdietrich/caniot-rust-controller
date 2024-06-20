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
import { Command, State } from "@caniot-controller/caniot-api-grpc-web/api/ng_heaters_pb";
import { Button, Form, Radio, RadioChangeEvent, Select, Space, Spin } from "antd";
import React, { useEffect, useState } from "react";
import { LuLeaf, LuThermometerSnowflake } from "react-icons/lu";
import { PiSnowflakeLight, PiSnowflakeThin } from "react-icons/pi";
import { TbSnowflake } from "react-icons/tb";
import heatersStore from "../store/HeatersStore";
import useFormItemStatus from "antd/es/form/hooks/useFormItemStatus";
import MediaMobile from "./responsive/MediaMobile";

interface IHeaterModeSelectorProps {
  label: string;
  heaterIndex: number;
  initialMode?: State;
  onModeChange?: (heaterIndex: number, mode: State) => void;
  isMobile?: boolean;
}

function HeaterModeSelector({
  label,
  heaterIndex,
  initialMode = State.NONE,
  onModeChange = () => {},
  isMobile = false,
}: IHeaterModeSelectorProps) {
  const [form] = Form.useForm();

  const disabled = initialMode === State.NONE;

  const onChange = (e: RadioChangeEvent) => {
    onModeChange(heaterIndex, e.target.value);
  };

  // Reseting the fields is required in order to have the initial values set correctly
  // https://github.com/ant-design/ant-design/issues/22372
  useEffect(() => form.resetFields(), [initialMode]);

  return (
    <Form form={form} initialValues={{ [heaterIndex]: initialMode }}>
      <Form.Item label={!isMobile ? label : undefined} name={heaterIndex}>
        <Radio.Group
          disabled={disabled}
          buttonStyle="solid"
          onChange={onChange}
          size={isMobile ? "large" : "middle"}
        >
          <Radio.Button value={State.OFF}>
            <PoweroffOutlined /> {!isMobile ? "Off" : undefined}
          </Radio.Button>
          <Radio.Button value={State.COMFORT_ENERGY_SAVING}>
            <LuLeaf /> {!isMobile ? "Eco" : undefined}
            {/* <ThunderboltOutlined /> Eco */}
          </Radio.Button>
          <Radio.Button value={State.COMFORT}>
            <HomeOutlined /> {!isMobile ? "Comfort" : undefined}
          </Radio.Button>
          <Radio.Button value={State.COMFORT_MIN_1}>
            {!isMobile ? "Comfort -1°C" : "-1°C"}
          </Radio.Button>
          <Radio.Button value={State.COMFORT_MIN_2}>
            {!isMobile ? (
              <>
                <MoonOutlined /> Comfort -2°C
              </>
            ) : (
              "-2°C"
            )}
          </Radio.Button>
          <Radio.Button value={State.FROST_FREE}>
            <LuThermometerSnowflake />
            {!isMobile ? "Hors-gel" : undefined}
          </Radio.Button>
        </Radio.Group>
      </Form.Item>
    </Form>
  );
}

export default HeaterModeSelector;
