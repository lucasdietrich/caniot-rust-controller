import React, { useState } from "react";
import {
  Button,
  Form,
  Input,
  InputNumber,
  Radio,
  Row,
  Select,
  Slider,
  Space,
} from "antd";
import { NumberOutlined, SearchOutlined } from "@ant-design/icons";

function CaniotQueryForm() {
  const [form] = Form.useForm();
  const [frameType, setFrameType] = useState<string>("telemetry");

  return (
    <Form
      layout="horizontal" // horitzontal / inline
      variant="outlined"
      labelAlign="right"
      labelCol={{ span: 2 }}
      form={form}
      onFinish={() => console.log("Hello")}
      size="middle"
    >
      <Form.Item label="Device ID" name="did">
        <Space>
          <InputNumber
            style={{ width: "100px" }}
            max={64}
            min={0}
            defaultValue={0}
          ></InputNumber>
          <Space.Compact>
            {/* Disable hover */}
            <InputNumber
              style={{ width: "60px", pointerEvents: "none" }}
              readOnly
            ></InputNumber>
            <InputNumber
              style={{ width: "60px", pointerEvents: "none" }}
              readOnly
            ></InputNumber>
          </Space.Compact>
        </Space>
      </Form.Item>
      <Form.Item label="Frame type" name="frame_type">
        <Radio.Group
          value="inline"
          onChange={(e) => {
            console.log(e.target.value);
            setFrameType(e.target.value);
          }}
          buttonStyle="solid"
        >
          <Radio.Button value="telemetry">Telemetry</Radio.Button>
          <Radio.Button value="command">Command</Radio.Button>
          <Radio.Button value="attribute_read">Attribute Read</Radio.Button>
          <Radio.Button value="attribute_write">Attribute Write</Radio.Button>
        </Radio.Group>
      </Form.Item>

      <Form.Item name="slider" label="Timeout" initialValue={1.0}>
        <Slider
          style={{
            width: "300px",
          }}
          tooltip={{
            formatter: (value) => `${value} s`,
          }}
          step={0.1}
          max={5.0}
          min={0.1}
          marks={{
            0.0: "0",
            1.0: "1s",
            5.0: "5s",
          }}
        />
      </Form.Item>

      {(frameType === "telemetry" || frameType === "command") && (
        <Form.Item label="Endpoint" name="endpoint" initialValue="blc">
          <Select
            // onChange={onGenderChange}
            style={{ width: 200 }}
            // disabled={frameType !== "telemetry" && frameType !== "command"}
          >
            <Select.Option value="app0">(0) App Default</Select.Option>
            <Select.Option value="app1">(1) App 1</Select.Option>
            <Select.Option value="app2">(2) App 2</Select.Option>
            <Select.Option value="blc">
              (3) Board Level Controller
            </Select.Option>
          </Select>
        </Form.Item>
      )}
      {frameType.startsWith("attribute") && (
        <Form.Item label="Key" name="attribute_key">
          <Space.Compact>
            <InputNumber
              addonBefore={<NumberOutlined />}
              // disabled={!frameType.startsWith("attribute")}
              style={{ width: "140px" }}
            ></InputNumber>
            <InputNumber
              disabled={frameType !== "attribute_write"}
              style={{ width: "160px" }}
            ></InputNumber>
          </Space.Compact>
        </Form.Item>
      )}
      {frameType === "command" && (
        <Form.Item label="Payload" name="command_payload">
          <InputNumber
            // disabled={frameType !== "command"}
            style={{ width: "300px" }}
          ></InputNumber>
        </Form.Item>
      )}
      <Form.Item>
        <Button type="primary">Query</Button>
      </Form.Item>
    </Form>
  );
}

export default CaniotQueryForm;
