import AddLocation from "./AddLocation";
import EnteredLocation from "./EnteredLocation";
import { useState } from "react";

function LocationCard(props) {

    const [locations, setLocations] = useState([])
    const [newLocError, setNewLocError] = useState("")
    const [newLocVal, setNewLocVal] = useState("")

    const onDelete = (idx) => {
        setLocations(locations.filter((_, i) => i !== idx))
        props.deleteCoords(idx)
    }

    const onNewLoc = (location, coords) => {
        if (locations.includes(location)) {
            setNewLocError("No duplicates");
            setNewLocVal(location);
            return;
        }
        setNewLocError("");
        setLocations([...locations, location])
        props.addCoords(coords)
    }

    const handleLocationChange = (idx, location, coords) => {
        console.log(idx, location, coords);
        let newLocations = [...locations];
        newLocations[idx] = location;
        setLocations(newLocations);
        props.changeCoords(idx, coords);
    }



    return (
        <div className="bg-white p-3 rounded-lg border border-gray-200 shadow ">
            <form className="w-full bg-white rounded">
                <h1 className="text-gray-600 font-semibold pb-2"> Locations: </h1>
                {locations.map((l, i) => {
                    return <EnteredLocation postcode={l} key={l} idx={i} onDelete={onDelete} handleSubmit={handleLocationChange} forbiddenNames={locations} />
                })}
                <AddLocation onClick={onNewLoc} error={newLocError} location={newLocVal} />
            </form>
        </div>
    );
}

export default LocationCard;