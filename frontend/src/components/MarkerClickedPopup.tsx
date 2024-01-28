import React from "react";
import { GreenPlusButton } from "./GreenPlusButton";

const MarkerClickedPopup = (props): React.ReactElement => {
  return (
    <div
      className="flex items-center justify-between border-red-200 rounded-12"
      onClick={props.onPopupClick}
    >
      <p className="!m-0 pl-2 pr-3 text-[14px]">Add Location </p>
      <GreenPlusButton onClick={props.addLoc} />
    </div>
  );
};

export default MarkerClickedPopup;
