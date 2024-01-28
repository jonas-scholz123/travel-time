import axios from "axios";
import config from "../config";
import { useCallback, useEffect } from "react";

const isBackendAwake = async () => {
  try {
    await axios.get(encodeURI(config.backendUrl));
    return true;
  } catch (e) {
    return false;
  }
};

export const HealthChecker = (props) => {
  const repeatHealthCheckUntilAwake = useCallback(
    async (backendAwake: boolean, setBackendAwake) => {
      const awake = await isBackendAwake();
      if (!awake) {
        setTimeout(
          () => repeatHealthCheckUntilAwake(awake, setBackendAwake),
          5000,
        );
      } else {
        setBackendAwake(awake);
      }
    },
    [],
  );

  useEffect(() => {
    console.log(`Health checking backend at ${config.backendUrl}.`);
    repeatHealthCheckUntilAwake(props.backendAwake, props.setBackendAwake);
  }, [props.backendAwake, props.setBackendAwake, repeatHealthCheckUntilAwake]);

  return null;
};
