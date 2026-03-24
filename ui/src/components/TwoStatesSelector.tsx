import { Button, Radio, RadioChangeEvent, Space } from "antd";
import React from "react";

// // enum for on/off/toogle
// eslint-disable-next-line react-refresh/only-export-components
export enum TwoStateCommand {
  OFF = 0,
  ON = 1,
  TOGGLE = 2,
}

interface ITwoStatesSelectorProps {
  value?: boolean;
  toggleButton?: boolean;
  onStateChange?: (state: TwoStateCommand) => void;
  disabledIfValueUndefined?: boolean;
}

function TwoStatesSelector({
  value = undefined,
  toggleButton = true,
  onStateChange = () => {},
  disabledIfValueUndefined = true,
}: ITwoStatesSelectorProps) {
  const disabled = disabledIfValueUndefined && value === undefined;

  const onChange = (e: RadioChangeEvent) => {
    const value = e.target.value ? TwoStateCommand.ON : TwoStateCommand.OFF;
    onStateChange(value);
  };

  const onToggle = () => {
    onStateChange(TwoStateCommand.TOGGLE);
  };

  return (
    <>
      <Space direction="horizontal">
        <Radio.Group disabled={disabled} buttonStyle="solid" onChange={onChange} value={value}>
          <Radio.Button value={true}>On</Radio.Button>
          <Radio.Button value={false}>Off</Radio.Button>
        </Radio.Group>
        {toggleButton && (
          <Button onClick={onToggle} type="link" disabled={disabled}>
            Toggle
          </Button>
        )}
      </Space>
    </>
  );
}

export default TwoStatesSelector;
