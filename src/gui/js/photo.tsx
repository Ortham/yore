import * as React from 'react';
import {FaExclamationCircle, FaLocationArrow, FaMapMarker} from 'react-icons/lib/fa';
import {Photo} from './interfaces';

export function locationDescription(photo: Photo) {
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

export function hasSuggestion(photo: Photo) {
  return !!(photo.location && photo.location.Suggested);
}

export function googleMapsCoordinates(photo: Photo) {
  if (photo.location) {
    let coordinates;
    if (photo.location.Existing) {
      coordinates = photo.location.Existing;
    } else {
      ({ location: { Suggested: [coordinates] } } = photo);
      coordinates = photo.location.Suggested[0];
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

export function chooseIcon(photo: Photo) {
  let icon;
  const style: React.CSSProperties = {
    left: undefined as string,
    position: 'relative',
    top: '-2px',
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
