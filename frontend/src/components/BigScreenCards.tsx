import React from "react";
import LocationCard from "./LocationCard";
import BoundsCard from "./BoundsCard";
import TimeCard from "./TimeCard";
import BackendStatusCard from "./BackendStatusCard.tsx";

export const BigScreenCards = (props) => {
  return (
    <div className="absolute top-0 left-0 p-3 md:w-96 w-full z-10000">
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
  );
};
