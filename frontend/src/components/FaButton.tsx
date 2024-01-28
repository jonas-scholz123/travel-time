import React from "react";

export const FaButton = (props) => {
  const { onClick } = props;

  const className = `bg-white text-green-500 hover:bg-green-500 hover:text-white
            flex items-center justify-center
            hover:cursor-pointer border-gray-200 h-full p-4 rounded-lg border-l`;

  return (
    <div className={className}>
      <button className="text-lg" onClick={onClick}>
        {props.faIcon}
      </button>
    </div>
  );
};
