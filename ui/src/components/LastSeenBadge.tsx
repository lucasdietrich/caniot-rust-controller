import { Badge } from "antd";
import React, { useEffect, useState } from "react";
import LastSeenSecondsCounter from "./LastSeenSecondsCounter";

interface ILastSeenBadge {
  lastSeenDate?: Date;
  lastSeenValue: number;
  secondsToConsiderOnline?: number;
}

function LastSeenBadge({
  lastSeenDate,
  lastSeenValue,
  secondsToConsiderOnline = 60,
}: ILastSeenBadge) {
  if (lastSeenDate !== undefined) {
    const isOnline = lastSeenValue < secondsToConsiderOnline;
    const lastseen_fmt = lastSeenDate?.toLocaleString();

    return (
      <Badge
        status={isOnline ? "success" : "error"}
        text={
          <>
            {lastseen_fmt}
            <LastSeenSecondsCounter lastSeenValue={lastSeenValue} refreshIntervalMs={1000} />
          </>
        }
      />
    );
  } else {
    return <Badge status="default" text="Jamais" />;
  }
}

export default LastSeenBadge;
