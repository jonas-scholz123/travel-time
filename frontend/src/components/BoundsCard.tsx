import React from "react";
import Slider from "@mui/material/Slider";
import clsx from "clsx";
import { SliderThumb } from "@mui/material";
import { useEffect, useState } from "react";
import config from "../config";

function ThumbComponent(props: { children: any; className: any; other: any }) {
  const { children, className, ...other } = props;
  const extraClassName = "thumb" + other["data-index"].toString();
  return (
    <SliderThumb {...other} className={clsx(className, extraClassName)}>
      {children}
    </SliderThumb>
  );
}

function BoundsCard(props: {
  bounds: number[];
  colours: string[];
  setBounds: (bounds: number[]) => void;
}) {
  const [vals, setVals] = useState(props.bounds);
  // If external bounds change, this function handles it.
  useEffect(() => {
    setVals(props.bounds);
  }, [props.bounds]);

  const handleChange = (event: any, newVals: any[]) => {
    for (const [i, newVal] of newVals.entries()) {
      if (vals[i] !== newVal) {
        // i is the index of the value that has changed.
        if (i > 0) {
          newVals[i] = Math.max(newVals[i - 1] + config.minBoundSize, newVal);
        }
        if (i < newVals.length - 1) {
          newVals[i] = Math.min(newVal, newVals[i + 1] - config.minBoundSize);
        }
      }
    }
    setVals(newVals);
  };

  let sx = {
    width: 300,
    color: "lightgray",
    colorSecondary: "lightgray",
    "& .MuiSlider-rail": {
      backgroundColor: "lightgray",
      opacity: 1,
    },
  };

  for (const [i, colour] of props.colours.entries()) {
    sx["& .thumb" + i] = { backgroundColor: colour };
  }

  return (
    <div className="my-2 rounded-lg border border-gray-200 bg-white p-3 shadow">
      <form className="w-full rounded bg-white">
        <h1 className="pb-2 font-semibold text-gray-600"> Bounds: </h1>
        <Slider
          components={{ Thumb: ThumbComponent }}
          value={vals}
          max={config.maxTravelMins}
          min={0}
          onChange={handleChange}
          onChangeCommitted={() => props.setBounds(vals)}
          valueLabelDisplay="auto"
          valueLabelFormat={(val, i) => {
            const prevVal = i === 0 ? "0" : vals[i - 1].toString();
            return prevVal + "-" + val + " mins";
          }}
          disableSwap
          sx={sx}
        />
      </form>
    </div>
  );
}

export default BoundsCard;
