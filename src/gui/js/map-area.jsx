import React from 'react';
import PropTypes from 'prop-types';
import GoogleMapReact from 'google-map-react';
import FaMapMarker from 'react-icons/lib/fa/map-marker';
import { googleMapsCoordinates } from './photo';

function MapMarker(props) {
  return (
    <FaMapMarker
      lat={props.lat}
      lng={props.lng}
      style={{
        height: '24px',
        width: '24px',
        color: 'crimson',
        position: 'relative',
        top: '-24px',
        left: '-12px'
      }}
    />
  );
}

MapMarker.propTypes = {
  lat: PropTypes.number.isRequired,
  lng: PropTypes.number.isRequired
};

export default function MapArea(props) {
  if (props.photo && props.photo.location) {
    const coord = googleMapsCoordinates(props.photo);
    return (
      <GoogleMapReact center={coord} zoom={5}>
        <MapMarker lat={coord.lat} lng={coord.lng} />
      </GoogleMapReact>
    );
  }
  return <GoogleMapReact center={[0, 0]} zoom={5} />;
}

MapArea.propTypes = {
  photo: PropTypes.shape({
    location: PropTypes.object
  })
};

MapArea.defaultProps = {
  photo: undefined
};
