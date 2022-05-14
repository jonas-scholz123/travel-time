import { FaPlus } from "react-icons/fa";
import { useEffect, useState } from "react";
import axios from "axios";

function AddLocation(props) {
    const [location, setLocation] = useState("");
    const [error, setError] = useState("");
    const [latlon, setLatlon] = useState(("", ""));

    const handleChange = (event) => {
        var loc = event.target.value;
        setLocation(loc);
    };

    const handleSubmit = async () => {
        axios
            .get("https://api.postcodes.io/postcodes/" + location)
            .then(response => {
                console.log("response: ", response);
                setLatlon([response.data.result.latitude, response.data.result.longitude])
            })
            .catch(error => setError(error.message))
    }

    useEffect(() => {
        if (latlon != "") {
            props.onClick(location, latlon);
            setLocation("");
            setLatlon("");
            setError("");
        }
    }, [latlon]);

    return (
        <div class={"flex items-center shadow-sm border rounded h-9 mt-5 " + (error == "" ? "" : "outline outline-red-500")}>
            <div class={"border-r h-full flex grow items-center rounded-l"}>
                <input class="
                        appearance-none
                        bg-transparent
                        w-full
                        text-gray-400
                        leading-tight
                        focus:outline-none
                        font-light
                        placeholder-gray-300
                        px-3
                        " type="text" placeholder="e.g. SW1A 2AA" value={location} onChange={handleChange}>
                </input>
            </div>
            <div class="bg-white text-green-500 hover:bg-green-500 hover:text-white
                h-full rounded-r flex items-center w-9 justify-center
                hover:cursor-pointer"
                onClick={handleSubmit}>
                <button class="text-lg" type="button">
                    <FaPlus />
                </button>
            </div>
        </div >
    )
}

export default AddLocation;