import { Badge } from "antd";
import React, { useEffect, useState } from "react";
import LastSeenSecondsCounter from "./LastSeenSecondsCounter";
import { SECONDS_TO_CONSIDER_ONLINE } from "../constants";

interface ILastSeenBadge {
  lastSeenDate?: Date;
  lastSeenValue?: number;
  secondsToConsiderOnline?: number;
  minimalDisplay?: boolean;
  counterPrefix?: string;
}

function LastSeenBadge({
  lastSeenDate,
  lastSeenValue = undefined,
  secondsToConsiderOnline = SECONDS_TO_CONSIDER_ONLINE,
  minimalDisplay = false,
  counterPrefix = undefined,
}: ILastSeenBadge) {
  if (lastSeenDate !== undefined) {
    const isOnline = lastSeenValue !== undefined && lastSeenValue < secondsToConsiderOnline;
    const lastseen_fmt = lastSeenDate?.toLocaleString();

    return (
      <Badge
        status={isOnline ? "success" : "error"}
        text={
          <>
            {!minimalDisplay && lastseen_fmt}
            {lastSeenValue !== undefined && (
              <LastSeenSecondsCounter
                lastSeenValue={lastSeenValue}
                refreshIntervalMs={1000}
                minimalDisplay={minimalDisplay}
                prefix={counterPrefix}
              />
            )}
          </>
        }
      />
    );
  } else {
    return <Badge status="default" text="Jamais" />;
  }
}

export default LastSeenBadge;
