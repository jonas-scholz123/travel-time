import { useMap } from "react-leaflet";
import { useEffect, useState } from 'react';

const CONFIG = require("../config.json");

function ChangeView({ locs, changeView }) {
    const map = useMap();
    if (!changeView) {
        return null;
    }

    if (locs.length == 0) {
        map.fitBounds(CONFIG.startingBounds);
        return null;
    }

    if (locs.length == 1) {
        map.setView(locs[0], 14);
        return null;
    }

    map.fitBounds(locs, { padding: [CONFIG.padding, CONFIG.padding] });
    return null;
}


export default ChangeView;
