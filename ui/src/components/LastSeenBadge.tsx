import { Badge } from "antd";
import React, { useEffect, useState } from "react";
import LastSeenSecondsCounter from "./LastSeenSecondsCounter";
import { SECONDS_TO_CONSIDER_ONLINE } from "../constants";

interface ILastSeenBadge {
  lastSeenDate?: Date;
  lastSeenValue: number;
  secondsToConsiderOnline?: number;
  minimalDisplay?: boolean;
}

function LastSeenBadge({
  lastSeenDate,
  lastSeenValue,
  secondsToConsiderOnline = SECONDS_TO_CONSIDER_ONLINE,
  minimalDisplay = false,
}: ILastSeenBadge) {
  if (lastSeenDate !== undefined) {
    const isOnline = lastSeenValue < secondsToConsiderOnline;
    const lastseen_fmt = lastSeenDate?.toLocaleString();

    return (
      <Badge
        status={isOnline ? "success" : "error"}
        text={
          <>
            {!minimalDisplay && lastseen_fmt}
            <LastSeenSecondsCounter
              lastSeenValue={lastSeenValue}
              refreshIntervalMs={1000}
              minimalDisplay={minimalDisplay}
            />
          </>
        }
      />
    );
  } else {
    return <Badge status="default" text="Jamais" />;
  }
}

export default LastSeenBadge;
