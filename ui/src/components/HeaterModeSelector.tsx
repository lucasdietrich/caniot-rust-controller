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

function getHeaterModeLabel(mode: State, isMobile: boolean) {
  switch (mode) {
    case State.OFF:
      return (
        <>
          <PoweroffOutlined /> {!isMobile ? "Off" : undefined}
        </>
      );
    case State.COMFORT_ENERGY_SAVING:
      return (
        <>
          <LuLeaf /> {!isMobile ? "Eco" : undefined}
        </>
      );
    case State.COMFORT:
      return (
        <>
          <HomeOutlined /> {!isMobile ? "Comfort" : undefined}
        </>
      );
    case State.COMFORT_MIN_1:
      return <>{!isMobile ? "Comfort -1°C" : "-1°C"}</>;
    case State.COMFORT_MIN_2:
      return (
        <>
          {!isMobile ? (
            <>
              <MoonOutlined /> Comfort -2°C
            </>
          ) : (
            "-2°C"
          )}
        </>
      );
    case State.FROST_FREE:
      return (
        <>
          <LuThermometerSnowflake />
          {!isMobile ? "Hors-gel" : undefined}
        </>
      );
    default:
      return "";
  }
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
          <Radio.Button value={State.OFF}>{getHeaterModeLabel(State.OFF, isMobile)}</Radio.Button>
          <Radio.Button value={State.COMFORT_ENERGY_SAVING}>
            {getHeaterModeLabel(State.COMFORT_ENERGY_SAVING, isMobile)}
            {/* <ThunderboltOutlined /> Eco */}
          </Radio.Button>
          <Radio.Button value={State.COMFORT}>
            {getHeaterModeLabel(State.COMFORT, isMobile)}
          </Radio.Button>
          <Radio.Button value={State.COMFORT_MIN_1}>
            {getHeaterModeLabel(State.COMFORT_MIN_1, isMobile)}
          </Radio.Button>
          <Radio.Button value={State.COMFORT_MIN_2}>
            {getHeaterModeLabel(State.COMFORT_MIN_2, isMobile)}
          </Radio.Button>
          <Radio.Button value={State.FROST_FREE}>
            {getHeaterModeLabel(State.FROST_FREE, isMobile)}
          </Radio.Button>
        </Radio.Group>
      </Form.Item>
    </Form>
  );
}

export default HeaterModeSelector;
export { getHeaterModeLabel };
