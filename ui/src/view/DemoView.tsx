import React, { useState } from "react";
import { Card, Checkbox, Col, Row, Slider } from "antd";
import TemperatureGaugeStatistic from "../components/Gauges";

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
