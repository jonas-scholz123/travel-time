import Slider from '@mui/material/Slider';
import clsx from 'clsx';
import { SliderThumb } from "@mui/material";
import { useEffect, useState } from 'react';

const CONFIG = require("../config.json");

function ThumbComponent(props) {
    const { children, className, ...other } = props;
    const extraClassName = "thumb" + other["data-index"].toString();
    return (
        <SliderThumb {...other} className={clsx(className, extraClassName)}>
            {children}
        </SliderThumb>
    );
}

function BoundsCard(props) {
    const [vals, setVals] = useState(props.bounds);
    // If external bounds change, this function handles it.
    useEffect(() => { setVals(props.bounds) }, [props.bounds]);

    const handleChange = (event, newVals) => {
        for (const [i, newVal] of newVals.entries()) {
            if (vals[i] != newVal) {
                // i is the index of the value that has changed.
                if (i > 0) {
                    newVals[i] = Math.max(newVals[i - 1] + CONFIG.minBoundSize, newVal);
                }
                if (i < newVals.length - 1) {
                    newVals[i] = Math.min(newVal, newVals[i + 1] - CONFIG.minBoundSize);
                }
            }
        }
        setVals(newVals);
    }

    let sx = {
        width: 300,
        color: 'lightgray',
        colorSecondary: 'lightgray',
        '& .MuiSlider-rail': {
            backgroundColor: 'lightgray',
            opacity: 1,
        },
    };

    for (const [i, colour] of props.colours.entries()) {
        sx['& .thumb' + i] = { backgroundColor: colour };
    }

    return (
        <div className="bg-white p-3 rounded-lg border border-gray-200 shadow my-2">
            <form className="w-full bg-white rounded">
                <h1 className="text-gray-600 font-semibold pb-2"> Bounds: </h1>
                <Slider
                    components={{ Thumb: ThumbComponent }}
                    value={vals}
                    max={120}
                    min={0}
                    onChange={handleChange}
                    onChangeCommitted={() => props.setBounds(vals)}
                    valueLabelDisplay="auto"
                    valueLabelFormat={(val, i) => {
                        const prevVal = i === 0 ? "0" : vals[i - 1].toString();
                        return prevVal + "-" + val + " mins";
                    }}
                    disableSwap
                    sx={sx}
                />
            </form>
        </div>
    );
}

export default BoundsCard;