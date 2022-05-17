import { useState, useEffect } from "react";
import { FaMinus } from "react-icons/fa";
import axios from "axios";

function EnteredLocation(props) {

    const [location, setLocation] = useState(props.postcode);
    const [coords, setCoords] = useState([]);
    const [error, setError] = useState("");

    useEffect(() => {
        if (coords.length > 0)
            props.handleSubmit(props.idx, location, coords);
    }, [coords])

    const onKeypress = (e) => {
        if (e.key === "Enter") {
            handleChange();
        }
    }

    const onChange = (e) => {
        setLocation(e.target.value);
    }

    const handleChange = () => {
        axios
            .get("https://api.postcodes.io/postcodes/" + location)
            .then(response => {
                setCoords([response.data.result.latitude, response.data.result.longitude])
            })
            .catch(error => setError(error.message))
    }

    return (
        <div className="flex items-center shadow-sm rounded h-9 border mt-1">
            <div className="border-r h-full flex grow items-center">
                <input className="
                        appearance-none
                        bg-transparent
                        w-full
                        text-gray-400
                        leading-tight
                        focus:outline-none
                        font-light
                        px-3
                        " type="text" value={location} onChange={onChange} onBlur={handleChange} onKeyDown={onKeypress}>

                </input>
            </div>
            <div className="bg-white text-gray-400 hover:bg-red-500 hover:text-white
                h-full rounded-r flex items-center w-9 justify-center hover:cursor-pointer"
                onClick={() => props.onDelete(props.idx)}>

                <button className="text-lg" type="button">
                    <FaMinus />
                </button>
            </div>
        </div>
    )
}

export default EnteredLocation;