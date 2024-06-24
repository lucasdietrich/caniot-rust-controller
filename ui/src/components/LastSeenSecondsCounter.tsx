import React, { useEffect, useState } from "react";

interface ILastSeenSecondsCounterProps {
  lastSeenValue?: number;
  refreshIntervalMs?: number;
  minimalDisplay?: boolean;
}

function LastSeenSecondsCounter({
  lastSeenValue = 0,
  refreshIntervalMs = 1000,
  minimalDisplay = false,
}: ILastSeenSecondsCounterProps) {
  const [seconds, setSeconds] = useState(lastSeenValue);

  useEffect(() => {
    setSeconds(lastSeenValue);
    const id = setInterval(() => setSeconds((oldCount) => oldCount + 1), refreshIntervalMs);

    return () => {
      clearInterval(id);
    };
  }, [lastSeenValue, refreshIntervalMs]);

  const fmt = minimalDisplay ? seconds + "s" : " (actif il y a " + seconds + "s)";

  return <>{fmt}</>;
}

export default LastSeenSecondsCounter;
