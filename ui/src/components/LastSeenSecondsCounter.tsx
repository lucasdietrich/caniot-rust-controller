import React, { useEffect, useState } from "react";

interface ILastSeenSecondsCounterProps {
  lastSeenValue?: number;
  refreshIntervalMs?: number;
  minimalDisplay?: boolean;
  prefix?: string;
}

// Convert second to human readable format
// 20 -> 20s
// 125 -> 2m 5s
// 3665 -> 1h 1m 5s
// 86400 -> 1j 0h 0m 0s
function convertSecondsToHumanReadable(seconds: number) {
  const days = Math.floor(seconds / (3600 * 24));
  const hours = Math.floor((seconds % (3600 * 24)) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const remainingSeconds = seconds % 60;

  let fmt = "";
  if (days > 0) {
    fmt += days + "j ";
  }
  if (hours > 0) {
    fmt += hours + "h ";
  }
  if (minutes > 0) {
    fmt += minutes + "m ";
  }
  fmt += remainingSeconds + "s";

  return fmt;
}

function LastSeenSecondsCounter({
  lastSeenValue = 0,
  refreshIntervalMs = 1000,
  minimalDisplay = false,
  prefix = "actif il y a ",
}: ILastSeenSecondsCounterProps) {
  const [seconds, setSeconds] = useState(lastSeenValue);

  useEffect(() => {
    setSeconds(lastSeenValue);
    const id = setInterval(() => setSeconds((oldCount) => oldCount + 1), refreshIntervalMs);

    return () => {
      clearInterval(id);
    };
  }, [lastSeenValue, refreshIntervalMs]);

  const fmt_seconds = convertSecondsToHumanReadable(seconds);
  const fmt = minimalDisplay ? fmt_seconds : " (" + prefix + fmt_seconds + ")";

  return <>{fmt}</>;
}

export default LastSeenSecondsCounter;
