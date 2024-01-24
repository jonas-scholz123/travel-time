import React from "react";

const TimeCard = ({ time, setTime }) => {
  const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setTime(e.target.value);
  };

  return (
    <div className="bg-white p-3 rounded-lg border border-gray-200 shadow ">
      <form className="w-full bg-white rounded">
        <h1 className="text-gray-600 font-semibold pb-2"> Departure Time: </h1>
        <div className="inline-flex text-lg border rounded-md p-2 h-9">
          <input
            className="text-gray-400 focus:outline-none font-light
      placeholder-gray-300 px-3"
            type="time"
            value={time}
            onChange={onChange}
          ></input>
        </div>
      </form>
    </div>
  );
};

export default TimeCard;
