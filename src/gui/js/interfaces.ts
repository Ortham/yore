export interface Coordinates {
  latitude: number;
  longitude: number;
}

export interface Photo {
  path: string;
  location?: Location; // eslint-disable-line no-restricted-globals
  error?: string;
  height?: number;
  width?: number;
  loaded?: boolean;
}

interface Location {
  Existing?: Coordinates;
  Suggested?: [Coordinates, LocationAccuracy];
}

export interface LocationAccuracy {
  meters: number;
  seconds: number;
}

export interface LocationResponse {
  location?: Location; // eslint-disable-line no-restricted-globals
  error?: string;
}
