import { CircularProgress } from '@mui/material';
import React from 'react'
import ReactDOM from 'react-dom'

function BoundsCard() {

    return (
        <div className="flex bg-white p-4 rounded-lg border border-gray-200 shadow my-2 justify-center items-center">
            <div className="flex flex-col grow justify-center items-center" >
                <p className="flex text-gray-600 font-light"> The backend is waking up... </p>
            </div>
            <CircularProgress size="1.5rem" />
        </div>
    );
}

export default BoundsCard;