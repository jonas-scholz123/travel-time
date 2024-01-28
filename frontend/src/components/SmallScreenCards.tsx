import React from "react";
import Draggable from "react-draggable";
import LocationCard from "./LocationCard";
import BoundsCard from "./BoundsCard";
import TimeCard from "./TimeCard";
import BackendStatusCard from "./BackendStatusCard";
import { FaButton } from "./FaButton";
import { FaBars, FaEyeSlash } from "react-icons/fa";

export const SmallScreenCards = (props) => {
  const [showCards, setShowCards] = React.useState(true);

  return (
    <div>
      {showCards && (
        <div className="absolute top-0 left-0 p-3 w-full z-10000">
          <LocationCard
            addLoc={props.addLoc}
            deleteLoc={props.deleteLoc}
            locations={props.locations}
          />
          {props.circles.length > 0 && (
            <BoundsCard
              colours={props.colours}
              setBounds={props.handleBoundsChange}
              bounds={props.bounds}
            />
          )}
          {props.circles.length > 0 && (
            <TimeCard time={props.time} setTime={props.setTime} />
          )}
          {!props.backendAwake && <BackendStatusCard />}
        </div>
      )}
      <div className="absolute bottom-0 right-0 p-3 z-10000">
        <FaButton
          faIcon={showCards ? <FaEyeSlash /> : <FaBars />}
          onClick={() => setShowCards(!showCards)}
        />
      </div>
    </div>
  );
};
