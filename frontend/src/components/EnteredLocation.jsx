import { useState } from "react";
import { FaMinus } from "react-icons/fa";

function EnteredLocation(props) {

    const [val, setVal] = useState(props.postcode);

    const onBlur = (e) => {
        console.log("triggered")
    }

    const onKeypress = (e) => {
        if (e.key === "Enter") {
            onBlur(e);
        }
    }

    const onChange = (e) => {
        setVal(e.target.value);
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
                        " type="text" value={val} onChange={onChange} onBlur={onBlur} onKeyDown={onKeypress}>

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