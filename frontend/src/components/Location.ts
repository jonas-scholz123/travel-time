class Location {
  name: string;
  coords: [number, number];

  constructor(name: string, coords: [number, number]) {
    this.name = name;
    this.coords = coords;
  }

  static fromString(locationString: string): Location {
    const [name, latStr, lngStr] = locationString.split(",");
    const coords: [number, number] = [parseFloat(latStr), parseFloat(lngStr)];
    return new Location(name, coords);
  }

  toString(): string {
    return `${this.name},${this.coords[0]},${this.coords[1]}`;
  }
}

export default Location;
