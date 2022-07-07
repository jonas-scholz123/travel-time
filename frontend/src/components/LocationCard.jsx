import AddLocation from "./AddLocation";
import EnteredLocation from "./EnteredLocation";
import { useState } from "react";

function LocationCard(props) {

  const [newLocError, setNewLocError] = useState("")

  const onNewLoc = (location, coords) => {
    if (props.locations.includes(location)) {
      setNewLocError("No duplicates");
      return;
    }
    setNewLocError("");
    props.addLoc(location, coords);
  }

  return (
    <div className="bg-white p-3 rounded-lg border border-gray-200 shadow ">
      <form className="w-full bg-white rounded">
        <h1 className="text-gray-600 font-semibold pb-2"> Locations: </h1>
        {props.locations.map((l, i) => {
          return <EnteredLocation
            postcode={l}
            key={l}
            idx={i}
            onDelete={props.deleteLoc}
            handleSubmit={props.changeLoc}
            forbiddenNames={props.locations} />
        })}
        <AddLocation
          onClick={onNewLoc}
          error={newLocError} />
      </form>
    </div>
  );
}

export default LocationCard;