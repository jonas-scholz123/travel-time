import AddLocation from "./AddLocation";
import EnteredLocation from "./EnteredLocation";
import { useEffect, useState } from "react";

function LocationCard(props) {

    const [locations, setLocations] = useState([])

    const onDelete = (idx) => {
        setLocations(locations.filter((_, i) => i !== idx))
        props.deleteCoords(idx)
    }

    const onNewLoc = (location, coords) => {
        setLocations([...locations, location])
        props.addCoords(coords)
    }


    return (
        <div className="bg-white p-3 rounded-lg border border-gray-200 shadow ">
            <form className="w-full bg-white rounded">
                <h1 className="text-gray-600 font-semibold pb-2"> Locations: </h1>
                {locations.map((l, i) => {
                    return <EnteredLocation postcode={l} key={l} idx={i} onDelete={onDelete} />
                })}
                <AddLocation onClick={onNewLoc} />
            </form>
        </div>
    );
}

export default LocationCard;