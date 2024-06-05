import React, { useEffect, useState } from "react";

interface ILastSeenSecondsCounterProps {
  lastSeenValue?: number;
  refreshIntervalMs?: number;
}

function LastSeenSecondsCounter({
  lastSeenValue = 0,
  refreshIntervalMs = 1000,
}: ILastSeenSecondsCounterProps) {
  const [seconds, setSeconds] = useState(lastSeenValue);

  useEffect(() => {
    setSeconds(lastSeenValue);
    const id = setInterval(() => setSeconds((oldCount) => oldCount + 1), refreshIntervalMs);

    return () => {
      clearInterval(id);
    };
  }, [lastSeenValue, refreshIntervalMs]);

  return <>{" (actif il y a " + seconds + "s)"}</>;
}

export default LastSeenSecondsCounter;
