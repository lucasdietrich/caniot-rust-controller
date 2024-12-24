import React, { useState } from "react";
import { Button, Card, Checkbox, Col, DatePicker, Form, Row, Slider, TimePicker } from "antd";
import TemperatureGaugeStatistic from "../components/Gauges";

const { RangePicker } = DatePicker;

const formItemLayout = {
  labelCol: {
    xs: { span: 24 },
    sm: { span: 8 },
  },
  wrapperCol: {
    xs: { span: 24 },
    sm: { span: 16 },
  },
};

const config = {
  rules: [{ type: "object" as const, required: true, message: "Please select time!" }],
};

const rangeConfig = {
  rules: [{ type: "array" as const, required: true, message: "Please select time!" }],
};

const onFinish = (fieldsValue: any) => {
  // Should format date value before submit.
  const rangeValue = fieldsValue["range-picker"];
  const rangeTimeValue = fieldsValue["range-time-picker"];
  const values = {
    ...fieldsValue,
    "date-picker": fieldsValue["date-picker"].format("YYYY-MM-DD"),
    "date-time-picker": fieldsValue["date-time-picker"].format("YYYY-MM-DD HH:mm:ss"),
    "month-picker": fieldsValue["month-picker"].format("YYYY-MM"),
    "range-picker": [rangeValue[0].format("YYYY-MM-DD"), rangeValue[1].format("YYYY-MM-DD")],
    "range-time-picker": [
      rangeTimeValue[0].format("YYYY-MM-DD HH:mm:ss"),
      rangeTimeValue[1].format("YYYY-MM-DD HH:mm:ss"),
    ],
    "time-picker": fieldsValue["time-picker"].format("HH:mm:ss"),
  };
  console.log("Received values of form: ", values);
};

function DemoView() {
  const [temperature, setTemperature] = useState(20);
  const [indoor, setIndoor] = useState(false);
  const [summer, setSummer] = useState(false);

  return (
    <>
      <Card title="Temperatures gauges">
        <Row>
          <Col span={3}>
            <TemperatureGaugeStatistic
              title="indoor very hot"
              temperature={temperature}
              indoor={indoor}
              summer={summer}
            />
          </Col>
          <Col span={19}>
            {" "}
            <Slider
              defaultValue={20}
              min={-5}
              max={40}
              onChange={(value) => setTemperature(value)}
              step={0.5}
              marks={{
                0: "0 °C",
                10: "10 °C",
                15: "15 °C",
                20: "20 °C",
                25: "25 °C",
                30: "30 °C",
              }}
            />
          </Col>
          <Col span={2}>
            <Checkbox checked={indoor} onChange={(e) => setIndoor(e.target.checked)}>
              Indoor
            </Checkbox>
            <Checkbox checked={summer} onChange={(e) => setSummer(e.target.checked)}>
              Summer
            </Checkbox>
          </Col>
        </Row>
      </Card>
    </>
  );
}

export default DemoView;
