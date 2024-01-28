import React from "react";
import { GreenPlusButton } from "./GreenPlusButton";

const MarkerClickedPopup = (props): React.ReactElement => {
  return (
    <div className="rounded-12 flex items-center justify-between border-red-200">
      <p className="!m-0 pl-2 pr-3 text-[14px]" onClick={props.onPopupClick}>
        Add Location
      </p>
      <GreenPlusButton onClick={props.addLoc} />
    </div>
  );
};

export default MarkerClickedPopup;
