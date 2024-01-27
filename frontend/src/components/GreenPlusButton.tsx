import React from "react";
import { FaPlus } from "react-icons/fa";

export const GreenPlusButton = (props) => {
  const { onClick } = props;

  const className = `bg-white text-green-500 hover:bg-green-500 hover:text-white
            flex items-center justify-center
            hover:cursor-pointer border-gray-200 h-full p-2 rounded-r-lg border-l`;

  return (
    <div className={className}>
      <button className="text-lg" onClick={onClick}>
        <FaPlus />
      </button>
    </div>
  );
};
