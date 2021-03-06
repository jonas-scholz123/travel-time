import { FaPlus } from "react-icons/fa";
import { useEffect, useState } from "react";
import axios from "axios";

function AddLocation(props) {
  const [location, setLocation] = useState("");
  const [error, setError] = useState(props.error);

  useEffect(() => { setError(props.error) }, [props.error]);

  const handleChange = (event) => {
    setLocation(event.target.value);
  };

  const handleSubmit = async () => {
    axios
      .get("https://api.postcodes.io/postcodes/" + location)
      .then(response => {
        const latlon = [response.data.result.latitude, response.data.result.longitude];
        props.onClick(location, latlon);
        setLocation("");
        setError("");
      })
      .catch(_ => setError("Unknown postcode"))
  }

  const onKeypress = (e) => {
    if (e.key === "Enter") {
      e.preventDefault();
      handleSubmit();
    }
  }

  return (
    <div className="mt-5">
      {error !== "" && <p className="mb-1 text-xs text-red-500">{error}</p>}
      <div className={"flex items-center shadow-sm border rounded h-9 " + (error === "" ? "" : "outline outline-red-500")}>
        <div className={"border-r h-full flex grow items-center rounded-l"}>
          <input className="
                        appearance-none
                        bg-transparent
                        w-full
                        text-gray-400
                        leading-tight
                        focus:outline-none
                        font-light
                        placeholder-gray-300
                        px-3
                        "
            type="text"
            placeholder="e.g. SW1A 2AA"
            value={location}
            onChange={handleChange}
            onKeyDown={onKeypress}>
          </input>
        </div>
        <div className="bg-white text-green-500 hover:bg-green-500 hover:text-white
                h-full rounded-r flex items-center w-9 justify-center
                hover:cursor-pointer"
          onClick={handleSubmit}>
          <button className="text-lg" type="button">
            <FaPlus />
          </button>
        </div>
      </div >
    </div>
  )
}

export default AddLocation;