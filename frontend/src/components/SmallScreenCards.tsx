import React from "react";
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
        <div className="z-10000 absolute left-0 top-0 w-full p-3">
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
      <div className="z-10000 absolute bottom-0 right-0 p-3">
        <FaButton
          faIcon={showCards ? <FaEyeSlash /> : <FaBars />}
          onClick={() => setShowCards(!showCards)}
        />
      </div>
    </div>
  );
};
