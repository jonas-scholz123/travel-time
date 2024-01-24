import React from "react";
import { FaMinus } from "react-icons/fa";
import Location from "./Location";

function EnteredLocation(props: {
  location: Location;
  idx: number;
  onDelete: (idx: number) => void;
}) {
  const locName = props.location.name;

  return (
    <div className={"flex items-center shadow-sm rounded h-9 border mt-1"}>
      <div className="border-r h-full flex grow items-center bg-gray-100">
        <p className="appearance-none w-full text-gray-400 leading-tight focus:outline-none font-light px-3">
          {locName}
        </p>
      </div>
      <div
        className="bg-white text-gray-400 hover:bg-red-500 hover:text-white
                h-full rounded-r flex items-center w-9 justify-center hover:cursor-pointer"
        onClick={() => props.onDelete(props.idx)}
      >
        <button className="text-lg" type="button">
          <FaMinus />
        </button>
      </div>
    </div>
  );
}

export default EnteredLocation;
