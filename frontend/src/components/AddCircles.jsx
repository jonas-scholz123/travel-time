import { useMap } from "react-leaflet";
import { useEffect, useState } from 'react';
import L from 'leaflet';

function AddCircles({ coords, radii }) {
    const map = useMap();
    var myRenderer = L.canvas();
}

export default AddCircles;