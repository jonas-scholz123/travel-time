import React from "react";
import AddPostcode from "./AddPostcode";
import EnteredLocation from "./EnteredLocation";
import { useState } from "react";
import Location from "./Location";

function LocationCard(props: {
  locations: Location[];
  addLoc: (loc: Location) => void;
  deleteLoc: (idx: number) => void;
}) {
  const [newLocError, setNewLocError] = useState("");
  const names = props.locations.map((l) => l.name);

  const onNewLoc = (location: Location) => {
    if (names.includes(location.name)) {
      setNewLocError("No duplicates");
      return;
    }
    setNewLocError("");
    props.addLoc(location);
  };

  return (
    <div className="bg-white p-3 rounded-lg border border-gray-200 shadow ">
      <form className="w-full bg-white rounded">
        <h1 className="text-gray-600 font-semibold pb-2"> Locations: </h1>
        {props.locations.map((l, i) => {
          return (
            <EnteredLocation
              key={l.name}
              location={l}
              idx={i}
              onDelete={props.deleteLoc}
            />
          );
        })}
        <AddPostcode onClick={onNewLoc} error={newLocError} />
      </form>
    </div>
  );
}

export default LocationCard;
