import { useMapEvents } from "react-leaflet";

const MapClickHandler = ({ onClick }) => {
  useMapEvents({
    click: (e) => {
      onClick(e);
    },
  });

  return null;
};

export default MapClickHandler;
