# London Travel Time :bus:
You can find this webapp [Here](londontraveltime.herokuapp.com).
Because I'm using Heroku's free tier it needs a couple of seconds to open if it hasn't been opened in a while, so be patient.

The app allows you to input any number of locations in London via postcode, and it colours a map of london by how long it takes to get there via public transport from all locations.

This is useful for when you want to meet someone and don't know where the middle is in terms of travel time, or when you move to a new postcode and want to check out how well connected it is.

## How it works
I use the open [TFL api](https://api.tfl.gov.uk/) and static [national rail](https://opendata.nationalrail.co.uk/) data to determine how long it takes to move between neighbouring stations at any time of day (weekdays only for now - sorry). From there, I build a graph of all stop points in London, with weights on edges representing the time needed to travel between them.

I then compute a modified version of Dijkstra's algorithm that depends on time (because you have to wait for the train) to determine how long it takes to get to every node in the graph.

## Tech
I built the backend in Rust and the frontend in React. The frontend heavily relies on the leafletjs and react-leaflet packages as well as OpenStreetMap for drawing the map.

To not have to rely on the TFL api, I copied the data into my own MongoDB instance.

## To do
This is V1 and it's pretty useful already, but it can be improved in lots of ways.

Backend:

- [ ] Fix the now broken TFL api (they introduced breaking changes).
- [ ] Add data for the Elizabeth line
- [ ] Connect to national rail's push queue instead of using their static CSV.
- [ ] Get TFL data for each time of day instead of using a random weekday as reference for all days.
- [ ] Add multiple edges between stop points to represent different lines. (There might exist multiple trains from A -> B, but right now they are treated equally, with the assumption that there is no transfer time between them. This leads to faster than reality travel time estimates in some cases.)
- [ ] Relies on previous point: Add transfer times between trains.

Frontend:

- [ ] Add other location input methods except postcodes. Let people click on the map too.
- [ ] Optimise the rendering speed. Maybe by pre-rendering in the backend and sending an SVG/Canvas?
- [x] Make URLs shareable.