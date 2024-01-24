import { useMapEvents } from "react-leaflet";

const MapClickHandler = ({ onClick }) => {
  useMapEvents({
    click: (e: React.ChangeEvent<HTMLInputElement>) => {
      onClick(e);
    },
  });

  return null;
};

export default MapClickHandler;
