import * as React from 'react';
import { FiAlertCircle, FiNavigation, FiMapPin } from 'react-icons/fi';
import { LocationResponse, Photo } from './interfaces';

export function locationDescription(photo: Photo) {
  if (photo.location && photo.location.Suggested) {
    const accuracy = photo.location.Suggested[1];
    return `Suggested location: accuracy is ${accuracy.meters} meters and ${
      accuracy.seconds
    } seconds`;
  }
  if (photo.location) {
    return 'Existing location';
  }
  if (photo.error) {
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
    top: '-2px'
  };
  if (photo.error) {
    icon = <FiAlertCircle style={style} />;
  } else if (photo.location) {
    if (photo.location.Existing) {
      icon = <FiMapPin style={style} />;
    } else if (photo.location.Suggested) {
      style.left = '-1px';
      style.top = '-1px';
      icon = <FiNavigation style={style} />;
    } else {
      return null;
    }
  } else {
    return null;
  }

  return <div className="icon">{icon}</div>;
}

export function updatePhotoLocations(
  photos: Photo[],
  locations: LocationResponse[]
) {
  if (photos.length !== locations.length) {
    throw new Error('photos and locations array lengths are not equal');
  }

  const updatedPhotos = photos.slice();
  for (let i = 0; i < updatedPhotos.length; i += 1) {
    // Don't mutate the existing object.
    updatedPhotos[i] = Object.assign({}, updatedPhotos[i]);

    // Assign these here instead of using Object.assign to set any undefined
    // values.
    updatedPhotos[i].location = locations[i].location;
    updatedPhotos[i].error = locations[i].error;
    updatedPhotos[i].loaded = true;
  }

  return updatedPhotos;
}
