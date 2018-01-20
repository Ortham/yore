import React from 'react';
import FaLocationArrow from 'react-icons/lib/fa/location-arrow';
import FaMapMarker from 'react-icons/lib/fa/map-marker';
import FaExclamationCircle from 'react-icons/lib/fa/exclamation-circle';

export function locationDescription(photo) {
  if (photo.location && photo.location.Suggested) {
    const accuracy = photo.location.Suggested[1];
    return `Suggested location: accuracy is ${accuracy.meters} meters and ${
      accuracy.seconds
    } seconds`;
  } else if (photo.location) {
    return 'Existing location';
  } else if (photo.error) {
    return photo.error;
  }
  return 'No location';
}

export function hasSuggestion(photo) {
  return !!(photo.location && photo.location.Suggested);
}

export function googleMapsCoordinates(photo) {
  if (photo.location) {
    let coordinates;
    if (photo.location.Existing) {
      coordinates = photo.location.Existing;
    } else {
      ({ location: { Suggested: [coordinates] } } = photo);
      coordinates = photo.location.Suggested[0]; // eslint-disable-line prefer-destructuring
    }

    return {
      lat: coordinates.latitude,
      lng: coordinates.longitude
    };
  }
  return {
    lat: 0,
    lng: 0
  };
}

export function chooseIcon(photo) {
  let icon;
  const style = {
    position: 'relative',
    top: '-2px'
  };
  if (photo.error) {
    icon = <FaExclamationCircle style={style} />;
  } else if (photo.location) {
    if (photo.location.Existing) {
      icon = <FaMapMarker style={style} />;
    } else if (photo.location.Suggested) {
      style.left = '-1px';
      icon = <FaLocationArrow style={style} />;
    } else {
      return null;
    }
  } else {
    return null;
  }

  return <div className="icon">{icon}</div>;
}
