import GoogleMapReact from 'google-map-react';
import * as React from 'react';
import { FaMapMarker } from 'react-icons/lib/fa';

import { Photo } from './interfaces';
import { googleMapsCoordinates } from './photo';

interface MapMarkerProps {
  lat: number;
  lng: number;
}

function MapMarker(props: MapMarkerProps) {
  return (
    <FaMapMarker
      {...props}
      style={{
        color: 'crimson',
        height: '24px',
        left: '-12px',
        position: 'relative',
        top: '-24px',
        width: '24px'
      }}
    />
  );
}

export interface MapAreaProps {
  photo?: Photo;
}

export function MapArea(props: MapAreaProps) {
  let coord = { lat: 0, lng: 0};
  let marker;
  if (props.photo && props.photo.location) {
    coord = googleMapsCoordinates(props.photo);
    marker = <MapMarker {...coord} />;
  }

  const style: React.CSSProperties = {
    height: '50%',
    position: 'relative',
    width: '100%'
  };

  return (
    <div style={style}>
      <GoogleMapReact center={coord} zoom={5}>
        {marker}
      </GoogleMapReact>
    </div>
  );
}
