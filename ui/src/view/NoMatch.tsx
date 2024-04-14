import { Alert } from "antd";
import React from "react";

function NoMatch() {
  return (
    <div>
      <Alert message="Page not found" type="error"></Alert>
    </div>
  );
}

export default NoMatch;
